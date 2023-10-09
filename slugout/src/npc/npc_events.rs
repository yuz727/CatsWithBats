use bevy::prelude::*;
use rand::prelude::*;
use std::time::Duration;

use super::npc::{MovementTimer, NPCVelocity, NPC};
use crate::components::*;

const PLAYER_SIZE: f32 = 30.;
// 5px/frame @60Hz == 300px/s
const PLAYER_SPEED: f32 = 300.;
// 1px/frame^2 @60Hz == 3600px/s^2
const ACCEL_RATE: f32 = 3600.;

pub fn rand_movement(
    mut npcs: Query<
        (&mut Transform, &mut MovementTimer, &mut NPCVelocity),
        (With<NPC>, Without<Player>),
    >,
    mut player: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let (mut npc_transform, mut timer, mut velocity) = npcs.single_mut();
    let player_transform = player.single_mut();
    //let mut rng = thread_rng();
    // let distance = Vec3::new(
    //     npc_transform.translation.x - player_transform.translation.x,
    //     npc_transform.translation.y - player_transform.translation.y,
    //     npc_transform.translation.z - player_transform.translation.z,
    // );
    let distance = Vec3::project_onto(npc_transform.translation, player_transform.translation);
    timer.tick(time.delta());
    //if timer.just_finished() {
    // The duration of before next movement change
    //timer.set_duration(Duration::from_secs(rng.gen_range(0..5)));

    // Decide change in velocity based on rng
    let mut deltav = Vec2::splat(0.);
    // let result_x = rng.gen_range(-10..10);
    // let result_y = rng.gen_range(-10..10);
    if distance.x < 0. {
        deltav.x += -10.;
    }
    if distance.x > 0. {
        deltav.x += 10.;
    }
    if distance.y < 0. {
        deltav.y += -10.;
    }
    if distance.y > 0. {
        deltav.y += 10.;
    }

    let deltat = time.delta_seconds();

    // for debugging
    info!(distance.x);
    info!(distance.y);

    // Calculate change in vector
    let acc = ACCEL_RATE * deltat;
    velocity.velocity = if deltav.length() > 0. {
        (velocity.velocity + (deltav.normalize_or_zero() * acc)).clamp_length_max(PLAYER_SPEED)
    } else if velocity.velocity.length() > acc {
        velocity.velocity + (velocity.velocity.normalize_or_zero() * -acc)
    } else {
        Vec2::splat(0.)
    };
    //velocity.velocity = velocity.velocity * deltat;
    let change = velocity.velocity * deltat;
    velocity.velocity = change;
    //}
    // movement
    npc_transform.translation.x = (npc_transform.translation.x + velocity.velocity.x).clamp(
        -(1280. / 2.) + PLAYER_SIZE / 2.,
        1280. / 2. - PLAYER_SIZE / 2.,
    );
    npc_transform.translation.y = (npc_transform.translation.y + velocity.velocity.y).clamp(
        -(720. / 2.) + PLAYER_SIZE / 2.,
        720. / 2. - PLAYER_SIZE / 2.,
    );
}
