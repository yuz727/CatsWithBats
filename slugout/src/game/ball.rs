use bevy::prelude::*;
use crate::GameState;

use crate::game::components::*;

use super::components::BallVelocity;
use super::components::Ball;
use super::components::Bat;
use super::components::Colliding;
use super::components::Density;
use super::components::Player;
use super::components::Rug;

const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;
const BALL_SIZE: f32 = 10.;
const HIT_POWER: Vec3 = Vec3::new(500.0, 500.0, 2.0);
const BASE_FRICTION: f32 = 0.4;
const G: f32 = 9.81;
const MIN_BALL_VELOCITY: f32 = 30.;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, bounce.run_if(in_state(GameState::Game)));
        app.add_systems(Update, swing.run_if(in_state(GameState::Game)));
        app.add_systems(Update, friction.run_if(in_state(GameState::Game)));
    }
}

//ball Creation
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("yarnball.png"),
        transform: Transform::from_xyz(0., 0., 2.).with_scale(Vec3::splat(0.03)),
        ..Default::default()
    })
    .insert(Ball {
        radius: 0.03 / 2.0,
    })
    .insert(BallVelocity {
        velocity: Vec3::new(300.0, 300.0, 2.0),
    })
    .insert(Colliding::new())
    .insert(Density {
        density: 2.,
    });


    // 2ND ball
     commands.spawn(SpriteBundle {
        texture: asset_server.load("yarnball.png"),
        transform: Transform::from_xyz(500., 5., 2.).with_scale(Vec3::splat(0.03)),
        ..Default::default()
    })
    .insert(Ball {
        radius: 0.03 / 2.0,
    })
    .insert(crate::game::components::BallVelocity {
        velocity: Vec3::new(300.0, 100.0, 2.0),
    })
    .insert(Colliding::new())
    .insert(Density {
        density: 2.,
    });

    //3RD ball
    commands.spawn(SpriteBundle {
        texture: asset_server.load("yarnball.png"),
        transform: Transform::from_xyz(-400., -100., 2.).with_scale(Vec3::splat(0.03)),
        ..Default::default()
    })
    .insert(Ball {
        radius: 0.03 / 2.0,
    })
    .insert(BallVelocity {
        velocity: Vec3::new(-500., 3., 2.),
    })
    .insert(Colliding::new())
    .insert(Density {
        density: 2.,
    });
}

