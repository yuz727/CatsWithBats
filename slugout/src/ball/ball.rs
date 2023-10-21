use crate::components::*;
use bevy::prelude::*;

const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;
const BALL_SIZE: f32 = 10.;
const PLAYER_SIZE: f32 = 30.;
const HIT_POWER: Vec3 = Vec3::new(500.0, 500.0, 2.0);

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, bounce);
        app.add_systems(Update, swing);
    }
}

//ball Creation
fn setup(mut commands: Commands) {

    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(0.0, 0.0, 2.0).with_scale(Vec3::new(10.0, 10.0,2.0)),
        ..default()
    }) .insert(Ball) .insert(crate::components::BallVelocity {
        velocity: Vec3::new(300.0, 300.0, 2.0),
    }).insert(Colliding::new());
}

//bounce the ball
fn bounce(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Colliding, &mut crate::components::BallVelocity), (With<Ball>, Without<Player>)>,
    mut player_colliding: Query<&mut Colliding, (With<Player>, Without<Ball>)>,
    player_transform: Query<&Transform, (With<Player>, Without<Ball>)>,
) {
 
    for (mut transform, mut colliding, mut ball) in query.iter_mut() {

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

        let recliner_size = Vec2::new(120., 130.);
        let recliner_translation = Vec3::new(-60., 210., 1.);
        let recliner = bevy::sprite::collide_aabb::collide(recliner_translation, 
        recliner_size, new_translation, Vec2::new(BALL_SIZE, BALL_SIZE));

        let tv_size = Vec2::new(170., 100.);
        let tv_translation = Vec3::new(0., -250., 1.);
        let tv_stand = bevy::sprite::collide_aabb::collide(tv_translation, 
        tv_size, new_translation, Vec2::new(BALL_SIZE, BALL_SIZE));

        let table_size = Vec2::new(113.,113.);
        let table_translation = Vec3::new(120., 170., 1.);
        let side_table = bevy::sprite::collide_aabb::collide(table_translation, table_size, new_translation, Vec2::new(BALL_SIZE, BALL_SIZE));


        //other collisions//////////////////////////////////////////////////////
 
        if recliner == Some(bevy::sprite::collide_aabb::Collision::Right){
            ball.velocity.x = -ball.velocity.x;
            new_translation_x = recliner_translation.x - recliner_size.x/2. - 1.;
        }else if recliner == Some(bevy::sprite::collide_aabb::Collision::Left){
            ball.velocity.x = -ball.velocity.x;
            new_translation_x = recliner_translation.x + recliner_size.x/2. + 1.;
        }else if recliner == Some(bevy::sprite::collide_aabb::Collision::Top){
            ball.velocity.y = -ball.velocity.y;
            new_translation_y = recliner_translation.y - recliner_size.y/2. - 1.;
        }else if recliner == Some(bevy::sprite::collide_aabb::Collision::Bottom){
                ball.velocity.y = -ball.velocity.y;
                new_translation_y = recliner_translation.y + recliner_size.y/2. + 1.;
        }else if recliner == Some(bevy::sprite::collide_aabb::Collision::Inside){
             print!("INSIDE");
         }

        if tv_stand == Some(bevy::sprite::collide_aabb::Collision::Left){
            ball.velocity.x = -ball.velocity.x;
            //info!("444444444444444");
            new_translation_x = tv_translation.x + tv_size.x/2. + 1.;
        }else if tv_stand == Some(bevy::sprite::collide_aabb::Collision::Right){
                ball.velocity.x = -ball.velocity.x;
                new_translation_x = tv_translation.x - tv_size.x/2. - 1.;
        }else if tv_stand == Some(bevy::sprite::collide_aabb::Collision::Top){
            ball.velocity.y = -ball.velocity.y;
            new_translation_y = tv_translation.y - tv_size.y/2. - 1.;
        }else if tv_stand == Some(bevy::sprite::collide_aabb::Collision::Bottom){
            ball.velocity.y = -ball.velocity.y;
            new_translation_y = tv_translation.y + tv_size.y/2. + 1.;

        }

        if side_table == Some(bevy::sprite::collide_aabb::Collision::Left){
            ball.velocity.x = -ball.velocity.x;
            new_translation_x = table_translation.x + table_size.x/2. + 1.;
        }else if side_table == Some(bevy::sprite::collide_aabb::Collision::Right) {
            ball.velocity.x = -ball.velocity.x;
            new_translation_x = table_translation.x - table_size.x/2. - 1.;
        }else if side_table == Some(bevy::sprite::collide_aabb::Collision::Top){
            ball.velocity.y = -ball.velocity.y;
            new_translation_y = table_translation.y - table_size.y/2. - 1.;
        }else if side_table == Some(bevy::sprite::collide_aabb::Collision::Bottom){
            ball.velocity.y = -ball.velocity.y;
            new_translation_y = table_translation.y + table_size.y/2. + 1.;
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

// bat swing function (hits ball no matter where player is, as long as mouse is clicked)
fn swing(
    mut commands: Commands,
    input_state: Res<Input<KeyCode>>,
    input_mouse: Res<Input<MouseButton>>,
    mut query: Query<(&mut Ball, &mut BallVelocity)>,
    mut query_bat: Query<(&Bat, &mut Transform)>,
) {
    static mut MOUSE_BUTTON_PRESSED: bool = false;
    static mut BAT_TRANSFORMED: bool = false;

    if input_mouse.just_pressed(MouseButton::Left) {
        // Mouse button was just pressed
        unsafe {
            MOUSE_BUTTON_PRESSED = true;
            BAT_TRANSFORMED = false;

        }
    } else if input_mouse.just_released(MouseButton::Left) {
        // Mouse button was just released
        unsafe {
            MOUSE_BUTTON_PRESSED = false;
            BAT_TRANSFORMED = true;
        }
    }

    // Animation for swinging the bat
    for (bat, mut bat_transform) in query_bat.iter_mut() {
        if unsafe { MOUSE_BUTTON_PRESSED } {
        // Left mouse button is pressed, set the bat to horizontal
            bat_transform.scale.y = -0.13;
            //if mouse released:
        } else if unsafe { BAT_TRANSFORMED } {
                bat_transform.scale.y = 0.13;
        }
    
    }

    if unsafe { MOUSE_BUTTON_PRESSED } {
        for (mut _ball, mut ball_velocity) in query.iter_mut() {
            // Initialize the ball's velocity
            ball_velocity.velocity = Vec3::new(0.0, 0.0, 0.0);

            // hit based on game pong functionality, until i can get the cursor library approved
            if input_state.pressed(KeyCode::W) {
                ball_velocity.velocity.y = HIT_POWER.y; //ball moves up
            }
            if input_state.pressed(KeyCode::S) {
                ball_velocity.velocity.y = -HIT_POWER.y; //down
            }
            if input_state.pressed(KeyCode::A) {
                ball_velocity.velocity.x = -HIT_POWER.x; //left
            }
            if input_state.pressed(KeyCode::D) {
                ball_velocity.velocity.x = HIT_POWER.x; //right
            } else if !input_state.pressed(KeyCode::W)
                && !input_state.pressed(KeyCode::S)
                && !input_state.pressed(KeyCode::A)
                && !input_state.pressed(KeyCode::D) { 
                
                ball_velocity.velocity.y = HIT_POWER.y;
            }
        }
    }
}

