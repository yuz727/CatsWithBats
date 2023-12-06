// use crate::components::*;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    multiplayer::{ClientSocket, SocketAddress},
    MAP,
};

use super::components::{Bat, Face, Player};

use crate::game::npc::*;

const PLAYER_SIZE: f32 = 30.;
// 5px/frame @60Hz == 300px/s
const PLAYER_SPEED: f32 = 300.;
const RUG_SPEED: f32 = 200.;
// 1px/frame^2 @60Hz == 3600px/s^2
const ACCEL_RATE: f32 = 58000.;
const NPC_SIZE: f32 = 30.;

#[derive(Serialize, Deserialize)]
struct PlayerInfo {
    position: (f32, f32),
    health: u32,
    // Add other relevant fields here
}

#[derive(Component)]
pub struct PlayerVelocity {
    pub velocity: Vec2,
    pub prev_position: (f32, f32),
    pub confused: bool, 
}

impl PlayerVelocity {
    pub fn new() -> Self {
        Self {
            velocity: Vec2::splat(0.),
            prev_position: (0.0, 0.0),
            confused: false, 
        }
    }
}
pub fn move_player(
    input: Res<Input<KeyCode>>,
    mut player: Query<
        (&mut Transform, &mut PlayerVelocity),
        (With<Player>, Without<Face>, Without<Bat>),
    >,
    //  mut face: Query<&mut Transform, (With<Face>, Without<Player>, Without<Bat>)>,
    time: Res<Time>,
) {
    let (mut transform, mut velocity) = player.single_mut();
    //  let mut face_transform = face.single_mut();
    //let mut bat_transform = bat.single_mut();

    let mut deltav = Vec2::splat(0.);
    if input.pressed(KeyCode::A) {
        deltav.x -= 1000.;
    }
    if input.pressed(KeyCode::D) {
        deltav.x += 1000.;
    }
    if input.pressed(KeyCode::W) {
        deltav.y += 1000.;
    }
    if input.pressed(KeyCode::S) {
        deltav.y -= 1000.;
    }

    // confusion check
    if velocity.confused {
        deltav = -deltav;
    }

    let deltat = time.delta_seconds();

    // Calculate change in vector
    let acc = ACCEL_RATE * deltat;

    if unsafe { MAP == 1 } {
        let rug_collision = bevy::sprite::collide_aabb::collide(
            Vec3::new(0., 0., 1.),
            Vec2::new(732.6, 507.6),
            transform.translation,
            Vec2::new(PLAYER_SIZE, PLAYER_SIZE),
        );
        if (rug_collision == Some(bevy::sprite::collide_aabb::Collision::Right))
            || (rug_collision == Some(bevy::sprite::collide_aabb::Collision::Left))
            || (rug_collision == Some(bevy::sprite::collide_aabb::Collision::Top))
            || (rug_collision == Some(bevy::sprite::collide_aabb::Collision::Bottom))
            || (rug_collision == Some(bevy::sprite::collide_aabb::Collision::Inside))
        {
            velocity.velocity = if deltav.length() > 0. {
                (velocity.velocity + (deltav.normalize_or_zero() * acc)).clamp_length_max(RUG_SPEED)
            } else if velocity.velocity.length() > acc {
                velocity.velocity + (velocity.velocity.normalize_or_zero() * -acc)
            } else {
                Vec2::splat(0.)
            };
        } else {
            velocity.velocity = if deltav.length() > 0. {
                (velocity.velocity + (deltav.normalize_or_zero() * acc))
                    .clamp_length_max(PLAYER_SPEED)
            } else if velocity.velocity.length() > acc {
                velocity.velocity + (velocity.velocity.normalize_or_zero() * -acc)
            } else {
                Vec2::splat(0.)
            };
        }
        velocity.velocity = velocity.velocity * deltat;
        velocity.velocity = collision_check_map1(
            transform.translation,
            velocity.velocity,
            Vec3::splat(10000.),
        );
    } else if unsafe { MAP == 2 || MAP == 3 } {
        velocity.velocity = if deltav.length() > 0. {
            (velocity.velocity + (deltav.normalize_or_zero() * acc)).clamp_length_max(PLAYER_SPEED)
        } else if velocity.velocity.length() > acc {
            velocity.velocity + (velocity.velocity.normalize_or_zero() * -acc)
        } else {
            Vec2::splat(0.)
        };
        velocity.velocity = velocity.velocity * deltat;
    } else if unsafe { MAP == 4 } {
        velocity.velocity = if deltav.length() > 0. {
            (velocity.velocity + (deltav.normalize_or_zero() * acc)).clamp_length_max(PLAYER_SPEED)
        } else if velocity.velocity.length() > acc {
            velocity.velocity + (velocity.velocity.normalize_or_zero() * -acc)
        } else {
            Vec2::splat(0.)
        };
        velocity.velocity = velocity.velocity * deltat;
        velocity.velocity = collision_check_map4(
            transform.translation,
            velocity.velocity,
            Vec3::splat(10000.),
        );
    }

    // movement
    transform.translation.x = (transform.translation.x + velocity.velocity.x).clamp(
        -(1280. / 2.) + PLAYER_SIZE / 2.,
        1280. / 2. - PLAYER_SIZE / 2.,
    );

    transform.translation.y = (transform.translation.y + velocity.velocity.y).clamp(
        -(720. / 2.) + PLAYER_SIZE / 2.,
        720. / 2. - PLAYER_SIZE / 2.,
    );

    // face_transform.translation.x = transform.translation.x;
    // face_transform.translation.y = transform.translation.y;
    //bat_transform.translation.x = transform.translation.x - 5.;
    //bat_transform.translation.y = transform.translation.y;
}