//bounce the ball
fn bounce(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut BallVelocity), (With<Ball>, Without<Player>)>,
) {
 
    for (mut transform, mut ball) in query.iter_mut() {

        // Find the new translation for the x and y for the ball
        let mut new_translation_x = (transform.translation.x + (ball.velocity.x * time.delta_seconds())).clamp(
            -(1280. / 2.) + BALL_SIZE / 2.,
            1280. / 2. - BALL_SIZE / 2.,
        );

        let mut new_translation_y = (transform.translation.y + (ball.velocity.y * time.delta_seconds())).clamp(
            -(720. / 2.) + BALL_SIZE / 2.,
            720. / 2. - BALL_SIZE / 2.,
        );

        let new_translation = Vec3::new(new_translation_x, new_translation_y, transform.translation.z);

        // Check for collision with player

        let recliner_size = Vec2::new(109., 184.);
        let recliner_translation = Vec3::new(-60., 210., 1.);
        let recliner = bevy::sprite::collide_aabb::collide(recliner_translation, 
        recliner_size, new_translation, Vec2::new(BALL_SIZE, BALL_SIZE));

        let tv_size = Vec2::new(164., 103.);
        let tv_translation = Vec3::new(0., -250., 1.);
        let tv_stand = bevy::sprite::collide_aabb::collide(tv_translation, 
        tv_size, new_translation, Vec2::new(BALL_SIZE, BALL_SIZE));

        let table_size = Vec2::new(103.,107.);
        let table_translation = Vec3::new(120., 170., 1.);
        let side_table = bevy::sprite::collide_aabb::collide(table_translation, table_size, 
            new_translation, Vec2::new(BALL_SIZE, BALL_SIZE));


        //other collisions//////////////////////////////////////////////////////
 
        if recliner == Some(bevy::sprite::collide_aabb::Collision::Right){
            ball.velocity.x = -ball.velocity.x;
            new_translation_x = recliner_translation.x - recliner_size.x/2. - BALL_SIZE/2.;
        }else if recliner == Some(bevy::sprite::collide_aabb::Collision::Left){
            ball.velocity.x = -ball.velocity.x;
            new_translation_x = recliner_translation.x + recliner_size.x/2. + BALL_SIZE/2.;
        }else if recliner == Some(bevy::sprite::collide_aabb::Collision::Top){
            ball.velocity.y = -ball.velocity.y;
            new_translation_y = recliner_translation.y - recliner_size.y/2. - BALL_SIZE/2.;
        }else if recliner == Some(bevy::sprite::collide_aabb::Collision::Bottom){
                ball.velocity.y = -ball.velocity.y;
                new_translation_y = recliner_translation.y + recliner_size.y/2. + BALL_SIZE/2.;
        }

        if tv_stand == Some(bevy::sprite::collide_aabb::Collision::Left){
            ball.velocity.x = -ball.velocity.x;
            new_translation_x = tv_translation.x + tv_size.x/2. + BALL_SIZE/2.;
        }else if tv_stand == Some(bevy::sprite::collide_aabb::Collision::Right){
                ball.velocity.x = -ball.velocity.x;
                new_translation_x = tv_translation.x - tv_size.x/2. - BALL_SIZE/2.;
        }else if tv_stand == Some(bevy::sprite::collide_aabb::Collision::Top){
            ball.velocity.y = -ball.velocity.y;
            new_translation_y = tv_translation.y - tv_size.y/2. - BALL_SIZE/2.;
        }else if tv_stand == Some(bevy::sprite::collide_aabb::Collision::Bottom){
            ball.velocity.y = -ball.velocity.y;
            new_translation_y = tv_translation.y + tv_size.y/2. + BALL_SIZE/2.;

        }

        if side_table == Some(bevy::sprite::collide_aabb::Collision::Left){
            ball.velocity.x = -ball.velocity.x;
            new_translation_x = table_translation.x + table_size.x/2. + BALL_SIZE/2.;
        }else if side_table == Some(bevy::sprite::collide_aabb::Collision::Right) {
            ball.velocity.x = -ball.velocity.x;
            new_translation_x = table_translation.x - table_size.x/2. - BALL_SIZE/2.;
        }else if side_table == Some(bevy::sprite::collide_aabb::Collision::Top){
            ball.velocity.y = -ball.velocity.y;
            new_translation_y = table_translation.y - table_size.y/2. - BALL_SIZE/2.;
        }else if side_table == Some(bevy::sprite::collide_aabb::Collision::Bottom){
            ball.velocity.y = -ball.velocity.y;
            new_translation_y = table_translation.y + table_size.y/2. + BALL_SIZE/2.;
        }

        // Move ball
        transform.translation.x = new_translation_x;
        transform.translation.y = new_translation_y;

        // Bounce when hitting the screen edges
        if transform.translation.x.abs() == WIN_W / 2.0 - BALL_SIZE / 2. {
            ball.velocity.x = -ball.velocity.x;
        }
        if transform.translation.y.abs() == WIN_H / 2.0 - BALL_SIZE / 2.{
            ball.velocity.y = -ball.velocity.y;
        }
    }
}

