use std::net::{UdpSocket, SocketAddr};
use std::str;
use std::thread;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct PlayerInfo {
    position: (f32, f32),
    health: u32,
    // Used basic fields for Player info for now
    // Add other relevant fields here
}

pub fn create_server() {
    let server_address = "0.0.0.0:8080";
    let socket = UdpSocket::bind(server_address).expect("Failed to bind to address.");
    socket.set_nonblocking(true).expect("cannot set nonblocking");

    println!("Someone joined!");

    let mut buf = [0; 1024];

    loop {
        match socket.recv_from(&mut buf) {
            Ok((size, peer)) => {
                let client_msg = str::from_utf8(&buf[0..size]).expect("Bad data.");

                if let Ok(player_info) = serde_json::from_str::<PlayerInfo>(&client_msg) {
                    // Handle player_info and perform game logic here
                    println!(
                        "Received Player Info: Position: {:?}, Health: {}",
                        player_info.position, player_info.health
                    );
                }

                socket.send_to(client_msg.as_bytes(), peer).expect("Failed to send data");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
            }
        }
    }
}