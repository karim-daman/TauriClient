// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::io::ErrorKind;
use std::net::UdpSocket;
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

fn main() {
    let _menu = Menu::new(); // configure the menu
                             // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let close = CustomMenuItem::new("close".to_string(), "Close");
    let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(submenu);

    tauri::Builder::default()
        .menu(menu)
        .invoke_handler(tauri::generate_handler![greet, scan])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn scan() -> String {
    let broadcast_address = "255.255.255.255:12345";
    let input_message = "AddressRequest";

    let socket = UdpSocket::bind("0.0.0.0:1234").expect("Failed to bind UDP socket");
    socket.set_nonblocking(true).unwrap();
    socket
        .set_broadcast(true)
        .expect("Failed to enable broadcast mode");

    socket
        .send_to(input_message.as_bytes(), broadcast_address)
        .expect("Failed to send UDP packet");

    println!(
        "Sent broadcast message: {} to {}",
        input_message, broadcast_address
    );

    let mut buffer: [u8; 1024] = [0u8; 1024];

    let start = std::time::Instant::now();

    loop {
        match socket.recv_from(&mut buffer) {
            Ok((num_bytes, _src_addr)) => {
                let message = String::from_utf8_lossy(&buffer[..num_bytes]);
                // break format!("Received message: '{}' from {}", message, src_addr);
                break format!(
                    "{{ \"success\": true, \"message\": \"{}\", \"src_addr\": \"{}\" }}",
                    message, _src_addr
                );
            }
            Err(err) if err.kind() == ErrorKind::WouldBlock => {
                if start.elapsed() <= std::time::Duration::from_secs(1) {
                    continue;
                }
                break format!("{{\"success\": false, \"message\":\"{}\"}}", err);
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}
