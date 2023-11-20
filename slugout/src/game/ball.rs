use bevy::prelude::*;

use crate::{GameState, MultiplayerState};
//use bevy::window::CursorMoved;

use super::components::Aim;
use super::components::Ball;
use super::components::BallVelocity;
use super::components::Bat;
use super::components::Colliding;
use super::components::Density;
use super::components::Player;
use super::components::Rug;
use crate::game::components::Hitbox;

const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;
const BALL_SIZE: f32 = 10.;
//const HIT_POWER: Vec3 = Vec3::new(500.0, 500.0, 2.0);
const BASE_FRICTION: f32 = 0.4;
const G: f32 = 9.81;
const MIN_BALL_VELOCITY: f32 = 30.;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup);
        app.add_systems(OnEnter(MultiplayerState::Game), setup);
        app.add_systems(Update, bounce.run_if(in_state(GameState::Game)));
        app.add_systems(Update, bounce.run_if(in_state(MultiplayerState::Game)));
        app.add_systems(Update, swing.run_if(in_state(GameState::Game)));
        app.add_systems(Update, swing.run_if(in_state(MultiplayerState::Game)));
        app.add_systems(Update, friction.run_if(in_state(GameState::Game)));
        app.add_systems(Update, friction.run_if(in_state(MultiplayerState::Game)));
        app.add_systems(Update, bat_hitbox.run_if(in_state(GameState::Game)));
        app.add_systems(Update, bat_hitbox.run_if(in_state(MultiplayerState::Game)));
        app.add_systems(Update, aim_follows_cursor.run_if(in_state(GameState::Game)));
        app.add_systems(
            Update,
            aim_follows_cursor.run_if(in_state(MultiplayerState::Game)),
        );
    }
}

//ball Creation
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("yarnball.png"),
            transform: Transform::from_xyz(0., 0., 2.).with_scale(Vec3::new(0.025, 0.025, 0.)),
            ..Default::default()
        })
        .insert(Ball {
            radius: 0.025,
            elasticity: 0.95,
        })
        .insert(BallVelocity {
            velocity: Vec3::new(300.0, 300.0, 2.0),
        })
        .insert(Colliding::new())
        .insert(Density { density: 2. });

    // 2ND ball
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("yarnball.png"),
            transform: Transform::from_xyz(500., 5., 2.).with_scale(Vec3::new(0.028, 0.028, 0.)),
            ..Default::default()
        })
        .insert(Ball {
            radius: 0.0275,
            elasticity: 1.,
        })
        .insert(super::components::BallVelocity {
            velocity: Vec3::new(300.0, 100.0, 2.0),
        })
        .insert(Colliding::new())
        .insert(Density { density: 2. });

    //3RD ball
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("yarnball.png"),
            transform: Transform::from_xyz(-400., -100., 2.)
                .with_scale(Vec3::new(0.031, 0.031, 0.)),
            ..Default::default()
        })
        .insert(Ball {
            radius: 0.031,
            elasticity: 0.975,
        })
        .insert(BallVelocity {
            velocity: Vec3::new(-500., 3., 2.),
        })
        .insert(Colliding::new())
        .insert(Density { density: 2. });

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("yarnball.png"),
            transform: Transform::from_xyz(8., 6., 2.).with_scale(Vec3::new(0.034, 0.034, 0.)),
            ..Default::default()
        })
        .insert(Ball {
            radius: 0.034,
            elasticity: 0.9,
        })
        .insert(BallVelocity {
            velocity: Vec3::new(300.0, 300.0, 2.0),
        })
        .insert(Colliding::new())
        .insert(Density { density: 2. });

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("yarnball.png"),
            transform: Transform::from_xyz(20., 30., 2.).with_scale(Vec3::new(0.038, 0.038, 0.)),
            ..Default::default()
        })
        .insert(Ball {
            radius: 0.038,
            elasticity: 0.875,
        })
        .insert(BallVelocity {
            velocity: Vec3::new(300.0, 300.0, 2.0),
        })
        .insert(Colliding::new())
        .insert(Density { density: 2. });

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("yarnball.png"),
            transform: Transform::from_xyz(400., 400., 2.).with_scale(Vec3::new(0.042, 0.042, 0.)),
            ..Default::default()
        })
        .insert(Ball {
            radius: 0.042,
            elasticity: 0.85,
        })
        .insert(BallVelocity {
            velocity: Vec3::new(300.0, 300.0, 2.0),
        })
        .insert(Colliding::new())
        .insert(Density { density: 2. });

    //Spawn bat hitbox for bat
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(240., 140., 100., 0.),
                custom_size: Some(Vec2::new(45., 75.)),
                ..default()
            },
            transform: Transform::from_xyz(0., 0., 2.),
            ..Default::default()
        })
        .insert(Hitbox {
            size: Vec2::new(45., 75.), //30 52
        });
}