pub fn player_npc_collisions(
    mut player: Query<&mut Transform, (With<Player>, Without<NPC>)>,
    mut npc_query: Query<&mut Transform, (With<NPC>, Without<Player>)>,
) {
    let mut player_transform = player.single_mut();

    for mut npc_transform in npc_query.iter_mut() {
        let collision = bevy::sprite::collide_aabb::collide(
            npc_transform.translation,
            Vec2::new(PLAYER_SIZE, PLAYER_SIZE),
            player_transform.translation,
            Vec2::new(PLAYER_SIZE, PLAYER_SIZE),
        );
        if collision == Some(bevy::sprite::collide_aabb::Collision::Right) {
            player_transform.translation.x = player_transform.translation.x - 5.;
            npc_transform.translation.x = npc_transform.translation.x + 5.;
        } else if collision == Some(bevy::sprite::collide_aabb::Collision::Left) {
            player_transform.translation.x = player_transform.translation.x + 5.;
            npc_transform.translation.x = npc_transform.translation.x - 5.;
        } else if collision == Some(bevy::sprite::collide_aabb::Collision::Top) {
            player_transform.translation.y = player_transform.translation.y - 5.;
            npc_transform.translation.y = npc_transform.translation.y + 5.;
        } else if collision == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
            player_transform.translation.y = player_transform.translation.y + 5.;
            npc_transform.translation.y = npc_transform.translation.y - 5.;
        }
    }
}

