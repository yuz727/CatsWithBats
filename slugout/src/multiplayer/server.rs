use bevy::prelude::*;
use bevy::transform::commands;
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json;
use std::net::UdpSocket;
use std::str;

use crate::game::components::{Player, Face, Bat};

use super::{ConnectRequest, ConnectResponse};

#[derive(Serialize, Deserialize)]
struct PlayerInfo {
    position: (f32, f32),
    health: u32,
    // Add other relevant fields here
}

#[derive(Serialize, Deserialize)]
struct YarnBall {
    position: (f32, f32),
}

pub fn create_server(
    mut socket: ResMut<super::ServerSocket>,
    mut _client_list: ResMut<super::ClientList>,
    server_address: Res<super::SocketAddress>,
) {
    // Use the [IP Address]:[Port#] from user input (server_address from mod.rs in Multiplayer)
    let server_address_str = &server_address.0;
    // Attempt to bind the socket to the specified address
    socket.0 = Some(UdpSocket::bind(server_address_str).expect("Failed to bind to address s."));
    // Log whether the server socket was successfully created
    info!("{}", socket.0.is_some());
    // let's say that server was just created now
    println!("Created server {}", server_address_str);
}

pub fn update(
    mut server_socket: ResMut<super::ServerSocket>,
    mut client_list: ResMut<super::ClientList>,
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    // Create a buffer to store incoming data
    let mut buf = [0; 1024];
    // Check if the server socket is not yet initialized
    if server_socket.0.is_none() {
        // If not initialized, return early from the function
        return;
    }
    // Unwrap to get a mutable reference to the server socket
    let socket = server_socket.0.as_mut().unwrap();
    // Set the socket to non-blocking mode
    socket
        .set_nonblocking(true)
        .expect("Cannot set nonblocking");
    // Attempt to receive data from the socket
    match socket.recv_from(&mut buf) {
        // If data is successfully received
        Ok((size, peer)) => {
            // Convert the received bytes to a string
            let client_msg = str::from_utf8(&buf[0..size]).expect("Bad data.");
            // Get a mutable reference to the client list
            let clients = &mut client_list.clients;
            // Determine the player number based on whether the client already exists
            let player_number =
                if let Some(index) = clients.iter().position(|client| client.address == peer) {
                    // Existing client, get the player number
                    index + 1
                } else {
                    // This is a new client, add it to the list
                    clients.push(super::Client {
                        address: peer,
                        username: String::from(generate_username(10)),
                    });
                    println!("New client connected: {}", peer);
                    // Assign player number as the length of the client list
                    clients.len() 
                };
            // Check if the received message is a connection request from a new client
            if let Ok(_connect_request) = serde_json::from_str::<ConnectRequest>(&client_msg) {
                // Only spawn the player if the player_number is not 1
                if player_number != 1 {
                    // just spawn the png's for player for now (need to make it work to show different Player#.png's)
                    commands
                        .spawn(SpriteBundle {
                            texture: asset_server.load("Player2.png"),
                            transform: Transform::with_scale(Transform::from_xyz(0., 0., 10.), Vec3::splat(0.13)),
                            ..default()
                        });

                    commands
                        .spawn(SpriteBundle {
                            texture: asset_server.load("Face.png"),
                            transform: Transform::with_scale(Transform::from_xyz(0., 0., 20.), Vec3::splat(0.13)),
                            ..default()
                        });

                    commands
                        .spawn(SpriteBundle {
                            texture: asset_server.load("Bat.png"),
                            transform: Transform::with_scale(
                                Transform::from_xyz(-5., 0., 20.),
                                Vec3::new(0.175, 0.175, 0.),
                            ),
                            ..default()
                        });
                }
                /*/ Send a response back to the client with the assigned player ID
                let response_msg = serde_json::to_string(&ConnectResponse { player_number }).expect("Failed to serialize");
                socket
                    .send_to(response_msg.as_bytes(), peer)
                    .expect("Failed to send data");*/
            }
            if let Ok(player_info) = serde_json::from_str::<PlayerInfo>(&client_msg) {
                // Handle player_info and perform game logic here
                println!(
                    "Player {} - Received Player Info: Position: {:?}, Health: {}",
                    player_number, player_info.position, player_info.health
                );            
            }

            // Send the received message back to the client
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