//bounce the ball
fn bounce(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut BallVelocity, &mut Ball), (With<Ball>, Without<Player>)>,
) {
    for (mut transform, mut ball_velocity, mut ball) in query.iter_mut() {
        let ball_radius = ball.radius * 300.;

        // Find the new translation for the x and y for the ball
        let mut new_translation_x = (transform.translation.x
            + (ball_velocity.velocity.x * time.delta_seconds()))
        .clamp(-(1280. / 2.) + ball_radius, 1280. / 2. - ball_radius);

        let mut new_translation_y = (transform.translation.y
            + (ball_velocity.velocity.y * time.delta_seconds()))
        .clamp(-(720. / 2.) + ball_radius, 720. / 2. - ball_radius);

        let new_translation = Vec3::new(
            new_translation_x,
            new_translation_y,
            transform.translation.z,
        );

        // Check for collision with player

        let recliner_size = Vec2::new(109., 184.);
        let recliner_translation = Vec3::new(-60., 210., 1.);
        let recliner = bevy::sprite::collide_aabb::collide(
            recliner_translation,
            recliner_size,
            new_translation,
            Vec2::new(ball_radius * 2., ball_radius * 2.),
        );

        let tv_size = Vec2::new(164., 103.);
        let tv_translation = Vec3::new(0., -250., 1.);
        let tv_stand = bevy::sprite::collide_aabb::collide(
            tv_translation,
            tv_size,
            new_translation,
            Vec2::new(ball_radius * 2., ball_radius * 2.),
        );

        let table_size = Vec2::new(103., 107.);
        let table_translation = Vec3::new(120., 170., 1.);
        let side_table = bevy::sprite::collide_aabb::collide(
            table_translation,
            table_size,
            new_translation,
            Vec2::new(ball_radius * 2., ball_radius * 2.),
        );

        //other collisions//////////////////////////////////////////////////////

        if recliner == Some(bevy::sprite::collide_aabb::Collision::Right) {
            ball_velocity.velocity.x = -ball_velocity.velocity.x * 0.8 * ball.elasticity;
            ball_velocity.velocity.y = ball_velocity.velocity.y * 0.8 * ball.elasticity;
            new_translation_x = recliner_translation.x - recliner_size.x / 2. - ball_radius;
        } else if recliner == Some(bevy::sprite::collide_aabb::Collision::Left) {
            ball_velocity.velocity.x = -ball_velocity.velocity.x * 0.8 * ball.elasticity;
            ball_velocity.velocity.y = ball_velocity.velocity.y * 0.8 * ball.elasticity;
            new_translation_x = recliner_translation.x + recliner_size.x / 2. + ball_radius;
        } else if recliner == Some(bevy::sprite::collide_aabb::Collision::Top) {
            ball_velocity.velocity.y = -ball_velocity.velocity.y * 0.8 * ball.elasticity;
            ball_velocity.velocity.x = ball_velocity.velocity.x * 0.8 * ball.elasticity;
            new_translation_y = recliner_translation.y - recliner_size.y / 2. - ball_radius;
        } else if recliner == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
            ball_velocity.velocity.y = -ball_velocity.velocity.y * 0.8 * ball.elasticity;
            ball_velocity.velocity.x = ball_velocity.velocity.x * 0.8 * ball.elasticity;
            new_translation_y = recliner_translation.y + recliner_size.y / 2. + ball_radius;
        }

        if tv_stand == Some(bevy::sprite::collide_aabb::Collision::Left) {
            ball_velocity.velocity.x = -ball_velocity.velocity.x * 0.9 * ball.elasticity;
            ball_velocity.velocity.y = ball_velocity.velocity.y * 0.9 * ball.elasticity;
            new_translation_x = tv_translation.x + tv_size.x / 2. + ball_radius;
        } else if tv_stand == Some(bevy::sprite::collide_aabb::Collision::Right) {
            ball_velocity.velocity.x = -ball_velocity.velocity.x * 0.9 * ball.elasticity;
            ball_velocity.velocity.y = ball_velocity.velocity.y * 0.9 * ball.elasticity;
            new_translation_x = tv_translation.x - tv_size.x / 2. - ball_radius;
        } else if tv_stand == Some(bevy::sprite::collide_aabb::Collision::Top) {
            ball_velocity.velocity.y = -ball_velocity.velocity.y * 0.9 * ball.elasticity;
            ball_velocity.velocity.x = ball_velocity.velocity.x * 0.9 * ball.elasticity;
            new_translation_y = tv_translation.y - tv_size.y / 2. - ball_radius;
        } else if tv_stand == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
            ball_velocity.velocity.y = -ball_velocity.velocity.y * 0.9 * ball.elasticity;
            ball_velocity.velocity.x = ball_velocity.velocity.x * 0.9 * ball.elasticity;
            new_translation_y = tv_translation.y + tv_size.y / 2. + ball_radius;
        }

        if side_table == Some(bevy::sprite::collide_aabb::Collision::Left) {
            ball_velocity.velocity.x = -ball_velocity.velocity.x * 0.85 * ball.elasticity;
            ball_velocity.velocity.y = ball_velocity.velocity.y * 0.85 * ball.elasticity;
            new_translation_x = table_translation.x + table_size.x / 2. + ball_radius;
        } else if side_table == Some(bevy::sprite::collide_aabb::Collision::Right) {
            ball_velocity.velocity.x = -ball_velocity.velocity.x * 0.85 * ball.elasticity;
            ball_velocity.velocity.y = ball_velocity.velocity.y * 0.85 * ball.elasticity;
            new_translation_x = table_translation.x - table_size.x / 2. - ball_radius;
        } else if side_table == Some(bevy::sprite::collide_aabb::Collision::Top) {
            ball_velocity.velocity.y = -ball_velocity.velocity.y * 0.85 * ball.elasticity;
            ball_velocity.velocity.x = ball_velocity.velocity.x * 0.85 * ball.elasticity;
            new_translation_y = table_translation.y - table_size.y / 2. - ball_radius;
        } else if side_table == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
            ball_velocity.velocity.y = -ball_velocity.velocity.y * 0.85 * ball.elasticity;
            ball_velocity.velocity.x = ball_velocity.velocity.x * 0.85 * ball.elasticity;
            new_translation_y = table_translation.y + table_size.y / 2. + ball_radius;
        }

        // Move ball
        transform.translation.x = new_translation_x;
        transform.translation.y = new_translation_y;

        // Bounce when hitting the screen edges
        if transform.translation.x.abs() == WIN_W / 2.0 - ball_radius {
            ball_velocity.velocity.x = -ball_velocity.velocity.x * ball.elasticity;
            ball_velocity.velocity.y = ball_velocity.velocity.y * ball.elasticity;
        }
        if transform.translation.y.abs() == WIN_H / 2.0 - ball_radius {
            ball_velocity.velocity.y = -ball_velocity.velocity.y * ball.elasticity;
            ball_velocity.velocity.x = ball_velocity.velocity.x * ball.elasticity;
        }
    }
}