pub fn move_player_mult(
    input: Res<Input<KeyCode>>,
    mut player: Query<
        (&mut Transform, &mut PlayerVelocity),
        (With<Player>, Without<Face>, Without<Bat>),
    >,
    //    mut face: Query<&mut Transform, (With<Face>, Without<Player>, Without<Bat>)>,
    mut client_socket: ResMut<ClientSocket>,
    socket_address: Res<SocketAddress>,
    time: Res<Time>,
) {
    let (mut transform, mut velocity) = player.single_mut();
    //  let mut face_transform = face.single_mut();
    //let mut bat_transform = bat.single_mut();

    /////////////////////////////////////////////////////////////// with objects
    let recliner_size = Vec2::new(109., 184.);
    let recliner_translation = Vec3::new(-60., 210., 1.);
    let recliner = bevy::sprite::collide_aabb::collide(
        recliner_translation,
        recliner_size,
        transform.translation,
        Vec2::new(PLAYER_SIZE, PLAYER_SIZE),
    );

    let tv_size = Vec2::new(164., 103.);
    let tv_translation = Vec3::new(0., -245., 1.);
    let tv_stand = bevy::sprite::collide_aabb::collide(
        tv_translation,
        tv_size,
        transform.translation,
        Vec2::new(PLAYER_SIZE, PLAYER_SIZE),
    );

    let table_size = Vec2::new(103., 107.);
    let table_translation = Vec3::new(120., 170., 1.);
    let side_table = bevy::sprite::collide_aabb::collide(
        table_translation,
        table_size,
        transform.translation,
        Vec2::new(PLAYER_SIZE, PLAYER_SIZE),
    );

    /////////////////////////////////////////////////////////////////////

    let mut deltav = Vec2::splat(0.);
    if input.pressed(KeyCode::A) {
        deltav.x -= 1000.;
    }
    if input.pressed(KeyCode::D) {
        deltav.x += 1000.;
    }
    if input.pressed(KeyCode::W) {
        deltav.y += 1000.;
    }
    if input.pressed(KeyCode::S) {
        deltav.y -= 1000.;
    }

    let deltat = time.delta_seconds();

    // Calculate change in vector
    let acc = ACCEL_RATE * deltat;
    let rug_collision = bevy::sprite::collide_aabb::collide(
        Vec3::new(0., 0., 1.),
        Vec2::new(732.6, 507.6),
        transform.translation,
        Vec2::new(PLAYER_SIZE, PLAYER_SIZE),
    );
    if (rug_collision == Some(bevy::sprite::collide_aabb::Collision::Right))
        || (rug_collision == Some(bevy::sprite::collide_aabb::Collision::Left))
        || (rug_collision == Some(bevy::sprite::collide_aabb::Collision::Top))
        || (rug_collision == Some(bevy::sprite::collide_aabb::Collision::Bottom))
        || (rug_collision == Some(bevy::sprite::collide_aabb::Collision::Inside))
    {
        velocity.velocity = if deltav.length() > 0. {
            (velocity.velocity + (deltav.normalize_or_zero() * acc)).clamp_length_max(RUG_SPEED)
        } else if velocity.velocity.length() > acc {
            velocity.velocity + (velocity.velocity.normalize_or_zero() * -acc)
        } else {
            Vec2::splat(0.)
        };
    } else {
        velocity.velocity = if deltav.length() > 0. {
            (velocity.velocity + (deltav.normalize_or_zero() * acc)).clamp_length_max(PLAYER_SPEED)
        } else if velocity.velocity.length() > acc {
            velocity.velocity + (velocity.velocity.normalize_or_zero() * -acc)
        } else {
            Vec2::splat(0.)
        };
    }
    velocity.velocity = velocity.velocity * deltat;
    /////////////////////
    if recliner == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.velocity.x = -1. * 0.8;
    } else if recliner == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.velocity.x = 1. * 0.8;
    } else if recliner == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.velocity.y = -1. * 0.8;
    } else if recliner == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.velocity.y = 1. * 0.8;
    }

    if tv_stand == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.velocity.x = 1. * 0.9;
    } else if tv_stand == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.velocity.x = -1. * 0.9;
    } else if tv_stand == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.velocity.y = -1. * 0.9;
    } else if tv_stand == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.velocity.y = 1. * 0.9;
    }

    if side_table == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.velocity.x = 1. * 0.85;
    } else if side_table == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.velocity.x = -1. * 0.85;
    } else if side_table == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.velocity.y = -1. * 0.85;
    } else if side_table == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.velocity.y = 1. * 0.85;
    }
    ///////////////////////////

    // movement
    transform.translation.x = (transform.translation.x + velocity.velocity.x).clamp(
        -(1280. / 2.) + PLAYER_SIZE / 2.,
        1280. / 2. - PLAYER_SIZE / 2.,
    );
    transform.translation.y = (transform.translation.y + velocity.velocity.y).clamp(
        -(720. / 2.) + PLAYER_SIZE / 2.,
        720. / 2. - PLAYER_SIZE / 2.,
    );

    // face_transform.translation.x = transform.translation.x;
    // face_transform.translation.y = transform.translation.y;
    //bat_transform.translation.x = transform.translation.x - 5.;
    //bat_transform.translation.y = transform.translation.y;
    // Check if the position has changed before sending a message
    if transform.translation.x != velocity.prev_position.0
        || transform.translation.y != velocity.prev_position.1
    {
        // Check if the client socket is not yet initialized
        if client_socket.0.is_none() {
            // If not initialized, return early from the function
            return;
        }
        // Unwrap to get a mutable reference to the client socket
        let socket = client_socket.0.as_mut().unwrap();
        // Set the socket to non-blocking mode
        socket
            .set_nonblocking(true)
            .expect("Cannot set nonblocking");
        // Create player information
        let player_info = PlayerInfo {
            position: (transform.translation.x, transform.translation.y),
            health: 100,
            // Other fields go here
        };
        // Serialize the player information into a JSON-formatted string
        let message = serde_json::to_string(&player_info).expect("Failed to serialize");
        // Get the server address from the socket_address resource
        let server_address_str = &socket_address.0;
        // Send the serialized player information to the server
        socket
            .send_to(message.as_bytes(), server_address_str)
            .expect("Failed to send data.");
        let mut response = [0; 1024];
        match socket.recv_from(&mut response) {
            Ok((size, _peer)) => {
                let response_str = std::str::from_utf8(&response[0..size]).expect("Bad data.");
                //println!("Received response: {}", response_str);
            }
            Err(_e) => {
                //eprintln!("Error receiving data: {}", e);
            }
        }
        // Update the previous position
        velocity.prev_position = (transform.translation.x, transform.translation.y);
    }
}

