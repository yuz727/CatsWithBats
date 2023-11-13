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

pub fn create_client(
    mut socket: ResMut<super::ClientSocket>,
    mut socket_address: ResMut<super::SocketAddress>,
) {
    socket.0 = Some(UdpSocket::bind("0.0.0.0:8080").expect("Failed to bind to address."));   
}

pub fn update(
    mut client_socket: ResMut<super::ClientSocket>,
    mut socket_address: ResMut<super::SocketAddress>,
)
{
    let mut buf = [0; 1024];

    if client_socket.0.is_none() {
        return;
    }

    let socket = client_socket.0.as_mut().unwrap();
    socket.set_nonblocking(true).expect("cannot set nonblocking");

    let mut player_info = PlayerInfo {
        position: (42.0, 24.0),
        health: 100,
    };

    let mut move_left = true;

    if move_left {
        player_info.position.0 -= 1.0; // Move left
    } else {
        player_info.position.0 += 1.0; // Move right
    }

    let message = serde_json::to_string(&player_info).expect("Failed to serialize");

    socket.send_to(message.as_bytes(), socket_address.0.as_str()).expect("Failed to send data.");

    let mut response = [0; 1024];

    match socket.recv_from(&mut response) {
        Ok((size, peer)) => {
            let response_str = std::str::from_utf8(&response[0..size]).expect("Bad data.");
            println!("Received response: {}", response_str); 

            // Toggle the movement direction
            // ** THIS IS ONLY USED TO EMULATE CHANGE/MOVEMENT **
            move_left = !move_left;
        }
        Err(e) => {
            //eprintln!("Error receiving data: {}", e);
        }
    }
    

    // Sleep to control the sending frequency 
}