fn bat_hitbox(
    mut hitbox: Query<&mut Sprite, (With<Hitbox>, Without<Bat>)>,
    bat: Query<&Transform, (With<Bat>, Without<Hitbox>)>,
    input_mouse: Res<Input<MouseButton>>,
) {
    let mut color_hitbox = hitbox.single_mut();

    if input_mouse.pressed(MouseButton::Left) {
        // Left button was pressed
        color_hitbox.color = Color::rgba(240., 140., 100., 0.2);
    } else {
        color_hitbox.color = Color::rgba(240., 140., 100., 0.);
    }
}

fn friction(
    mut query: Query<(&Transform, &mut BallVelocity, &Density, &Ball), With<Ball>>,
    rug: Query<(&Transform, &Rug), With<Rug>>,
    time: Res<Time>,
) {
    let (rug_transform, rug) = rug.single();
    let rug_size = Vec2::new(720., 500.);
    let deltat = time.delta_seconds();

    for (ball_transform, mut ball_velocity, ball_density, ball) in query.iter_mut() {
        // If the ball is on the rug, slow it down using the rugs coefficient of friction
        let rug_collision = bevy::sprite::collide_aabb::collide(
            rug_transform.translation,
            rug_size,
            ball_transform.translation,
            Vec2::new(BALL_SIZE, BALL_SIZE),
        );
        if (rug_collision == Some(bevy::sprite::collide_aabb::Collision::Right))
            || (rug_collision == Some(bevy::sprite::collide_aabb::Collision::Left))
            || (rug_collision == Some(bevy::sprite::collide_aabb::Collision::Top))
            || (rug_collision == Some(bevy::sprite::collide_aabb::Collision::Bottom))
            || (rug_collision == Some(bevy::sprite::collide_aabb::Collision::Inside))
        {
            let newvx;
            let newvy;

            //Caclulate the new ball velocity using v' = v - G * coefficient of friction * deltat
            if ball_velocity.velocity.x < 0. {
                newvx = ball_velocity.velocity.x + G * rug.friction * deltat;
                if newvx < (-1. * MIN_BALL_VELOCITY) {
                    ball_velocity.velocity.x = newvx;
                }
            } else {
                newvx = ball_velocity.velocity.x - G * rug.friction * deltat;
                if newvx > MIN_BALL_VELOCITY {
                    ball_velocity.velocity.x = newvx;
                }
            }

            if ball_velocity.velocity.y < 0. {
                newvy = ball_velocity.velocity.y + G * rug.friction * deltat;
                if newvx < (-1. * MIN_BALL_VELOCITY) {
                    ball_velocity.velocity.y = newvy;
                }
            } else {
                newvy = ball_velocity.velocity.y - G * rug.friction * deltat;
                if newvy > MIN_BALL_VELOCITY {
                    ball_velocity.velocity.y = newvy;
                }
            }

        // If the ball is not on the rug, slow it down using the floors coefficient of friction (BASE_FRICTION)
        } else {
            let newvx;
            let newvy;

            //Caclulate the new ball velocity using v' = v - G * coefficient of friction * deltat
            if ball_velocity.velocity.x < 0. {
                newvx = ball_velocity.velocity.x + G * BASE_FRICTION * deltat;
                if newvx < (-1. * MIN_BALL_VELOCITY) {
                    ball_velocity.velocity.x = newvx;
                }
            } else {
                newvx = ball_velocity.velocity.x - G * BASE_FRICTION * deltat;
                if newvx > MIN_BALL_VELOCITY {
                    ball_velocity.velocity.x = newvx;
                }
            }

            if ball_velocity.velocity.y < 0. {
                newvy = ball_velocity.velocity.y + G * BASE_FRICTION * deltat;
                if newvx < (-1. * MIN_BALL_VELOCITY) {
                    ball_velocity.velocity.y = newvy;
                }
            } else {
                newvy = ball_velocity.velocity.y - G * BASE_FRICTION * deltat;
                if newvy > MIN_BALL_VELOCITY {
                    ball_velocity.velocity.y = newvy;
                }
            }
        }
    }
}

