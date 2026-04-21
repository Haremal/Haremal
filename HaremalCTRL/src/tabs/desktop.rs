use crate::{config_read, config_remove, config_update, config_write, monitors};
use dioxus::prelude::*;

const TRANSFORMS: &[&str] = &[
    "normal",
    "90",
    "180",
    "270",
    "flipped",
    "flipped-90",
    "flipped-180",
    "flipped-270",
];
#[derive(Clone, PartialEq)]
struct Dash {
    app: String,
    height: i32,
}

#[component]
pub fn Desktop() -> Element {
    let mut monitors = use_signal(monitors);
    let mut dragging_id: Signal<Option<String>> = use_signal(|| None);
    let mut offset = use_signal(|| (0.0, 0.0));
    let mut current_monitor: Signal<Option<String>> = use_signal(|| None);
    let mut dashboard = use_signal(|| !config_read(None, "dashboard").is_empty());
    let mut columns = use_signal(|| {
        let conf = config_read(None, "dashboard");
        if conf.is_empty() {
            return Vec::<i32>::new();
        }
        conf[0]
            .trim()
            .replace("dashboard", "")
            .split_whitespace()
            .map(|s| s.parse().unwrap_or(1))
            .collect()
    });
    rsx! {
        div {
            class: "tab",
            h1 { "Desktop" },
            div {
                div {
                    background_color: "#303000",
                    margin: "20px",
                    h3 { margin: 0, "Screens" }
                    form {
                        div {
                            overflow: "hidden",
                            position: "relative", height: "300px", width: "600px", border: "1px solid white",
                            onpointerup: move |evt| {
                                evt.stop_propagation();
                                if let Some(id) = dragging_id() {
                                    dragging_id.set(None);
                                    for monitor in monitors() {
                                        config_update(Some("niri/config.kdl"), &format!("// position {}", id), &format!("    position x={} y={} // position {}", monitor.x, monitor.y, id));
                                    }
                                }
                            },
                            onpointermove: move |evt| {
                                if let Some(id) = dragging_id() {
                                    let client = evt.client_coordinates();
                                    let (start_x, start_y) = offset();

                                    let dx = client.x - start_x;
                                    let dy = client.y - start_y;

                                    monitors.write().iter_mut().for_each(|m| {
                                        if m.id == id {
                                            m.x += (dx * 13.0) as i32;
                                            m.y += (dy * 13.0) as i32;
                                        }
                                    });
                                    offset.set((client.x, client.y));
                                }
                            },
                            for monitor in monitors() {{
                                let is_dragging = dragging_id() == Some(monitor.id.clone());
                                let width = (monitor.width as f64 / 13.0) / monitor.scale;
                                let height = (monitor.height as f64 / 13.0) / monitor.scale;
                                let x = (monitor.x as f64 / 13.0) - (width / 2.0);
                                let y = (monitor.y as f64 / 13.0) - (height / 2.0);
                                rsx! {
                                    div {
                                        position: "absolute", left: "{x+300.0}px", top: "{y+150.0}px", width: "{width}px", height: "{height}px",
                                        text_align: "center", align_content: "center",
                                        border: "1px solid white",
                                        pointer_events: if is_dragging { "none" } else { "auto" },
                                        onpointerdown: move |evt| {
                                            evt.stop_propagation();
                                            let client = evt.client_coordinates();
                                            offset.set((client.x, client.y)); // Start the "anchor" point
                                            dragging_id.set(Some(monitor.id.clone()));
                                            current_monitor.set(Some(monitor.id.clone()));
                                        },
                                        "{monitor.id}"
                                    }
                                }
                            }}
                        }
                        for monitor in monitors().iter().filter(|m| { m.id == current_monitor().unwrap_or_default() }) {
                            div {
                                display: "flex", height: "20px", font_size: "10px",
                                p { flex: "90%", "x: {monitor.x} / y: {monitor.x}" }
                                button { flex: "10%", padding: 0,
                                    onclick: move |_| {
                                        monitors.write().iter_mut().for_each(|m| {
                                            if m.id == current_monitor().unwrap_or_default() {
                                                m.x = 0;
                                                m.y = 0;
                                                config_update(Some("niri/config.kdl"), &format!("// position {}", current_monitor().unwrap_or_default()), &format!("    position x={} y={} // position {}", m.x, m.y, current_monitor().unwrap_or_default()));
                                            }
                                        });
                                    }, "RESET"
                                }
                            }
                            div {
                                display: "flex", height: "40px", margin_top: "10px",
                                p { flex: "80%", margin: 0, "Mode: " }
                                select {
                                    flex: "20%", height: "30px",
                                    onchange: move |evt| {
                                        evt.prevent_default();
                                        monitors.write().iter_mut().for_each(|m| {
                                            if m.id == current_monitor().unwrap_or_default() {
                                                m.current_mode = evt.value();
                                                config_update(Some("niri/config.kdl"), &format!("// mode {}", current_monitor().unwrap_or_default()), &format!("    mode \"{}\" // mode {}", evt.value(), current_monitor().unwrap_or_default()));
                                            }
                                        });
                                    },
                                    for mode in &monitor.modes {
                                        option {
                                            selected: *mode == monitor.current_mode,
                                            value: "{mode}",
                                            "{mode}"
                                         }
                                    }
                                }
                            }
                            div {
                                display: "flex", height: "40px",
                                p { flex: "80%", margin: 0, "Scale" }
                                input {
                                    flex: "20%", r#type: "number", margin_top: "-5px",
                                    min: "0.1", step: "0.1",
                                    oninput: move |evt| {
                                        evt.prevent_default();
                                        if 0.1 > evt.value().parse().unwrap_or(0.0) { return; }
                                        monitors.write().iter_mut().for_each(|m| {
                                            if m.id == current_monitor().unwrap_or_default() {
                                                m.scale = evt.value().parse().unwrap_or(1.0);
                                                config_update(Some("niri/config.kdl"), &format!("// scale {}", current_monitor().unwrap_or_default()), &format!("    scale {} // scale {}", evt.value(), current_monitor().unwrap_or_default()));
                                            }
                                        });
                                    },
                                    value: monitor.scale
                                }
                            }
                            div {
                                display: "flex", height: "40px",
                                p { flex: "80%", margin: 0, "Transform" }
                                select {
                                    flex: "20%", height: "30px",
                                    onchange: move |evt| {
                                        evt.prevent_default();
                                        monitors.write().iter_mut().for_each(|m| {
                                            if m.id == current_monitor().unwrap_or_default() {
                                                m.transform = evt.value();
                                                config_update(Some("niri/config.kdl"), &format!("// transform {}", current_monitor().unwrap_or_default()), &format!("    transform \"{}\" // transform {}", evt.value(), current_monitor().unwrap_or_default()));
                                            }
                                        });
                                    },
                                    for t in TRANSFORMS {
                                        option {
                                            selected: monitor.transform.contains(*t),
                                            value: "{t}",
                                            "{t}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                form {
                    background_color: "#303000",
                    display: "flex", margin: "20px",
                    h3 { flex: "70%",margin: 0, "Sleep Time" }
                    input {
                        flex: "20%", r#type: "number",
                        oninput: move |evt| {
                            evt.prevent_default();
                        },
                        // value: cursor_size()
                    }
                    div {
                        flex: "10%",
                        label {
                            class: "switch",
                            input {
                                // checked: cursor_trail(),
                                onclick: move |_| {
                                    // cursor_trail.set(!cursor_trail());
                                    // let c = format!("trail-cursor = {}", cursor_trail());
                                    // config_update(Some("rio/config.toml"), "trail-cursor", &c);
                                },
                                type: "checkbox"
                            },
                            span { class: "slider round" }
                        }
                    }
                }
                form {
                    margin: "20px",
                    div {
                        display: "flex",
                        h3 { flex: "60%", margin: 0, "Dashboard" }
                        button {
                            visibility: if !dashboard() { "hidden" },
                            flex: "10%",
                            "SAVE"
                        }
                        input {
                            visibility: if !dashboard() { "hidden" },
                            flex: "20%", r#type: "number", min: 0, max: 5,
                            oninput: move |evt| {
                                evt.prevent_default();
                                let value = evt.value().parse().unwrap_or(0);
                                if value > 5 { return; }
                                let mut vec = columns().clone();
                                if vec.len() < value {
                                    while vec.len() < value { vec.push(1); }
                                } else {
                                    vec.truncate(value);
                                }
                                let c = vec.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" ");
                                if !config_read(None, "dashboard").is_empty() {
                                    config_update(None, "dashboard", &format!("dashboard {}", c));
                                }
                                columns.set(vec);
                            },
                            value: columns().len()
                        }
                        div {
                            flex: "10%",
                            label {
                                class: "switch",
                                input {
                                    checked: dashboard(),
                                    onclick: move |_| {
                                        let turned = config_read(None, "dashboard").is_empty();
                                        let c = columns().iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" ");
                                        dashboard.set(turned);
                                        if turned {
                                            config_write(None, "", &format!("dashboard {}", c));
                                        } else {
                                            config_remove(None, "dashboard");
                                        }
                                    },
                                    type: "checkbox"
                                },
                                span { class: "slider round"}
                            }
                        }
                    }
                    div {
                        visibility: if !dashboard() { "hidden" },
                        border: "1px solid white", height: "300px", width: "600px",
                        display: "flex", padding: "10px",
                        onpointerup: move |evt| {
                            evt.stop_propagation();
                            if let Some(id) = dragging_id() {
                                dragging_id.set(None);
                                // for monitor in monitors() {
                                //     config_update(Some("niri/config.kdl"), &format!("// position {}", id), &format!("    position x={} y={} // position {}", monitor.x, monitor.y, id));
                                // }
                            }
                        },
                        onpointermove: move |evt| {
                            if let Some(id) = dragging_id() {
                                let client = evt.client_coordinates();
                                let (start_x, start_y) = offset();

                                let dx = client.x - start_x;
                                let dy = client.y - start_y;

                                // monitors.write().iter_mut().for_each(|m| {
                                //     if m.id == id {
                                //         m.x += (dx * 13.0) as i32;
                                //         m.y += (dy * 13.0) as i32;
                                //     }
                                // });
                                offset.set((client.x, client.y));
                            }
                        },
                        for (i, c) in columns().iter().enumerate() {
                            div {
                                position: "relative",
                                display: "flex", flex_direction: "column",
                                border: "1px solid yellow", width: "100%", height: "100%", flex: 1,
                                for i in 0..*c {
                                    div {
                                        border: "1px solid yellow", width: "100%", height: "100%",
                                        align_items: "end", flex: 1,
                                        if i != 0 {
                                            div {
                                                background_color: "green", height: "3px", margin_top: "-2px",
                                            }
                                        }
                                    }
                                }
                                div {
                                    visibility: if !dashboard() { "hidden" },
                                    position: "absolute", top: "300px", width: "100%",
                                    input {
                                        r#type: "number", min: 1, max: 5,
                                        oninput: move |evt| {
                                            evt.prevent_default();
                                            let value = evt.value().parse().unwrap_or(0);
                                            if value > 5 { return; }
                                            let mut vec = columns().clone();
                                            vec[i] = value;
                                            let c = vec.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" ");
                                            if !config_read(None, "dashboard").is_empty() {
                                                config_update(None, "dashboard", &format!("dashboard {}", c));
                                            }
                                            columns.set(vec);
                                        },
                                        value: columns()[i]
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// https://mintlify.wiki/niri-wm/niri/configuration/named-workspaces
