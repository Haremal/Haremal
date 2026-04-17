use bluer::{Device, Session};
use dioxus::prelude::*;
use futures_util::StreamExt;
use udev::Enumerator;

#[derive(Clone, Debug)]
enum DeviceState {
    Wired(String),
    Connected(Device, String),
    Paired(Device, String),
    Nearby(Device, String),
}

#[component]
pub fn Devices() -> Element {
    let adapter =
        use_resource(|| async move { Session::new().await.ok()?.default_adapter().await.ok() });
    let mut devices = use_signal(Vec::new);
    let sync_handle = use_coroutine(move |mut rx: UnboundedReceiver<()>| async move {
        let adapter = loop {
            if let Some(Some(a)) = adapter.read().clone() {
                break a;
            }
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        };

        let Ok(events) = adapter.events().await else {
            return;
        };
        let Ok(discovery) = adapter.discover_devices().await else {
            return;
        };

        let mut events = Box::pin(events);
        let mut discovery = Box::pin(discovery);
        loop {
            let mut results = Vec::new();
            for device in get_wired_devices() {
                let clean_name = device.replace("\\x20", " ");
                let is_system = clean_name.contains("Host Controller")
                    || clean_name.contains("Root Hub")
                    || clean_name.chars().all(|c| c.is_numeric());
                if !is_system && !clean_name.is_empty() {
                    results.push(DeviceState::Wired(clean_name));
                }
            }
            if let Ok(addrs) = adapter.device_addresses().await {
                for addr in addrs {
                    if let Ok(dev) = adapter.device(addr) {
                        let name = dev.alias().await.unwrap_or_else(|_| addr.to_string());
                        let state = if dev.is_connected().await.unwrap_or(false) {
                            DeviceState::Connected(dev, name)
                        } else if dev.is_paired().await.unwrap_or(false) {
                            DeviceState::Paired(dev, name)
                        } else {
                            DeviceState::Nearby(dev, name)
                        };
                        results.push(state);
                    }
                }
            }
            results.sort_by_key(|d| match d {
                DeviceState::Wired(_) => 0,
                DeviceState::Connected(..) => 1,
                DeviceState::Paired(..) => 2,
                _ => 3,
            });
            devices.set(results);

            events.next().await;
            tokio::select! {
                _ = events.next() => {},
                _ = discovery.next() => {},
                _ = rx.next() => {},
                _ = tokio::time::sleep(std::time::Duration::from_millis(1000)) => {},
            }
        }
    });
    let is_on = use_resource(move || async move {
        if let Some(Some(a)) = adapter.read().as_ref() {
            return a.is_powered().await.unwrap_or(false);
        }
        false
    });
    let mut ui_on = use_signal(|| false);
    use_effect(move || {
        if let Some(val) = is_on.read().as_ref() {
            ui_on.set(*val);
        }
    });
    rsx! {
        div {
            class: "tab",
            h1 { "Devices" }
            div {
                div {
                    div {
                        align_items: "center", width: "100%",
                        p { float: "left", margin: "20px", "Bluetooth" }
                        div {
                            float: "right", margin: "20px",
                            label {
                                class: "switch",
                                onchange: move |evt| {
                                    let checked = evt.checked();
                                    spawn(async move {
                                        if let Some(Some(a)) = adapter.read().as_ref() {
                                            let _ = a.set_powered(checked).await;
                                        }
                                    });
                                    ui_on.set(checked);
                                },
                                input {
                                    type: "checkbox",
                                    checked: "{is_on().unwrap_or(false)}"
                                },
                                span { class: "slider round"}
                            }
                        }
                    }

                    div {
                        table {
                            for device in devices() {
                                match device {
                                    DeviceState::Wired(n) => rsx! { tr {
                                        td { colspan: 2, h2 { "{n}" } span { color: "#9a8c98", "WIRED" } }
                                    } },
                                    DeviceState::Connected(d, n) => {
                                        let d = d.clone();
                                        rsx! { tr {
                                            visibility: if !ui_on() { "hidden" },
                                            td { h2 { "{n}" } span { color: "#06d6a0", "CONNECTED" } }
                                            td { width: "10px", button { onclick: move |_| {
                                                let d = d.clone();
                                                spawn(async move {
                                                    let _ = d.disconnect().await;
                                                    sync_handle.send(());
                                                });
                                            }, "Disconnect" } }
                                        }
                                    } },
                                    DeviceState::Paired(d, n) => {
                                        let d1 = d.clone();
                                        let d2 = d.clone();
                                        rsx! { tr {
                                            visibility: if !ui_on() { "hidden" },
                                            td { h2 { "{n}" } span { color: "#ef476f", "DISCONNECTED" } }
                                            td { width: "10px",
                                                button { onclick: move |_| {
                                                    let d1 = d1.clone();
                                                    spawn(async move {
                                                        let _ = d1.connect().await;
                                                        sync_handle.send(());
                                                    });
                                                }, "Connect" },
                                                button { onclick: move |_| {
                                                    let d2 = d2.clone();
                                                    let adapter_opt = adapter.read().clone().flatten();
                                                    spawn(async move {
                                                        if let Some(a) = adapter_opt {
                                                            let _ = a.remove_device(d2.address()).await;
                                                            sync_handle.send(());
                                                        }
                                                    });
                                                }, "Remove" }
                                            }
                                        }
                                    } },
                                    DeviceState::Nearby(d, n) => rsx! { tr {
                                        visibility: if !ui_on() { "hidden" },
                                        td { h2 { "{n}" } span { color: "#118ab2", "AVAILABLE" } }
                                        td { width: "10px", text_align: "center",
                                            button { onclick: move |_| {
                                                let d = d.clone();
                                                let n = n.clone();
                                                spawn(async move {
                                                    let confirmed = rfd::AsyncMessageDialog::new()
                                                        .set_level(rfd::MessageLevel::Info)
                                                        .set_title("Confirm Pairing")
                                                        .set_description(format!("Pair this device: {}?", n))
                                                        .set_buttons(rfd::MessageButtons::YesNo)
                                                        .show()
                                                        .await;
                                                    if confirmed == rfd::MessageDialogResult::Yes {
                                                        if let Ok(s) = bluer::Session::new().await {
                                                            let agent = bluer::agent::Agent {
                                                                request_default: true,
                                                                request_confirmation: Some(Box::new(|_| Box::pin(async { Ok(()) }))),
                                                                ..Default::default()
                                                            };

                                                            if let Ok(_handle) = s.register_agent(agent).await {
                                                                let _ = d.pair().await;
                                                                let _ = d.set_trusted(true).await;
                                                                let _ = d.connect().await;
                                                            }
                                                        }
                                                        sync_handle.send(());
                                                    }
                                                });
                                            }, "Pair" }
                                        }
                                    } },
                                    _ => rsx!(),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn get_wired_devices() -> Vec<String> {
    let mut enumerator = Enumerator::new().unwrap();
    let mut results = Vec::new();
    for device in enumerator.scan_devices().unwrap() {
        if let Some(name) = device.property_value("ID_MODEL_ENC") {
            let clean_name = name.to_string_lossy();
            if !results.contains(&clean_name.to_string()) {
                results.push(clean_name.to_string());
            }
        }
    }
    results
}
