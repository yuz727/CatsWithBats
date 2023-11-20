use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;
use std::net::{SocketAddr, UdpSocket};
use std::str;

use crate::game::components::{Player, Face, Bat};

#[derive(Serialize, Deserialize)]
struct PlayerInfo {
    position: (f32, f32),
    health: u32,
    // Add other relevant fields here
}

#[derive(Component)]
pub struct PlayerVelocity {
    pub velocity: Vec2,
}

impl PlayerVelocity {
    pub fn new() -> Self {
        Self {
            velocity: Vec2::splat(0.),
        }
    }
}

pub fn create_client(
    mut socket: ResMut<super::ClientSocket>,
    mut client_list: ResMut<super::ClientList>,
    server_address: Res<super::SocketAddress>,
    mut query: Query<(&Transform, &PlayerVelocity)>,
) {
    // Use the server address from the resource
    let server_address_str = &server_address.0;

    // Parse the server address string into SocketAddr
    if let Ok(server_address) = server_address_str.parse::<SocketAddr>() {
        socket.0 = Some(UdpSocket::bind("0.0.0.0:0").expect("Failed to bind to address c."));
        socket
            .0
            .as_mut()
            .unwrap()
            .connect(server_address)
            .expect("Failed to connect to the server.");
    } else {
        eprintln!("Invalid server address format: {}", server_address_str);
    }
    println!("Connected to server {}", server_address_str);
    // Create the new client
    let new_client = super::Client {
        address: socket.0.as_ref().unwrap().local_addr().unwrap(),
        username: String::from("user"), // You might want to replace this with the actual username
    };

    // Add the first client to the client list
    client_list.clients.push(new_client);
}

pub fn update(
    mut client_socket: ResMut<super::ClientSocket>,
    socket_address: Res<super::SocketAddress>,
    mut query: Query<(&Transform, &PlayerVelocity)>,
) {
    let mut _buf = [0; 1024];

    if client_socket.0.is_none() {
        return;
    }

    let socket = client_socket.0.as_mut().unwrap();
    socket
        .set_nonblocking(true)
        .expect("cannot set nonblocking");

    let (transform, velocity) = match query.iter().next() {
        Some((transform, velocity)) => (transform, velocity),
        None => {
            // Handle the case when there are no entities matching the query
            println!("No entities found matching the query.");
            return;
        }
    };

    let player_info = PlayerInfo {
        position: (transform.translation.x, transform.translation.y),
        health: 100,
    };

    let message = serde_json::to_string(&player_info).expect("Failed to serialize");

    let server_address_str = &socket_address.0;
    socket
        .send_to(message.as_bytes(), server_address_str)
        .expect("Failed to send data.");

    let mut response = [0; 1024];

    match socket.recv_from(&mut response) {
        Ok((size, _peer)) => {
            let response_str = std::str::from_utf8(&response[0..size]).expect("Bad data.");
            println!("Received response: {}", response_str);
        }
        Err(_e) => {
            //eprintln!("Error receiving data: {}", e);
        }
    }

    // Sleep to control the sending frequency
}
