use dioxus::desktop::{Config, WindowBuilder};
use dioxus::prelude::*;
use std::path::Path;
const CSS: Asset = asset!("/assets/main.css");
use include_dir::{include_dir, Dir};

mod tabs;

static SHADERS: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets/shaders");

fn main() {
    std::fs::write("debug.log", "App started\n").ok();
    let base = std::env::var("XDG_CONFIG_HOME")
        .ok()
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| {
            let home = std::env::var("HOME").expect("HOME is not set!");
            std::path::PathBuf::from(home).join(".config")
        });
    let path = base.join("haremal-ctrl");
    if let Err(e) = std::fs::create_dir_all(&path) {
        eprintln!("Failed to create config directory: {}", e);
        return;
    }
    let shaders_path = path.join("shaders");
    if !shaders_path.exists() {
        let _ = std::fs::create_dir_all(&shaders_path);
        for entry in SHADERS.entries() {
            extract_all(entry, &shaders_path);
        }
    }

    let mimeapps_path = base.join("mimeapps.list");
    if !Path::new(&mimeapps_path).exists() {
        let mut mimeapps = config_read(Some("/usr/share/applications/mimeinfo.cache"), "");
        mimeapps.retain(|item| {
            let m = &item.to_string();
            m.starts_with("inode/directory")
                || m.starts_with("text/")
                || m.starts_with("image/")
                || m.starts_with("video/")
                || m.starts_with("audio/")
                || m.starts_with("application/pdf")
                || m.starts_with("x-scheme-handler/https")
                || m.starts_with("x-scheme-handler/http")
                || m.starts_with("x-scheme-handler/mailto")
        });
        for mimeapp in &mimeapps {
            config_write(Some("mimeapps.list"), "", mimeapp);
        }
        config_write(Some("mimeapps.list"), "", "[Default Applications]");
    }

    let data = std::env::var("XDG_DATA_HOME").expect("XDG_DATA_HOME is not set!");
    let data_dir = std::path::PathBuf::from(&data).join("haremal-ctrl");
    let window = WindowBuilder::new()
        .with_title("HaremalCTRL")
        .with_decorations(false)
        .with_transparent(true)
        .with_resizable(true);

    let cfg = Config::new()
        .with_window(window)
        .with_data_directory(data_dir)
        .with_disable_context_menu(true)
        .with_disable_drag_drop_handler(true);

    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn App() -> Element {
    let mut tab = use_signal(|| 0);
    let major_changes = use_context_provider(|| Signal::new(false));
    rsx! {
        document::Link { rel: "stylesheet", href: CSS }
        main {
            display: "flex",
            div {
                class: "menu",
                h2 { font_size: "200%", font_weight: "bold", padding_left: "20px", "HaremalCTRL Settings" },
                button { onclick: move |_| tab.set(0), class: "tab_button", background_color: if tab() == 0 { "#3f4146" },  "Update" }
                button { onclick: move |_| tab.set(1), class: "tab_button", background_color: if tab() == 1 { "#3f4146" },  "Region" }
                button { onclick: move |_| tab.set(2), class: "tab_button", background_color: if tab() == 2 { "#3f4146" },  "Applications" }
                button { onclick: move |_| tab.set(3), class: "tab_button", background_color: if tab() == 3 { "#3f4146" },  "Devices" }
                button { onclick: move |_| tab.set(4), class: "tab_button", background_color: if tab() == 4 { "#3f4146" },  "Appearance" }
                button { onclick: move |_| tab.set(5), class: "tab_button", background_color: if tab() == 5 { "#3f4146" },  "Desktop" }
                h3 { width: "260px", visibility: if !major_changes() { "hidden" }, color: "orange", opacity: "50%", text_align: "center", font_weight: "200", position: "absolute", bottom: "60px", left: "20px", "Some changes might require a restart to work properly"}
                button { width: "260px", visibility: if !major_changes() { "hidden" }, onclick: move |_| { std::process::Command::new("systemctl").arg("reboot").status().ok(); }, position: "absolute", bottom: "20px", left: "20px", color: "orange", opacity: "50%", "REBOOT NOW" }
            }
            match tab() {
                1 => rsx! { tabs::region::Region {} },
                2 => rsx! { tabs::applications::Applications {} },
                3 => rsx! { tabs::devices::Devices {} },
                4 => rsx! { tabs::appearance::Appearance {} },
                5 => rsx! { tabs::desktop::Desktop {} },
                _ => rsx! { tabs::update::Update {} },
            }
        }
    }
}

