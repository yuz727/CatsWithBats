use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Bat;

#[derive(Component)]
pub struct Face;

#[derive(Component)]
pub struct Object;

#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct BallVelocity {
    pub velocity: Vec3,
}

#[derive(Component)]
pub struct Colliding {
    pub currently_colliding: bool,
}

impl Colliding{
    pub fn new() -> Self {
        Self {
            currently_colliding: false,
        }
    }
}
