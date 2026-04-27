use dioxus::prelude::*;
use std::process::Stdio;
use tokio::io::AsyncBufReadExt;

#[component]
pub fn Update() -> Element {
    let mut is_updating = use_signal(|| false);
    let mut update_msg = use_signal(|| "");
    let mut console_output = use_signal(|| String::from("Loading..."));

    use_resource(move || async move {
        if let Ok(output) = std::process::Command::new("fastfetch")
            .arg("--pipe")
            .output()
        {
            if let Ok(text) = String::from_utf8(output.stdout) {
                console_output.set(text);
            }
        }
    });
    rsx! {
        div {
            class: "tab",
            div {
                display: "flex", align_items: "center",
                h1 { flex: "60%", "Update" }
                div {
                    align_items: "center", flex: "33%",
                    button {
                        disabled: is_updating(),
                        onclick: move |_| {
                            is_updating.set(true);
                            update_msg.set("Updating...");
                            spawn(async move {
                                console_output.set("--- Cleaning Cache ---\n".to_string());
                                let mut clean_child = tokio::process::Command::new("sudo")
                                    .args(["pacman", "-Sc", "--noconfirm"])
                                    .stderr(Stdio::piped())
                                    .stdout(Stdio::piped())
                                    .spawn()
                                    .expect("Failed to clean cache");
                                let stdout = clean_child.stdout.take().unwrap();
                                let mut reader = tokio::io::BufReader::new(stdout).lines();
                                while let Ok(Some(line)) = reader.next_line().await {
                                    console_output.with_mut(|out| out.push_str(&format!("{}\n", line)));
                                }
                                let _ = clean_child.wait().await;
                                console_output.with_mut(|out| out.push_str("\n--- Updating System ---\n"));
                                let mut update_child = tokio::process::Command::new("sudo")
                                    .args(["pacman", "-Syu", "--noconfirm"])
                                    .stderr(Stdio::piped())
                                    .stdout(Stdio::piped())
                                    .spawn()
                                    .expect("Failed to update");
                                let stdout = update_child.stdout.take().unwrap();
                                let mut reader = tokio::io::BufReader::new(stdout).lines();
                                while let Ok(Some(line)) = reader.next_line().await {
                                    console_output.with_mut(|out| out.push_str(&format!("{}\n", line)));
                                }
                                let status = update_child.wait().await;
                                is_updating.set(false);
                                update_msg.set(if status.is_ok() { "Up to date" } else { "Something went wrong. Try again" });
                            });
                        },
                        float: "left", margin: "7px",
                        "Check For Updates"
                    }
                }
            }
            p {
                color: "#555555",
                width: "200px",
                text_align: "center",
                margin_top: "-20px",
                font_size: "20px",
                float: "right",
                "{update_msg()}"
            }
            pre {
                position: "absolute", top: "10vh",
                max_height: "400px", overflow_y: "auto",
                color: "#06d6a0", padding: "15px", font_size: "16px", opacity: "80%",
                "{console_output}"
            }
        }
    }
}