/*
   bat swing function, now on RELEASE of mouse button (based on cursor)
       doesn't exactly move towards cursor bc it depends on the current ball velocity and position
   still hits both yarnballs
   (DOES NOT ACCOUNT FOR COLLISIONS)
*/

fn swing(
    //mut commands: Commands,
    input_mouse: Res<Input<MouseButton>>,
    mut query: Query<
        (&mut Ball, &mut BallVelocity, &mut Transform),
        (With<Ball>, Without<Hitbox>, Without<Bat>, Without<Player>),
    >,
    mut query_bat: Query<
        &mut Transform,
        (With<Bat>, Without<Hitbox>, Without<Ball>, Without<Player>),
    >,
    //cursor_events: ResMut<Events<CursorMoved>>,
    mut hitbox: Query<
        (&mut Transform, &mut Hitbox),
        (With<Hitbox>, Without<Ball>, Without<Ball>, Without<Player>),
    >,
    window: Query<&Window>,
    player: Query<&Transform, (With<Player>, Without<Hitbox>, Without<Bat>, Without<Ball>)>,
) {
    let (mut hitbox_transform, hitbox) = hitbox.single_mut();

    static mut MOUSE_BUTTON_PRESSED: bool = false;
    static mut BAT_TRANSFORMED: bool = false;
    static mut MOUSE_BUTTON_JUST_RELEASED: bool = false;
    //let mut mouse_position: Vec2;
    let mut bat_transform = query_bat.single_mut();
    let player_transform = player.single();

    if input_mouse.just_pressed(MouseButton::Left) {
        // Mouse button was just pressed
        unsafe {
            MOUSE_BUTTON_PRESSED = true;
            BAT_TRANSFORMED = false;
            MOUSE_BUTTON_JUST_RELEASED = false;
        }
        //println!("Mouse button pressed");
    } else if input_mouse.just_released(MouseButton::Left) {
        // Mouse button was just released
        unsafe {
            if MOUSE_BUTTON_PRESSED {
                MOUSE_BUTTON_PRESSED = false;
                BAT_TRANSFORMED = true;
                MOUSE_BUTTON_JUST_RELEASED = true;
                //println!("Mouse button released");
            }
        }
    }

    /*let mut cursor_event_reader = cursor_events.get_reader();
    for event in cursor_event_reader.iter(&cursor_events) {
        // Update the mouse position
        mouse_position = event.position;
        //println!("Mouse position changed");
    }*/

    //for (bat, mut bat_transform) in query_bat.iter_mut() {
    if unsafe { MOUSE_BUTTON_PRESSED } {
        // Left mouse button is pressed, set the bat to horizontal
        bat_transform.scale.y = -0.175;
    } else if unsafe { BAT_TRANSFORMED } {
        bat_transform.scale.y = 0.175;
    }
    //}

    if let Some(mouse_position) = window.single().physical_cursor_position() {
        //println!("Cursor is inside window {:?}", mouse_position);
        //if unsafe { MOUSE_BUTTON_JUST_RELEASED } {
        if ((mouse_position.x - WIN_W) / 2.) > player_transform.translation.x {
            bat_transform.translation = player_transform.translation;
            bat_transform.translation.x = bat_transform.translation.x + 8.;
            bat_transform.scale.x = -0.175;

            hitbox_transform.translation = bat_transform.translation;
            hitbox_transform.translation.x = hitbox_transform.translation.x + 20.;
            hitbox_transform.translation.y = hitbox_transform.translation.y - 5.;
        } else {
            bat_transform.translation = player_transform.translation;
            bat_transform.translation.x = bat_transform.translation.x - 5.;
            bat_transform.scale.x = 0.175;

            hitbox_transform.translation = bat_transform.translation;
            hitbox_transform.translation.x = hitbox_transform.translation.x - 20.;
            hitbox_transform.translation.y = hitbox_transform.translation.y - 5.;
        }
        if unsafe { MOUSE_BUTTON_JUST_RELEASED } {
            for (mut ball, mut ball_velocity, mut ball_transform) in query.iter_mut() {
                let bat_to_ball_collision = bevy::sprite::collide_aabb::collide(
                    hitbox_transform.translation,
                    hitbox.size,
                    ball_transform.translation,
                    Vec2::new(BALL_SIZE, BALL_SIZE),
                );

                if (bat_to_ball_collision == Some(bevy::sprite::collide_aabb::Collision::Right))
                    || (bat_to_ball_collision == Some(bevy::sprite::collide_aabb::Collision::Left))
                    || (bat_to_ball_collision == Some(bevy::sprite::collide_aabb::Collision::Top))
                    || (bat_to_ball_collision
                        == Some(bevy::sprite::collide_aabb::Collision::Bottom))
                    || (bat_to_ball_collision
                        == Some(bevy::sprite::collide_aabb::Collision::Inside))
                {
                    ball_velocity.velocity = Vec3::splat(0.);
                    let change_x =
                        (((mouse_position.x - WIN_W) / 2.) - ball_transform.translation.x).abs();
                    let change_y =
                        ((-(mouse_position.y - WIN_H) / 2.) - ball_transform.translation.y).abs();
                    let mut new_velocity = Vec3::new(change_x, change_y, 0.);
                    new_velocity = new_velocity.normalize_or_zero();

                    if ((mouse_position.x - WIN_W) / 2.) > ball_transform.translation.x {
                        new_velocity.x = new_velocity.x;
                    } else {
                        new_velocity.x = -1. * new_velocity.x;
                    }

                    if (-(mouse_position.y - WIN_H) / 2.) > ball_transform.translation.y {
                        new_velocity.y = new_velocity.y;
                    } else {
                        new_velocity.y = -1. * new_velocity.y;
                    }

                    new_velocity.x = new_velocity.x * 500.;
                    new_velocity.y = new_velocity.y * 500.;
                    ball_velocity.velocity = new_velocity * ball.elasticity;
                }

                // let ball_position = ball_velocity.velocity.truncate();
                // println!("Ball position: {:?}", ball_position);

                /*let direction =  MOUSE_POSITION - ball_velocity.velocity.truncate();;
                println!("Direction: {:?}", direction);


                // Normalize the direction and set the ball's velocity
                let normalized_direction = direction.normalize_or_zero();
                //println!("Normalized direction: {:?}", normalized_direction);

                ball_velocity.velocity = Vec3::new(
                    normalized_direction.x * HIT_POWER.x,
                    normalized_direction.y * HIT_POWER.y,
                    0.0,
                );
                println!("Ball velocity: {:?}", ball_velocity.velocity);*/
            }

            // Reset the flags for the next interaction
            unsafe {
                MOUSE_BUTTON_JUST_RELEASED = false;
                BAT_TRANSFORMED = false;
            }
        }
    }
}

fn aim_follows_cursor(
    mut query_aim: Query<&mut Transform, With<Aim>>,
    //cursor_events: Res<Events<CursorMoved>>,
    window: Query<&Window>,
) {
    let mut aim_transform = query_aim.single_mut();
    /*let mut cursor_event_reader = cursor_events.get_reader();

    for event in cursor_event_reader.iter(&cursor_events) {
        // Update the aim's position to follow the cursor
        for mut aim_transform in query_aim.iter_mut() {
            aim_transform.translation.x = event.position.x - WIN_W / 2.0;
            aim_transform.translation.y = -(event.position.y - WIN_H / 2.0 + 40.0);
        }
    }*/

    if let Some(mouse_position) = window.single().physical_cursor_position() {
        aim_transform.translation.x = (mouse_position.x - WIN_W) / 2.;
        aim_transform.translation.y = -(mouse_position.y - WIN_H) / 2. - 40.;
    }
}
