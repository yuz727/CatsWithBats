use std::net::{UdpSocket, SocketAddr};
use std::str;
use std::thread;
use serde::{Serialize, Deserialize};
use serde_json;
use bevy::{app::AppExit, prelude::*, window::ReceivedCharacter};
use std::sync::{Arc, Mutex};
use std::net::SocketAddr;
use rand::Rng;
use rand::distributions::Alphanumeric;


pub struct Client {
    pub address: std::net::SocketAddr,
    pub username: String,
    pub player_info: PlayerInfo,
}
pub struct ClientList {
    pub clients: Arc<Mutex<Vec<Client>>>,
}


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
    socket.0 = Some(UdpSocket::bind("127.0.0.1:8080").expect("Failed to bind to address."));
    info!("{}", socket.0.is_some());

    // let's say that server was just created now 

     // Create the first client
     let first_client = Client {
        address: std::net::SocketAddr::from(([127, 0, 0, 1], 8080)),
        username: String::from("hostuser"),
    };

    // Add the first client to the client list
    client_list.clients.lock().unwrap().push(first_client);

}

pub fn update(
    mut server_socket: ResMut<super::ServerSocket>
    client_list: ResMut<ClientList>
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

            let mut clients = client_list.clients.lock().unwrap();
            if !clients.iter().any(|client| client.address == peer) {
                // This is a new client, add it to the list
                clients.push(Client {
                    address: peer,
                    username: String::from(generate_username(10)), 
                });
                println!("New client connected: {}", peer);
            }

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

pub fn generate_username(length: usize) -> String {
    let mut rng = rand::thread_rng();
    std::iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(length)
        .collect()
}