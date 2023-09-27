use bevy::prelude::*;


#[derive(Component)]
pub struct Credits;

// Load credit screen assets
pub fn load_credit_texture(mut commands: Commands){
    // Spawn credits sprite
    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    }).insert(Credits);
  
}

pub fn cycle_credits(mut update_screen: Query<&mut Handle<Image>, With<Credits>>, keyboard: Res<Input<KeyCode>>, asset_server: Res<AssetServer>){
    // Add your images here
    let credits_lib = vec!["Alex_Credits.png", "PaulR.png", "JakobR.png", "brayden.png", "nicolecredit.png", "lgy2credits.png", "RafaelCredits.png","Jimmy.png"];
    for(mut handle) in update_screen.iter_mut(){ 
        // Credits for everyone from number 1 - 9
        if keyboard.just_pressed(KeyCode::Key1){
            *handle = asset_server.load(credits_lib[0]);
        }
        if keyboard.just_pressed(KeyCode::Key2){
            *handle = asset_server.load(credits_lib[1]);
        }
        if keyboard.just_pressed(KeyCode::Key3){
            *handle = asset_server.load(credits_lib[2]);
        }
        if keyboard.just_pressed(KeyCode::Key4){
            *handle = asset_server.load(credits_lib[3]);
        }
        if keyboard.just_pressed(KeyCode::Key5){
            *handle = asset_server.load(credits_lib[4]);
        }
        if keyboard.just_pressed(KeyCode::Key6){
            *handle = asset_server.load(credits_lib[5]);
        }
        if keyboard.just_pressed(KeyCode::Key7){
            *handle = asset_server.load(credits_lib[6]);
        }
        if keyboard.just_pressed(KeyCode::Key8){
            *handle = asset_server.load(credits_lib[7]);
        }
    }
}
