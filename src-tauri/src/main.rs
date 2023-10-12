// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::process::Command;
use tauri::{
    Manager, SystemTray, SystemTrayEvent, SystemTrayMenu
};
use tauri_plugin_positioner::{Position, WindowExt};

fn main() {
    let system_tray_menu = SystemTrayMenu::new();
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .system_tray(SystemTray::new().with_menu(system_tray_menu))
        .on_system_tray_event(|app, event| {
            tauri_plugin_positioner::on_tray_event(app, &event);
            match event {
                SystemTrayEvent::LeftClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    let window = app.get_window("main").unwrap();
                    // let _ = window.move_window(Position::TrayCenter);
                    let _ = window.move_window(Position::TopLeft);

                    if window.is_visible().unwrap() {
                        window.hide().unwrap();
                    } else {
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                }
                SystemTrayEvent::RightClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    let window = app.get_window("main").unwrap();
                    let _ = window.move_window(Position::TrayCenter);

                    if window.is_visible().unwrap() {
                        window.hide().unwrap();
                    } else {
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                }
                SystemTrayEvent::DoubleClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    println!("system tray received a double click");
                }
                SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    "hide" => {
                        let window = app.get_window("main").unwrap();
                        window.hide().unwrap();
                    }
                    _ => {}
                },
                _ => {}
            }
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::Focused(is_focused) => {
                // detect click outside of the focused window and hide the app
                // if !is_focused {
                //     event.window().hide().unwrap();
                // }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![scan_apps, get_focused_app])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn scan_apps() -> Vec<String> {
    let command;
    let args;

    if cfg!(target_os = "macos") {
        command = "sh";
        args = vec!["-c", "ls /Applications"];
    } else if cfg!(target_os = "linux") {
        command = "sh";
        args = vec!["-c", "ls /usr/share/applications"];
    } else if cfg!(target_os = "windows") {
        command = "powershell.exe";
        args = vec!["-Command", "Get-ChildItem 'C:\\Program Files' -Recurse -Filter '*.exe' | ForEach-Object { $_.Name }"];
    } else {
        panic!("Unsupported operating system");
    }

    let output = Command::new(command)
        .args(args)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let app_list: Vec<String> = stdout.lines().map(String::from).collect();
        return app_list;
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("{}", stderr);
        return Vec::new();
    }
}

#[tauri::command]
fn get_focused_app() -> Result<String, String> {
    let output = match Command::new("osascript")
        .arg("-e")
        .arg(
            r#"tell application "System Events"
                    set frontmostProcess to first process where it is frontmost
                    set processName to name of frontmostProcess
                    set processGroup to unix id of (first process where name of it is processName)
                end tell
                return processGroup"#,
        )
        .output()
    {
        Ok(output) => output,
        Err(err) => return Err(err.to_string()),
    };

    if output.status.success() {
        let pid_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let output = Command::new("ps")
            .arg("-p")
            .arg(pid_str)
            .arg("-o")
            .arg("comm=")
            .output();
        match output {
            Ok(result) => {
                if result.status.success() {
                    let path = String::from_utf8_lossy(&result.stdout).trim().to_string();
                    let app_name = extract_name(&path);
                    Ok(app_name.to_string())
                } else {
                    let error_msg = String::from_utf8_lossy(&result.stderr).trim().to_string();
                    eprintln!("命令执行错误：{}", error_msg);
                    Err("Failed to get focused app.".to_string())
                }
            }
            Err(err) => {
                eprintln!("执行命令出错：{}", err);
                Err("Failed to get focused app.".to_string())
            }
        }
    } else {
        Err("Failed to get focused app.".to_string())
    }
}

fn extract_name(path: &str) -> &str {
    // 首先根据 macOS 的路径分隔符 "/" 分割字符串
    let parts: Vec<&str> = path.split('/').collect();

    // 找到 ".app" 后缀的索引
    let index = parts.iter().position(|&part| part.ends_with(".app"));

    match index {
        Some(i) => {
            // 返回索引前面的部分作为名称
            let name = parts[i];
            name
        }
        None => {
            // 如果没有找到 ".app" 后缀，则返回整个路径
            path
        }
    }
}
