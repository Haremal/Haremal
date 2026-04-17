use std::process::Termination;

use crate::{config_read, config_remove, config_update, config_write};
use dioxus::prelude::*;

#[component]
pub fn Appearance() -> Element {
    let mut tables = [0; 4].map(|_| use_signal(|| false));

    let mut db = fontdb::Database::new();
    db.load_system_fonts();
    let mut fonts: Vec<String> = db.faces().map(|f| f.families[0].0.clone()).collect();
    fonts.sort();
    fonts.dedup();
    let current_font = use_signal(|| {
        let f = config_read(Some("fontconfig/fonts.conf"), "edit")[0].clone();
        let ff = f
            .split("<string>")
            .nth(1)
            .and_then(|s| s.split("</string>").next())
            .map(|s| s.to_string())
            .unwrap_or_default();
        ff
    });

    let mut icons = Vec::new();
    let mut cursors = Vec::new();
    if let Ok(entries) = std::fs::read_dir("/usr/share/icons") {
        for entry in entries.flatten() {
            let name = entry.file_name().into_string().unwrap_or_default();
            if entry.path().join("cursors").exists() {
                cursors.push(name);
            } else {
                icons.push(name);
            }
        }
    }
    let mut current_icon = use_signal(|| {
        let output = std::process::Command::new("gsettings")
            .args(["get", "org.gnome.desktop.interface", "icon-theme"])
            .output()
            .unwrap();
        String::from_utf8_lossy(&output.stdout)
            .trim()
            .replace('\'', "")
    });

    let mut current_cursor = use_signal(|| {
        let output = std::process::Command::new("gsettings")
            .args(["get", "org.gnome.desktop.interface", "cursor-theme"])
            .output()
            .unwrap();
        String::from_utf8_lossy(&output.stdout)
            .trim()
            .replace('\'', "")
    });

    let mut cursor_size = use_signal(|| {
        let output = std::process::Command::new("gsettings")
            .args(["get", "org.gnome.desktop.interface", "cursor-size"])
            .output()
            .unwrap();
        let s = String::from_utf8_lossy(&output.stdout);
        s.chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<u32>()
            .unwrap_or(24)
    });

    let mut cursor_trail = use_signal(|| {
        let conf = config_read(Some("rio/config.toml"), "trail-cursor");
        conf[0].split(" ").nth(2).unwrap().parse().unwrap_or(false)
    });

    let mut gaps = use_signal(|| {
        let conf = config_read(Some("niri/config.kdl"), "gaps")[0].clone();
        conf.trim()
            .split(" ")
            .nth(1)
            .unwrap_or_default()
            .parse::<u32>()
            .unwrap_or(16)
    });

    let mut center_focused_column =
        use_signal(|| !config_read(Some("niri/config.kdl"), "center-focused-column")[0].is_empty());
    let mut center_single_column = use_signal(|| {
        !config_read(Some("niri/config.kdl"), "always-center-single-column")[0].is_empty()
    });
    let mut empty_workspace_above = use_signal(|| {
        !config_read(Some("niri/config.kdl"), "empty-workspace-above-first")[0].is_empty()
    });
    let mut shadow =
        use_signal(|| !config_read(Some("niri/config.kdl"), "on // shadow")[0].is_empty());

    let mut corner_radius = use_signal(|| {
        let conf = config_read(Some("niri/config.kdl"), "geometry-corner-radius")[0].clone();
        conf.trim()
            .split(" ")
            .nth(1)
            .unwrap_or_default()
            .parse::<u32>()
            .unwrap_or(12)
    });
    let mut opacity = use_signal(|| {
        let conf = config_read(Some("niri/config.kdl"), "// opacity active")[0].clone();
        conf.trim()
            .split(" ")
            .nth(1)
            .unwrap_or_default()
            .parse::<f32>()
            .unwrap_or(1.0)
    });
    let mut opacity_in = use_signal(|| {
        let conf = config_read(Some("niri/config.kdl"), "// opacity inactive")[0].clone();
        conf.trim()
            .split(" ")
            .nth(1)
            .unwrap_or_default()
            .parse::<f32>()
            .unwrap_or(9.0)
    });
    let mut duration = use_signal(|| {
        let conf = config_read(Some("niri/config.kdl"), "duration-ms")[0].clone();
        conf.trim()
            .split(" ")
            .nth(1)
            .unwrap_or_default()
            .parse::<u32>()
            .unwrap_or(150)
    });

    let base = std::env::var("XDG_CONFIG_HOME")
        .ok()
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| {
            let home = std::env::var("HOME").expect("HOME is not set!");
            std::path::PathBuf::from(home).join(".config")
        });
    let path = base.join("haremal-ctrl/shaders");
    let mut shaders = vec![String::from("default")];
    if let Ok(entries) = std::fs::read_dir(&path) {
        for entry in entries.flatten() {
            let name = entry.file_name().into_string().unwrap_or_default();
            shaders.push(name);
        }
    }
    let mut current_shader = use_signal(|| {
        let conf = config_read(Some("niri/config.kdl"), "custom-shader")[0].clone();
        conf.trim()
            .split(" ")
            .nth(4)
            .unwrap_or("default")
            .to_string()
    });
    rsx! {
        div {
            class: "tab",
            h1 { "Appearance" }
            div {
                div {
                    button {
                        class: "tab_button",
                        onclick: move |_| {
                            let state = !tables[0]();
                            tables.iter_mut().for_each(|t| t.set(false));
                            tables[0].set(state);
                        },
                        "System"
                    }
                    if *tables[0].read() { div {
                        max_height: "400px",
                        overflow_y: "auto",
                        form {
                            div {
                                display: "flex", margin_left: "20px", margin_right: "20px",
                                p { flex: "85%", margin: "20px", "Font" }
                                select {
                                    margin: "20px", flex: "15%",
                                    onchange: move |evt| {
                                        let value = evt.value();
                                        let a = format!("<edit name=\"family\" mode=\"assign\" binding=\"strong\"><string>{value}</string></edit>");
                                        config_update(Some("fontconfig/fonts.conf"), "edit", &a);
                                        spawn(async move {
                                            let _ = tokio::process::Command::new("fc-cache")
                                                .arg("-f").status().await;
                                            use_context::<Signal<bool>>().set(true);
                                        });
                                    },
                                    for face in &fonts {
                                        option {
                                            selected: *face == current_font(),
                                            value: "{face}",
                                            "{face}"
                                         }
                                    }
                                }
                            }
                            div {
                                display: "flex", margin_left: "20px", margin_right: "20px",
                                p { flex: "85%", margin: "20px", "Icons" }
                                select {
                                    margin: "20px", flex: "15%",
                                    onchange: move |evt| {
                                        let value = evt.value();
                                        spawn(async move {
                                            let _ = tokio::process::Command::new("gsettings")
                                                .args(["set", "org.gnome.desktop.interface", "icon-theme", &value])
                                                .status()
                                                .await;
                                            current_icon.set(value);

                                        });
                                    },
                                    for icon in icons {{
                                        let icon_display = format!("{}{}", &icon[..1].to_uppercase(), &icon[1..]).replace("-", " ");
                                        rsx! {
                                            option {
                                                selected: *icon == current_icon(),
                                                value: "{icon}",
                                                "{icon_display}"
                                            }
                                        }
                                    }}
                                }
                            }
                            div {
                                display: "flex", margin_left: "20px", margin_right: "20px",
                                p { flex: "60%", margin: "20px", "Cursor" }
                                input {
                                    margin: "20px", flex: "30%", r#type: "number",
                                    oninput: move |evt| {
                                        evt.prevent_default();
                                        let value = evt.value();
                                        let home = std::env::var("HOME").ok().unwrap();
                                        config_update(Some(&format!("{}/.bash_profile", home)), "XCURSOR_SIZE", &format!("export XCURSOR_SIZE={}", &value));
                                        config_update(Some("niri/config.kdl"), "xcursor-size", &format!("    xcursor-size {}", &value));
                                        spawn(async move {
                                            let _ = tokio::process::Command::new("gsettings")
                                                .args(["set", "org.gnome.desktop.interface", "cursor-size", &value])
                                                .status()
                                                .await;
                                            cursor_size.set(value.parse().unwrap_or_default());
                                            use_context::<Signal<bool>>().set(true);
                                        });
                                    },
                                    value: cursor_size()
                                }
                                select {
                                    margin: "20px", flex: "10%",
                                    onchange: move |evt| {
                                        let value = evt.value();
                                        let home = std::env::var("HOME").ok().unwrap();
                                        config_update(Some(&format!("{}/.bash_profile", home)), "XCURSOR_THEME", &format!("export XCURSOR_THEME=\"{}\"", &value));
                                        config_update(Some("niri/config.kdl"), "xcursor-theme", &format!("    xcursor-theme \"{}\"", &value));
                                        spawn(async move {
                                            let _ = tokio::process::Command::new("gsettings")
                                                .args(["set", "org.gnome.desktop.interface", "cursor-theme", &value])
                                                .status()
                                                .await;
                                            current_cursor.set(value);
                                            use_context::<Signal<bool>>().set(true);
                                        });
                                    },
                                    for cursor in cursors {{
                                        let cursor_display = cursor.replace("-", " ");
                                        rsx! {
                                            option {
                                                selected: *cursor == current_cursor(),
                                                value: "{cursor}",
                                                "{cursor_display}"
                                            }
                                        }
                                    }}

                                }
                            }
                            div {
                                display: "flex", margin_left: "20px", margin_right: "20px",
                                background_color: "#303000",
                                p { flex: "60%", margin: "20px", "Lockscreen" }
                                div {
                                    flex: "10%",
                                    label {
                                        flex: "10%", class: "switch", margin: "20px",
                                        input {
                                            onclick: move |_| {
                                            },
                                            type: "checkbox"
                                        },
                                        span { class: "slider round"}
                                    }
                                }
                                input {
                                    margin: "25px", flex: "15%", r#type: "file", padding: 0,
                                    accept: "image/png, image/jpeg, .png, .jpg, .jpeg",
                                    oninput: move |evt| {
                                        evt.prevent_default();
                                        let files = evt.files();
                                        for file in files {
                                             let path = file.path();
                                             let dest = base.join("haremal-ctrl/icon.png");
                                             let _ = std::fs::copy(&path, &dest);
                                        }
                                    }
                                }
                                select {
                                    margin: "20px", flex: "15%",
                                    onchange: move |evt| {
                                        let _value = evt.value();
                                    },
                                    // options: classic, minimal, osu!
                                }
                            }
                        }
                    }}
                }
                div {
                    button {
                        class: "tab_button",
                        onclick: move |_| {
                            let state = !tables[1]();
                            tables.iter_mut().for_each(|t| t.set(false));
                            tables[1].set(state);
                        },
                        "Layout"
                    }
                    if *tables[1].read() { div {
                        max_height: "500px", overflow_y: "auto",
                        form {
                            div {
                                display: "flex", margin_left: "20px", margin_right: "20px",
                                p { flex: "85%", margin: "20px", "Gaps" }
                                input {
                                    margin: "20px", flex: "15%", r#type: "number",
                                    oninput: move |evt| {
                                        evt.prevent_default();
                                        let mut value = evt.value();
                                        if let Ok(parsed) = value.parse::<u32>() {
                                            gaps.set(parsed);
                                        } else {
                                            value = String::from("0");
                                            gaps.set(0);
                                        }
                                        config_update(Some("niri/config.kdl"), "gaps", &format!("    gaps {}", &value));
                                    },
                                    value: gaps()
                                }
                            }
                            div {
                                display: "flex", margin_left: "20px", margin_right: "20px",
                                background_color: "#303000",
                                p { flex: "90%", margin: "20px", "Focus Ring / Borders" }
                                div {
                                    flex: "10%",
                                    label {
                                        flex: "10%", class: "switch", margin: "20px",
                                        input {
                                            // checked: center_focused_column(),
                                            onclick: move |_| {
                                                // center_focused_column.set(!center_focused_column());
                                                // if center_focused_column() {
                                                //     config_write(Some("niri/config.kdl"), "layout {", "    center-focused-column \"always\"");
                                                // } else {
                                                //     config_remove(Some("niri/config.kdl"), "center-focused-column");
                                                // }
                                            },
                                            type: "checkbox"
                                        },
                                        span { class: "slider round"}
                                    }
                                }
                            }
                            div {
                                display: "flex", margin_left: "20px", margin_right: "20px",
                                p { flex: "90%", margin: "20px", "Center Focused Column" }
                                div {
                                    flex: "10%",
                                    label {
                                        flex: "10%", class: "switch", margin: "20px",
                                        input {
                                            checked: center_focused_column(),
                                            onclick: move |_| {
                                                center_focused_column.set(!center_focused_column());
                                                if center_focused_column() {
                                                    config_write(Some("niri/config.kdl"), "layout {", "    center-focused-column \"always\"");
                                                } else {
                                                    config_remove(Some("niri/config.kdl"), "center-focused-column");
                                                }
                                            },
                                            type: "checkbox"
                                        },
                                        span { class: "slider round"}
                                    }
                                }
                            }
                            div {
                                display: "flex", margin_left: "20px", margin_right: "20px",
                                p { flex: "90%", margin: "20px", "Center Single Column" }
                                div {
                                    flex: "10%",
                                    label {
                                        flex: "10%", class: "switch", margin: "20px",
                                        input {
                                            checked: center_single_column(),
                                            onclick: move |_| {
                                                center_single_column.set(!center_single_column());
                                                if center_single_column() {
                                                    config_write(Some("niri/config.kdl"), "layout {", "    always-center-single-column");
                                                } else {
                                                    config_remove(Some("niri/config.kdl"), "always-center-single-column");
                                                }
                                            },
                                            type: "checkbox"
                                        },
                                        span { class: "slider round"}
                                    }
                                }
                            }
                            div {
                                display: "flex", margin_left: "20px", margin_right: "20px",
                                p { flex: "90%", margin: "20px", "Empty Workspace Above" }
                                div {
                                    flex: "10%",
                                    label {
                                        flex: "10%", class: "switch", margin: "20px",
                                        input {
                                            checked: empty_workspace_above(),
                                            onclick: move |_| {
                                                empty_workspace_above.set(!empty_workspace_above());
                                                if empty_workspace_above() {
                                                    config_write(Some("niri/config.kdl"), "layout {", "    empty-workspace-above-first");
                                                } else {
                                                    config_remove(Some("niri/config.kdl"), "empty-workspace-above-first");
                                                }
                                            },
                                            type: "checkbox"
                                        },
                                        span { class: "slider round"}
                                    }
                                }
                            }
                            div {
                                display: "flex", margin_left: "20px", margin_right: "20px",
                                p { flex: "90%", margin: "20px", "Shadow" }
                                div {
                                    flex: "10%",
                                    label {
                                        flex: "10%", class: "switch", margin: "20px",
                                        input {
                                            checked: shadow(),
                                            onclick: move |_| {
                                                shadow.set(!shadow());
                                                let s = if shadow() { "        on // shadow" } else { "        off // shadow" };
                                                config_update(Some("niri/config.kdl"), "// shadow", s);
                                            },
                                            type: "checkbox"
                                        },
                                        span { class: "slider round"}
                                    }
                                }
                            }
                        }
                    }}
                }
                div {
                    button {
                        class: "tab_button",
                        onclick: move |_| {
                            let state = !tables[2]();
                            tables.iter_mut().for_each(|t| t.set(false));
                            tables[2].set(state);
                        },
                        "Windows"
                    }
                    if *tables[2].read() { div {
                        max_height: "300px", overflow_y: "auto",
                        form {
                             div {
                                background_color: "#84141440",
                                display: "flex", margin_left: "20px", margin_right: "20px",
                                p { flex: "90%", margin: "20px", "Blur" }
                                div {
                                    flex: "10%",
                                    title: "Unavaiable for now",
                                    label {
                                        flex: "10%", class: "switch", margin: "20px",
                                        input {
                                            disabled: "true",
                                            type: "checkbox"
                                        },
                                        span { class: "slider round"}
                                    }
                                }
                            }
                            div {
                                display: "flex", margin_left: "20px", margin_right: "20px",
                                p { flex: "90%", margin: "20px", "Corner Radius" }
                                input {
                                    margin: "20px", flex: "15%", r#type: "number",
                                    oninput: move |evt| {
                                        evt.prevent_default();
                                        let mut value = evt.value();
                                        if let Ok(parsed) = value.parse::<u32>() {
                                            corner_radius.set(parsed);
                                        } else {
                                            value = String::from("0");
                                            corner_radius.set(0);
                                        }
                                        config_update(Some("niri/config.kdl"), "geometry-corner-radius", &format!("    geometry-corner-radius {}", &value));
                                    },
                                    value: corner_radius()
                                }
                            }
                            div {
                                display: "flex", margin_left: "20px", margin_right: "20px",
                                p { flex: "90%", margin: "20px", "Opacity" }
                                input {
                                    margin: "20px", flex: "15%", r#type: "number",
                                    min: "20", max: "100",
                                    oninput: move |evt| {
                                        evt.prevent_default();
                                        let mut value = evt.value();
                                        let parsed = (value.parse::<f32>().unwrap_or(100.0).round() / 100.0).clamp(0.2, 1.0);
                                        opacity.set(parsed);
                                        value = format!("{:.2}", parsed);
                                        config_update(Some("niri/config.kdl"), "// opacity active", &format!("    opacity {} // opacity active", &value));
                                    },
                                    value: (opacity() * 100.0).round() as u32
                                }
                                input {
                                    margin: "20px", flex: "15%", r#type: "number",
                                    min: "20", max: "100",
                                    oninput: move |evt| {
                                        evt.prevent_default();
                                        let mut value = evt.value();
                                        let parsed = (value.parse::<f32>().unwrap_or(100.0).round() / 100.0).clamp(0.2, 1.0);
                                        opacity_in.set(parsed);
                                        value = format!("{:.2}", parsed);
                                        config_update(Some("niri/config.kdl"), "// opacity inactive", &format!("    opacity {} // opacity inactive", &value));
                                    },
                                    value: (opacity_in() * 100.0).round() as u32
                                }
                            }
                        }
                    }}
                }
                div {
                    button {
                        class: "tab_button",
                        onclick: move |_| {
                            let state = !tables[3]();
                            tables.iter_mut().for_each(|t| t.set(false));
                            tables[3].set(state);
                        },
                        "Animations"
                    }
                    if *tables[3].read() { div {
                        max_height: "600px", overflow_y: "auto",
                        form {
                            div {
                                display: "flex", margin_left: "20px", margin_right: "20px",
                                background_color: "#303000",
                                p { flex: "85%", margin: "20px", "Hub Style" }
                                select {
                                    margin: "20px", flex: "15%",
                                    onchange: move |evt| {
                                        let value = evt.value();
                                        // spawn(async move {
                                        //     let _ = tokio::process::Command::new("gsettings")
                                        //         .args(["set", "org.gnome.desktop.interface", "icon-theme", &value])
                                        //         .status()
                                        //         .await;
                                        //     current_icon.set(value);
                                        // });
                                    },
                                    // for icon in icons {{
                                    //     let icon_display = icon.replace("-", " ");
                                    //     rsx! {
                                    //         option {
                                    //             selected: *icon == current_icon(),
                                    //             value: "{icon}",
                                    //             "{icon_display}"
                                    //         }
                                    //     }
                                    // }}
                                }
                            }
                            div {
                                display: "flex", margin_left: "20px", margin_right: "20px",
                                p { flex: "50%", margin: "20px", "Transitions" }
                                input {
                                    margin: "20px", flex: "20%", r#type: "number",
                                    oninput: move |evt| {
                                        evt.prevent_default();
                                        let mut value = evt.value();
                                        if let Ok(parsed) = value.parse::<u32>() {
                                            duration.set(parsed);
                                        } else {
                                            value = String::from("0");
                                            duration.set(0);
                                        }
                                        config_update(Some("niri/config.kdl"), "duration-ms", &format!("        duration-ms {}", &value));
                                    },
                                    value: duration()
                                }
                                select {
                                    margin: "20px", flex: "30%",
                                    onchange: move |evt| {
                                        let value = evt.value();
                                        current_shader.set(value.clone());
                                        config_remove(Some("niri/config.kdl"), "// marked_shader");
                                        if value == "default" {
                                            config_update(Some("niri/config.kdl"), "// open", "        // custom-shader r\" // open default");
                                            config_update(Some("niri/config.kdl"), "// close", "        // custom-shader r\" // close default");
                                            config_update(Some("niri/config.kdl"), "// end", "        // \" // end");
                                            return;
                                        } else {
                                            config_update(Some("niri/config.kdl"), "// open", &format!("        custom-shader r\" // open {}", value));
                                            config_update(Some("niri/config.kdl"), "// close", &format!("        custom-shader r\" // close {}", value));
                                            config_update(Some("niri/config.kdl"), "// end", "        \" // end");
                                        }
                                        let shader_path = &path.join(value);
                                        let open_path = shader_path.join("open.glsl");
                                        let close_path = shader_path.join("close.glsl");

                                        let open_content = std::fs::read_to_string(open_path).unwrap_or_default();
                                        let marked_open_content: Vec<String> = open_content.trim_matches('\n')
                                            .lines()
                                            .map(|line| format!("{} // marked_shader", line))
                                            .collect();
                                        let open_block = marked_open_content.join("\n");

                                        let close_content = std::fs::read_to_string(close_path).unwrap_or_default();
                                        let marked_close_content: Vec<String> = close_content.trim_matches('\n')                                                  .lines()
                                            .map(|line| format!("{} // marked_shader", line))
                                            .collect();
                                        let close_block = marked_close_content.join("\n");
                                        config_write(Some("niri/config.kdl"), "// open", &open_block);
                                        config_write(Some("niri/config.kdl"), "// close", &close_block);
                                    },
                                    for shader in shaders {{
                                        let shader_display = format!("{}{}", &shader[..1].to_uppercase(), &shader[1..]);
                                        rsx! {
                                            option {
                                                selected: *shader == current_shader(),
                                                value: "{shader}",
                                                "{shader_display}"
                                            }
                                        }
                                    }}
                                }
                            }
                            div {
                                display: "flex", margin_left: "20px", margin_right: "20px",
                                background_color: "#303000",
                                p { flex: "85%", margin: "20px", "Elasticity" }
                                input {
                                    margin: "20px", flex: "15%", r#type: "number",
                                    // oninput: move |evt| {
                                    //     evt.prevent_default();
                                    //     let mut value = evt.value();
                                    //     if let Ok(parsed) = value.parse::<u32>() {
                                    //         gaps.set(parsed);
                                    //     } else {
                                    //         value = String::from("0");
                                    //         gaps.set(0);
                                    //     }
                                    //     config_update(Some("niri/config.kdl"), "gaps", &format!("    gaps {}", &value));
                                    // },
                                    // value: gaps()
                                }
                            }
                            div {
                                display: "flex", margin_left: "20px", margin_right: "20px",
                                background_color: "#303000",
                                p { flex: "90%", margin: "20px", "Wall Cycle" }
                                div {
                                    flex: "10%",
                                    label {
                                        flex: "10%", class: "switch", margin: "20px",
                                        input {
                                            // checked: cursor_trail(),
                                            // onclick: move |_| {
                                            //     cursor_trail.set(!cursor_trail());
                                            //     let c = format!("trail-cursor = {}", cursor_trail());
                                            //     config_update(Some("rio/config.toml"), "trail-cursor", &c);
                                            // },
                                            type: "checkbox"
                                        },
                                        span { class: "slider round"}
                                    }
                                }
                            }
                            div {
                                display: "flex", margin_left: "20px", margin_right: "20px",
                                background_color: "#303000",
                                p { flex: "90%", margin: "20px", "Paralax" }
                                div {
                                    flex: "10%",
                                    label {
                                        flex: "10%", class: "switch", margin: "20px",
                                        input {
                                            // checked: cursor_trail(),
                                            // onclick: move |_| {
                                            //     cursor_trail.set(!cursor_trail());
                                            //     let c = format!("trail-cursor = {}", cursor_trail());
                                            //     config_update(Some("rio/config.toml"), "trail-cursor", &c);
                                            // },
                                            type: "checkbox"
                                        },
                                        span { class: "slider round"}
                                    }
                                }
                            }
                            div {
                                display: "flex", margin_left: "20px", margin_right: "20px",
                                p { flex: "90%", margin: "20px", "Cursor Trail" }
                                div {
                                    flex: "10%",
                                    label {
                                        flex: "10%", class: "switch", margin: "20px",
                                        input {
                                            checked: cursor_trail(),
                                            onclick: move |_| {
                                                cursor_trail.set(!cursor_trail());
                                                let c = format!("trail-cursor = {}", cursor_trail());
                                                config_update(Some("rio/config.toml"), "trail-cursor", &c);
                                            },
                                            type: "checkbox"
                                        },
                                        span { class: "slider round"}
                                    }
                                }
                            }
                        }
                    }}
                }
            }
        }
    }
}
