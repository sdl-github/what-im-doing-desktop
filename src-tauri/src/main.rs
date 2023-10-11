// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{ process::Command};
use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "关闭窗口");
    let hide = CustomMenuItem::new("hide".to_string(), "隐藏窗口");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);
    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| menu_handle(app, event))
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
        },
        None => {
            // 如果没有找到 ".app" 后缀，则返回整个路径
            path
        },
    }
}

fn menu_handle(app_handle: &tauri::AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            println!("鼠标-左击");
        }
        SystemTrayEvent::RightClick {
            position: _,
            size: _,
            ..
        } => {
            println!("鼠标-右击");
        }
        SystemTrayEvent::DoubleClick {
            position: _,
            size: _,
            ..
        } => {
            println!("鼠标-双击");
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                std::process::exit(0);
            }
            "hide" => {
                let item_handle = app_handle.tray_handle().get_item(&id);
                let window = app_handle.get_window("home").unwrap();
                if window.is_visible().unwrap() {
                    window.hide().unwrap();
                    item_handle.set_title("显示窗口").unwrap();
                } else {
                    window.show().unwrap();
                    item_handle.set_title("隐藏窗口").unwrap();
                }
            }
            _ => {}
        },
        _ => {}
    }
}
