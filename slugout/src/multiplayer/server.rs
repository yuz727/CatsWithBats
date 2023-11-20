use bevy::prelude::*;
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json;
use std::net::UdpSocket;
use std::str;

#[derive(Serialize, Deserialize)]
struct PlayerInfo {
    position: (f32, f32),
    health: u32,
    // Used basic fields for Player info for now
    // Add other relevant fields here
}

pub fn create_server(
    mut socket: ResMut<super::ServerSocket>,
    mut client_list: ResMut<super::ClientList>,
    server_address: Res<super::SocketAddress>,
) {
    // Use the server address from the resource
    let server_address_str = &server_address.0;

    socket.0 = Some(UdpSocket::bind(server_address_str).expect("Failed to bind to address s."));
    info!("{}", socket.0.is_some());

    // let's say that server was just created now
    println!("Created server {}", server_address_str);

    /*/ Create the host client
    let host_client = super::Client {
        address: socket.0.as_ref().unwrap().local_addr().unwrap(),
        username: String::from("hostuser"), // You might want to replace this with the actual username
    };

    // Add the host client to the client list
    client_list.clients.push(host_client);*/
}

pub fn update(
    mut server_socket: ResMut<super::ServerSocket>,
    mut client_list: ResMut<super::ClientList>,
) {
    //info!("{}", server_socket.0.is_some());
    let mut buf = [0; 1024];
    if server_socket.0.is_none() {
        return;
    }
    let socket = server_socket.0.as_mut().unwrap();
    socket
        .set_nonblocking(true)
        .expect("cannot set nonblocking");

    match socket.recv_from(&mut buf) {
        Ok((size, peer)) => {
            let client_msg = str::from_utf8(&buf[0..size]).expect("Bad data.");
            let clients = &mut client_list.clients;

            let player_number = if let Some(index) = clients.iter().position(|client| client.address == peer) {
                // Existing client, get the player number
                index + 1
            } else {
                // This is a new client, add it to the list
                clients.push(super::Client {
                    address: peer,
                    username: String::from(generate_username(10)),
                });
                println!("New client connected: {}", peer);
                clients.len() // Player number is the length of the client list
            };

            if let Ok(_player_info) = serde_json::from_str::<PlayerInfo>(&client_msg) {
                // Handle player_info and perform game logic here
                println!(
                    "Player {} - Received Player Info: Position: {:?}, Health: {}",
                    player_number, _player_info.position, _player_info.health
                );
            }

            socket
                .send_to(client_msg.as_bytes(), peer)
                .expect("Failed to send data");
        }
        Err(_e) => {
            //eprintln!("Error receiving data: {}", _e);
        }
    }
}

pub fn generate_username(length: usize) -> String {
    let mut rng = rand::thread_rng();
    std::iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(length)
        .collect()
}
