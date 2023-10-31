// use crate::components::*;
use bevy::prelude::*;

use super::components::{Player, Face, Bat};

const PLAYER_SIZE: f32 = 30.;
// 5px/frame @60Hz == 300px/s
const PLAYER_SPEED: f32 = 300.;
// 1px/frame^2 @60Hz == 3600px/s^2
const ACCEL_RATE: f32 = 18000.;

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

pub fn move_player(
    input: Res<Input<KeyCode>>,
    mut player: Query<
        (&mut Transform, &mut PlayerVelocity),
        (With<Player>, Without<Face>, Without<Bat>),
    >,
    mut face: Query<&mut Transform, (With<Face>, Without<Player>, Without<Bat>)>,
    mut bat: Query<&mut Transform, (With<Bat>, Without<Player>, Without<Face>)>,
    time: Res<Time>,
) {
    let (mut transform, mut velocity) = player.single_mut();
    let mut face_transform = face.single_mut();
    let mut bat_transform = bat.single_mut();

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
    velocity.velocity = if deltav.length() > 0. {
        (velocity.velocity + (deltav.normalize_or_zero() * acc)).clamp_length_max(PLAYER_SPEED)
    } else if velocity.velocity.length() > acc {
        velocity.velocity + (velocity.velocity.normalize_or_zero() * -acc)
    } else {
        Vec2::splat(0.)
    };
    velocity.velocity = velocity.velocity * deltat;
    /////////////////////
    if recliner == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.velocity.x = -1.;
    } else if recliner == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.velocity.x = 1.;
    } else if recliner == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.velocity.y = -1.;
    } else if recliner == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.velocity.y = 1.;
    }

    if tv_stand == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.velocity.x = 1.;
    } else if tv_stand == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.velocity.x = -1.;
    } else if tv_stand == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.velocity.y = -1.;
    } else if tv_stand == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.velocity.y = 1.;
    }

    if side_table == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.velocity.x = 1.;
    } else if side_table == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.velocity.x = -1.;
    } else if side_table == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.velocity.y = -1.;
    } else if side_table == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.velocity.y = 1.;
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

    face_transform.translation.x = transform.translation.x;
    face_transform.translation.y = transform.translation.y;
    bat_transform.translation.x = transform.translation.x - 5.;
    bat_transform.translation.y = transform.translation.y;
}