pub fn collision_check_map1(
    npc_translation: Vec3,
    mut velocity: Vec2,
    player_translation: Vec3,
) -> Vec2 {
    let recliner_size = Vec2::new(109., 184.);
    let recliner_translation = Vec3::new(-60., 210., 1.);
    let recliner = bevy::sprite::collide_aabb::collide(
        recliner_translation,
        recliner_size,
        npc_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
    );

    let tv_size = Vec2::new(164., 103.);
    let tv_translation = Vec3::new(0., -250., 1.);
    let tv_stand = bevy::sprite::collide_aabb::collide(
        tv_translation,
        tv_size,
        npc_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
    );

    let table_size = Vec2::new(103., 107.);
    let table_translation = Vec3::new(120., 170., 1.);
    let side_table = bevy::sprite::collide_aabb::collide(
        table_translation,
        table_size,
        npc_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
    );

    let player_collision = bevy::sprite::collide_aabb::collide(
        player_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
        npc_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
    );

    if recliner == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.x = -1. * 0.8;
    } else if recliner == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.x = 1. * 0.8;
    } else if recliner == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.y = -1. * 0.8;
    } else if recliner == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.y = 1. * 0.8;
    }

    if tv_stand == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.x = 1. * 0.9;
    } else if tv_stand == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.x = -1. * 0.9;
    } else if tv_stand == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.y = -1. * 0.9;
    } else if tv_stand == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.y = 1. * 0.9;
    }

    if side_table == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.x = 1. * 0.85;
    } else if side_table == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.x = -1. * 0.85;
    } else if side_table == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.y = -1. * 0.85;
    } else if side_table == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.y = 1. * 0.85;
    }

    if player_collision == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.x = 1. * 0.85;
    } else if player_collision == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.x = -1. * 0.85;
    } else if player_collision == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.y = -1. * 0.85;
    } else if player_collision == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.y = 1. * 0.85;
    } else if player_collision == Some(bevy::sprite::collide_aabb::Collision::Inside) {
        velocity.x = -1. * 0.85;
        velocity.y = -1. * 0.85;
    }

    return velocity;
}

