use std::net::{UdpSocket, SocketAddr};
use std::str;
use std::thread;
use serde::{Serialize, Deserialize};
use serde_json;
use bevy::{app::AppExit, prelude::*, window::ReceivedCharacter};

#[derive(Serialize, Deserialize)]
struct PlayerInfo {
    position: (f32, f32),
    health: u32,
    // Used basic fields for Player info for now
    // Add other relevant fields here
}

pub fn create_server(
    mut socket: ResMut<super::ServerSocket>
) {
    let server_address = "0.0.0.0:8080";
    socket.0 = Some(UdpSocket::bind("0.0.0.0:8080").expect("Failed to bind to address."));
    info!("{}", socket.0.is_some());
}

pub fn update(
    mut server_socket: ResMut<super::ServerSocket>
)
{
    //info!("{}", server_socket.0.is_some());
    let mut buf = [0; 1024];
    if server_socket.0.is_none() {
        return;
    }
    info!("receiving stuff");
    let socket = server_socket.0.as_mut().unwrap();
    socket.set_nonblocking(true).expect("cannot set nonblocking");

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