fn friction(
    mut query: Query<(&Transform, &mut BallVelocity, &Density, &Ball), With<Ball>>,
    rug: Query<(&Transform, &Rug), With<Rug>>,
    time: Res<Time>,
){
    let (rug_transform, rug) = rug.single();
    let rug_size = Vec2::new(720., 500.);
    let deltat = time.delta_seconds();

    for (ball_transform, mut ball_velocity, ball_density, ball) in query.iter_mut(){
        // If the ball is on the rug, slow it down using the rugs coefficient of friction
        let rug_collision = bevy::sprite::collide_aabb::collide(rug_transform.translation, rug_size, ball_transform.translation, Vec2::new(BALL_SIZE, BALL_SIZE));
        if (rug_collision == Some(bevy::sprite::collide_aabb::Collision::Right)) || (rug_collision == Some(bevy::sprite::collide_aabb::Collision::Left)) || (rug_collision == Some(bevy::sprite::collide_aabb::Collision::Top)) || (rug_collision == Some(bevy::sprite::collide_aabb::Collision::Bottom)) || (rug_collision == Some(bevy::sprite::collide_aabb::Collision::Inside)){
    
            let mut newvx = 0.;
            let mut newvy = 0.;

            //Caclulate the new ball velocity using v' = v - G * coefficient of friction * deltat
            if ball_velocity.velocity.x < 0.{
                newvx = ball_velocity.velocity.x + G * rug.friction * deltat;
                if newvx < (-1. * MIN_BALL_VELOCITY) {
                    ball_velocity.velocity.x = newvx;
                }
            }else{
                newvx = ball_velocity.velocity.x - G * rug.friction * deltat;
                if newvx > MIN_BALL_VELOCITY {
                    ball_velocity.velocity.x = newvx;
                }
            }

            if ball_velocity.velocity.y < 0.{
                newvy = ball_velocity.velocity.y + G * rug.friction * deltat;
                if newvx < (-1. * MIN_BALL_VELOCITY) {
                    ball_velocity.velocity.y = newvy;
                }
            }else{
                newvy = ball_velocity.velocity.y - G * rug.friction * deltat;
                if newvy > MIN_BALL_VELOCITY {
                    ball_velocity.velocity.y = newvy;
                }
            }


        // If the ball is not on the rug, slow it down using the floors coefficient of friction (BASE_FRICTION)
        }else{

            let mut newvx = 0.;
            let mut newvy = 0.;

            //Caclulate the new ball velocity using v' = v - G * coefficient of friction * deltat
            if ball_velocity.velocity.x < 0.{
                newvx = ball_velocity.velocity.x + G * BASE_FRICTION * deltat;
                if newvx < (-1. * MIN_BALL_VELOCITY) {
                    ball_velocity.velocity.x = newvx;
                }
            }else{
                newvx = ball_velocity.velocity.x - G * BASE_FRICTION * deltat;
                if newvx > MIN_BALL_VELOCITY {
                    ball_velocity.velocity.x = newvx;
                }
            }

            if ball_velocity.velocity.y < 0.{
                newvy = ball_velocity.velocity.y + G * BASE_FRICTION * deltat;
                if newvx < (-1. * MIN_BALL_VELOCITY) {
                    ball_velocity.velocity.y = newvy;
                }
            }else{
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
    mut commands: Commands,
    input_mouse: Res<Input<MouseButton>>,
    mut query: Query<(&mut Ball, &mut BallVelocity)>,
    mut query_bat: Query<(&Bat, &mut Transform)>,
    cursor_events: ResMut<Events<CursorMoved>>,
) {
    static mut MOUSE_BUTTON_PRESSED: bool = false;
    static mut BAT_TRANSFORMED: bool = false;
    static mut MOUSE_BUTTON_JUST_RELEASED: bool = false;
    let mut MOUSE_POSITION: Vec2 = Vec2::default();

    if input_mouse.just_pressed(MouseButton::Left) {
        // Mouse button was just pressed
        unsafe {
            MOUSE_BUTTON_PRESSED = true;
            BAT_TRANSFORMED = false;
            MOUSE_BUTTON_JUST_RELEASED = false;
            println!("Mouse button pressed");
        }
    } else if input_mouse.just_released(MouseButton::Left) {
        // Mouse button was just released
        unsafe {
            if MOUSE_BUTTON_PRESSED {
                MOUSE_BUTTON_PRESSED = false;
                BAT_TRANSFORMED = true;
                MOUSE_BUTTON_JUST_RELEASED = true;
                println!("Mouse button released");
            }
        }
    }

    let mut cursor_event_reader = cursor_events.get_reader();
    for event in cursor_event_reader.iter(&cursor_events) {
        // Update the mouse position
        MOUSE_POSITION = event.position;
        //println!("Mouse position changed");
    }

    for (bat, mut bat_transform) in query_bat.iter_mut() {
        if unsafe { MOUSE_BUTTON_PRESSED } {
            // Left mouse button is pressed, set the bat to horizontal
            bat_transform.scale.y = -0.13;
        } else if unsafe { BAT_TRANSFORMED } {
            bat_transform.scale.y = 0.13;
        }
    }

    if unsafe { MOUSE_BUTTON_JUST_RELEASED } {
        for (mut ball, mut ball_velocity) in query.iter_mut() {
            // let ball_position = ball_velocity.velocity.truncate();
            // println!("Ball position: {:?}", ball_position);
            
            let direction =  MOUSE_POSITION - ball_velocity.velocity.truncate();;
            println!("Direction: {:?}", direction);


            // Normalize the direction and set the ball's velocity
            let normalized_direction = direction.normalize_or_zero();
            println!("Normalized direction: {:?}", normalized_direction);

            ball_velocity.velocity = Vec3::new(
                normalized_direction.x * HIT_POWER.x,
                normalized_direction.y * HIT_POWER.y,
                0.0,
            );
            println!("Ball velocity: {:?}", ball_velocity.velocity);

        }

        // Reset the flags for the next interaction
        unsafe {
            MOUSE_BUTTON_JUST_RELEASED = false;
            BAT_TRANSFORMED = false;
        }
    }
}