pub fn collision_check_map4(
    npc_translation: Vec3,
    mut velocity: Vec2,
    player_translation: Vec3,
) -> Vec2 {
    let coral_size = Vec2::new(150., 150.);
    let coral1 = bevy::sprite::collide_aabb::collide(
        Vec3::new(0., 180., 2.),
        coral_size,
        npc_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
    );

    let coral2 = bevy::sprite::collide_aabb::collide(
        Vec3::new(0., -180., 2.),
        coral_size,
        npc_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
    );

    let coral3 = bevy::sprite::collide_aabb::collide(
        Vec3::new(-320., 180., 2.),
        coral_size,
        npc_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
    );

    let coral4 = bevy::sprite::collide_aabb::collide(
        Vec3::new(-320., -180., 2.),
        coral_size,
        npc_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
    );

    let coral5 = bevy::sprite::collide_aabb::collide(
        Vec3::new(320., 180., 2.),
        coral_size,
        npc_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
    );

    let coral6 = bevy::sprite::collide_aabb::collide(
        Vec3::new(320., -180., 2.),
        coral_size,
        npc_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
    );

    let player_collision = bevy::sprite::collide_aabb::collide(
        player_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
        npc_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
    );

    if coral1 == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.x = 1. * 0.9;
    } else if coral1 == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.x = -1. * 0.9;
    } else if coral1 == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.y = -1. * 0.9;
    } else if coral1 == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.y = 1. * 0.9;
    }

    if coral2 == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.x = 1. * 0.9;
    } else if coral2 == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.x = -1. * 0.9;
    } else if coral2 == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.y = -1. * 0.9;
    } else if coral2 == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.y = 1. * 0.9;
    }

    if coral3 == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.x = 1. * 0.9;
    } else if coral3 == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.x = -1. * 0.9;
    } else if coral3 == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.y = -1. * 0.9;
    } else if coral3 == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.y = 1. * 0.9;
    }

    if coral4 == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.x = 1. * 0.9;
    } else if coral4 == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.x = -1. * 0.9;
    } else if coral4 == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.y = -1. * 0.9;
    } else if coral4 == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.y = 1. * 0.9;
    }

    if coral5 == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.x = 1. * 0.9;
    } else if coral5 == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.x = -1. * 0.9;
    } else if coral5 == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.y = -1. * 0.9;
    } else if coral5 == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.y = 1. * 0.9;
    }

    if coral6 == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.x = 1. * 0.9;
    } else if coral6 == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.x = -1. * 0.9;
    } else if coral6 == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.y = -1. * 0.9;
    } else if coral6 == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.y = 1. * 0.9;
    }

    if player_collision == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.x = 1. * 0.85;
    } else if player_collision == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.x = -1. * 0.85;
    } else if player_collision == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.y = -1. * 0.85;
    } else if player_collision == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.y = 1. * 0.85;
    } else if player_collision == Some(bevy::sprite::collide_aabb::Collision::Inside) {
        velocity.x = -1. * 0.85;
        velocity.y = -1. * 0.85;
    }
    return velocity;
}

pub fn collision_check_no_objects(
    npc_translation: Vec3,
    mut velocity: Vec2,
    player_translation: Vec3,
) -> Vec2 {
    let player_collision = bevy::sprite::collide_aabb::collide(
        player_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
        npc_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
    );

    if player_collision == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.x = 1. * 0.85;
    } else if player_collision == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.x = -1. * 0.85;
    } else if player_collision == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.y = -1. * 0.85;
    } else if player_collision == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.y = 1. * 0.85;
    } else if player_collision == Some(bevy::sprite::collide_aabb::Collision::Inside) {
        velocity.x = -1. * 0.85;
        velocity.y = -1. * 0.85;
    }

    return velocity;
}
