use bevy::prelude::*;

use super::npc::{self, MovementTimer, NPCVelocity, NPC};
use crate::components::*;

const PLAYER_SIZE: f32 = 30.;
// 5px/frame @60Hz == 300px/s
const PLAYER_SPEED: f32 = 300.;
// 1px/frame^2 @60Hz == 3600px/s^2
const ACCEL_RATE: f32 = 3600.;

// Just go the the player straight
pub fn approach_player(
    mut npcs: Query<
        (&mut Transform, &mut MovementTimer, &mut NPCVelocity),
        (With<NPC>, Without<Player>),
    >,
    mut player: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let (mut npc_transform, mut _timer, mut velocity) = npcs.single_mut();
    let player_transform = player.single_mut();

    let mut deltav = Vec2::splat(0.);
    if npc_transform.translation.x < player_transform.translation.x {
        deltav.x += 10.;
    }
    if npc_transform.translation.x > player_transform.translation.x {
        deltav.x -= 10.;
    }
    if npc_transform.translation.y < player_transform.translation.y {
        deltav.y += 10.;
    }
    if npc_transform.translation.y > player_transform.translation.y {
        deltav.y -= 10.;
    }

    let deltat = time.delta_seconds();
    let acc = ACCEL_RATE * deltat;
    velocity.velocity = if deltav.length() > 0. {
        (velocity.velocity + (deltav.normalize_or_zero() * acc)).clamp_length_max(PLAYER_SPEED)
    } else if velocity.velocity.length() > acc {
        velocity.velocity + (velocity.velocity.normalize_or_zero() * -acc)
    } else {
        Vec2::splat(0.)
    };
    let change = velocity.velocity * deltat;

    npc_transform.translation.x = (npc_transform.translation.x + change.x).clamp(
        -(1280. / 2.) + PLAYER_SIZE / 2.,
        1280. / 2. - PLAYER_SIZE / 2.,
    );
    npc_transform.translation.y = (npc_transform.translation.y + change.y).clamp(
        -(720. / 2.) + PLAYER_SIZE / 2.,
        720. / 2. - PLAYER_SIZE / 2.,
    );
}

pub fn approach_ball(
    mut npcs: Query<
        (&mut Transform, &mut MovementTimer, &mut NPCVelocity),
        (With<NPC>, Without<Ball>, Without<Player>),
    >,
    mut ball: Query<&mut Transform, (With<Ball>, Without<NPC>, Without<Player>)>,
    time: Res<Time>,
) {
    let (mut npc_transform, mut _timer, mut velocity) = npcs.single_mut();
    let ball_transform = ball.single_mut();

    let mut deltav = Vec2::splat(0.);
    if npc_transform.translation.x < ball_transform.translation.x {
        deltav.x += 10.;
    }
    if npc_transform.translation.x > ball_transform.translation.x {
        deltav.x -= 10.;
    }
    if npc_transform.translation.y < ball_transform.translation.y {
        deltav.y += 10.;
    }
    if npc_transform.translation.y > ball_transform.translation.y {
        deltav.y -= 10.;
    }

    let deltat = time.delta_seconds();
    let acc = ACCEL_RATE * deltat;
    velocity.velocity = if deltav.length() > 0. {
        (velocity.velocity + (deltav.normalize_or_zero() * acc)).clamp_length_max(PLAYER_SPEED)
    } else if velocity.velocity.length() > acc {
        velocity.velocity + (velocity.velocity.normalize_or_zero() * -acc)
    } else {
        Vec2::splat(0.)
    };
    let change = velocity.velocity * deltat;

    npc_transform.translation.x = (npc_transform.translation.x + change.x).clamp(
        -(1280. / 2.) + PLAYER_SIZE / 2.,
        1280. / 2. - PLAYER_SIZE / 2.,
    );
    npc_transform.translation.y = (npc_transform.translation.y + change.y).clamp(
        -(720. / 2.) + PLAYER_SIZE / 2.,
        720. / 2. - PLAYER_SIZE / 2.,
    );
}
