use crate::components::*;
use bevy::prelude::*;

const PLAYER_SIZE: f32 = 30.;
// 5px/frame @60Hz == 300px/s
const PLAYER_SPEED: f32 = 300.;
// 1px/frame^2 @60Hz == 3600px/s^2
const ACCEL_RATE: f32 = 14400.;

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

    let mut deltav = Vec2::splat(0.);
    if input.pressed(KeyCode::A) {
        deltav.x -= 200.;
    }
    if input.pressed(KeyCode::D) {
        deltav.x += 200.;
    }
    if input.pressed(KeyCode::W) {
        deltav.y += 200.;
    }
    if input.pressed(KeyCode::S) {
        deltav.y -= 200.;
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
