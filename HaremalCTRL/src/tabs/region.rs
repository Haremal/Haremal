use crate::{config_read, config_update, config_write};
use dioxus::prelude::*;

#[component]
pub fn Region() -> Element {
    let mut current_timezone = use_signal(|| {
        let conf = config_read(None, "timezone");
        if conf.is_empty() {
            return String::from("America/New_York");
        }
        conf[0]
            .split('"')
            .nth(1)
            .unwrap_or("America/New_York")
            .to_string()
            .clone()
    });
    let timezones = get_timezones();

    let mut current_language = use_signal(|| {
        let conf = config_read(None, "language");
        if conf.is_empty() {
            return String::from("en_US.UTF-8 UTF-8");
        }
        conf[0]
            .split('"')
            .nth(1)
            .unwrap_or("English (US)")
            .to_string()
            .clone()
    });
    let languages = get_languages();
    let mut languages_disabled = use_signal(|| false);

    let mut current_layout = use_signal(|| {
        config_read(Some("niri/config.kdl"), "layout \"")[0]
            .split('"')
            .nth(1)
            .unwrap_or("us")
            .to_string()
            .clone()
    });
    let layouts = get_layouts();

    rsx! {
        div {
            class: "tab",
            h1 { "Region" }
            div {
                div {
                    height: "70px", align_items: "center",
                    p { float: "left", margin: "20px", "Timezone" },
                    select {
                        onchange: move |evt: FormEvent| {
                            current_timezone.set(evt.value());
                            std::process::Command::new("sudo")
                                .args(["timedatectl", "set-timezone", &evt.value()])
                                .spawn()
                                .expect("Failed to set timezone").wait().unwrap();
                            if config_read(None, "timezone").is_empty() {
                                config_write(None, "", &format!("timezone \"{}\"", evt.value()));
                            } else {
                                config_update(None, "timezone", &format!("timezone \"{}\"", evt.value()));
                            }
                        },
                        float: "right", margin: "20px", width: "150px",

                        for timezone in timezones {
                            option { value: "{timezone}", selected: timezone == current_timezone(), "{timezone}" }
                        }
                    }
                }
                div {
                    height: "70px", align_items: "center",
                    p { float: "left", margin: "20px", "System Language" },
                    select {
                        disabled: languages_disabled(),
                        onchange: move |evt: FormEvent| {
                            languages_disabled.set(true);
                            let val = evt.value();
                            let lang_id = val.split_whitespace().next().unwrap_or("en_US.UTF-8").to_string();
                            let var = val.clone();
                            spawn(async move {
                                let cmd = format!("echo -e 'en_US.UTF-8 UTF-8\n{var}' | sudo tee /etc/locale.gen");
                                let _ = tokio::process::Command::new("sh").args(["-c", &cmd]).status().await;
                                let _ = tokio::process::Command::new("sudo").arg("locale-gen").status().await;
                                let _ = tokio::process::Command::new("sudo").args(["localectl", "set-locale", &format!("LANG={lang_id}")]).status().await;
                                languages_disabled.set(false);
                                use_context::<Signal<bool>>().set(true);
                            });

                            current_language.set(evt.value());
                            if config_read(None, "language").is_empty() {
                                config_write(None, "", &format!("language \"{}\"", evt.value()));
                            } else {
                                config_update(None, "language", &format!("language \"{}\"", evt.value()));
                            }
                        },
                        float: "right", margin: "20px", width: "150px",

                        {
                            languages.iter().filter_map(|language| {
                                if language.contains('@') || !language.contains(".UTF-8") || language == "C.UTF-8" { return None; }
                                let mut parts = language.split('_');
                                let lang_id = parts.next().unwrap_or("");
                                let country_id = parts.next().unwrap_or("").split(".").next().unwrap_or("");
                                let lang_name = if lang_id.len() == 2 || lang_id.len() == 3 {
                                    locale_codes::language::lookup(lang_id)
                                        .map(|l| l.reference_name.to_string())
                                        .unwrap_or_else(|| lang_id.to_uppercase())
                                } else {
                                    return None;
                                };
                                let country_name = if !country_id.is_empty() {
                                    format!(" ({})", country_id)
                                } else {
                                    "".to_string()
                                };
                                let pretty_name = format!("{}{}", lang_name, country_name);
                                Some(rsx! {
                                    option {
                                        value: "{language}",
                                        selected: *language == current_language(),
                                        "{pretty_name}"
                                    }
                                })
                            })
                        }
                    }
                }
                div {
                    height: "70px", align_items: "center",
                    p { float: "left", margin: "20px", "Keyboard Layout" },
                    select {
                        onchange: move |evt: FormEvent| {
                            current_layout.set(evt.value());
                            config_update(Some("niri/config.kdl"), "layout \"", &format!("             layout \"{}\"", evt.value()));
                        },
                        float: "right", margin: "20px", width: "150px",

                        for layout in layouts {
                            option { value: "{layout}", selected: layout  == current_layout(), "{layout}" }
                        }
                    }
                }
            }
        }
    }
}

fn get_timezones() -> Vec<String> {
    let output = std::process::Command::new("timedatectl")
        .arg("list-timezones")
        .output()
        .expect("Failed to fetch timezones");

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|s| s.to_string())
        .collect()
}

fn get_languages() -> Vec<String> {
    let output = std::process::Command::new("cat")
        .arg("/usr/share/i18n/SUPPORTED")
        .output()
        .expect("Failed to fetch languages");

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|s| s.to_string())
        .collect()
}

fn get_layouts() -> Vec<String> {
    let output = std::process::Command::new("localectl")
        .arg("list-x11-keymap-layouts")
        .output()
        .expect("Failed to fetch layouts");

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|s| s.to_string())
        .collect()
}