fn extract_all(entry: &include_dir::DirEntry, base_path: &std::path::Path) {
    match entry {
        include_dir::DirEntry::Dir(d) => {
            for e in d.entries() {
                extract_all(e, base_path);
            }
        }
        include_dir::DirEntry::File(f) => {
            let dest = base_path.join(f.path());
            if let Some(parent) = dest.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            let _ = std::fs::write(dest, f.contents());
        }
    }
}

pub fn config_write(file: Option<&str>, below: &str, add: &str) {
    let filename = file.unwrap_or("haremal-ctrl/config.toml");
    let path = if !filename.starts_with("/") {
        let base = std::env::var("XDG_CONFIG_HOME").expect("XDG_CONFIG_HOME is not set!");
        std::path::PathBuf::from(base).join(filename)
    } else {
        std::path::PathBuf::from(filename)
    };
    let content = std::fs::read_to_string(&path).unwrap_or_default();
    let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

    if lines.is_empty() {
        lines.push(String::from(add));
    } else {
        lines = lines
            .into_iter()
            .flat_map(|line| {
                if line.trim().contains(below) {
                    vec![line, add.to_string()]
                } else {
                    vec![line]
                }
            })
            .collect();
    }

    if let Err(e) = std::fs::write(&path, lines.join("\n")) {
        eprintln!("Error saving config: {}", e);
    }
}

pub fn config_read(file: Option<&str>, read: &str) -> Vec<String> {
    let filename = file.unwrap_or("haremal-ctrl/config.toml");
    let path = if !filename.starts_with("/") {
        let base = std::env::var("XDG_CONFIG_HOME").expect("XDG_CONFIG_HOME is not set!");
        std::path::PathBuf::from(base).join(filename)
    } else {
        std::path::PathBuf::from(filename)
    };
    let content = std::fs::read_to_string(path).ok().unwrap_or_default();
    let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    let list: Vec<_> = lines
        .into_iter()
        .filter(|line| line.trim().contains(read))
        .collect();
    if list.is_empty() {
        return vec![String::from("")];
    }
    list
}

pub fn config_update(file: Option<&str>, replace: &str, replace_with: &str) {
    let filename = file.unwrap_or("haremal-ctrl/config.toml");
    let path = if !filename.starts_with("/") {
        let base = std::env::var("XDG_CONFIG_HOME").expect("XDG_CONFIG_HOME is not set!");
        std::path::PathBuf::from(base).join(filename)
    } else {
        std::path::PathBuf::from(filename)
    };
    let content = std::fs::read_to_string(&path).unwrap_or_default();
    let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

    for line in lines.iter_mut() {
        if line.trim().contains(replace) {
            *line = String::from(replace_with);
        }
    }
    if let Err(e) = std::fs::write(&path, lines.join("\n")) {
        eprintln!("Error saving config: {}", e);
    }
}

pub fn config_remove(file: Option<&str>, remove: &str) {
    let filename = file.unwrap_or("haremal-ctrl/config.toml");
    let base = std::env::var("XDG_CONFIG_HOME").expect("XDG_CONFIG_HOME is not set!");
    let path = std::path::PathBuf::from(base).join(filename);
    let content = std::fs::read_to_string(&path).unwrap_or_default();
    let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    lines.retain(|line| !line.trim().contains(remove));
    if let Err(e) = std::fs::write(&path, lines.join("\n")) {
        eprintln!("Error saving config: {}", e);
    }
}
