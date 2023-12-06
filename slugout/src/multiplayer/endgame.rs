use std::net::SocketAddr;
use bevy::ecs::{system::{ResMut, Query}, query::{With, Without}};
use crate::game::components::*;

pub fn check_health(
    mut server_socket: ResMut<super::ServerSocket>,
    client_info: ResMut<super::ClientSocket>,
    mut player_query: Query<&mut Player, (With<Player>, Without<Ball>)>,
) {
    for player in player_query.iter_mut() {
        // Find the client associated with the player's address
        if (player.health == 0) {
           // Despawn player
           
        }
    }
}

pub fn check_end_game(mut client_list: ResMut<ClientListVector>) {
    // Collect the client addresses to be removed
    let clients_to_remove: Vec<SocketAddr> = client_list
        .authenticated_clients
        .iter()
        .filter(|client| client.player_info.clone().unwrap().health == 0)
        .map(|client| client.client_address)
        .collect();

    // Remove the clients from the map and vector
    for address in clients_to_remove {
        client_list.clients.remove(&address);
        client_list.authenticated_clients.retain(|client| client.client_address != address);
    }
    // Check if all clients except for yourself have health equal to 0
    let all_clients_dead = client_list.authenticated_clients.iter().all(|client| client.player_info.clone().unwrap().health == 0);
    if all_clients_dead {
        println!("Game over");
        // Condition to exit the game
        std::process::exit(0); // This will terminate the program
    }
}