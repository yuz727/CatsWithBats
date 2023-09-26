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
    let credits_lib = vec!["Crystal.png", "PaulR.png"];
    for(mut handle) in update_screen.iter_mut(){ 
        // Credits for everyone from number 1 - 9
        if keyboard.just_pressed(KeyCode::Key1){
            *handle = asset_server.load(credits_lib[0]);
        }
        if keyboard.just_pressed(KeyCode::Key2){
            *handle = asset_server.load(credits_lib[1]);
        }
        
        // if keyboard.just_pressed(KeyCode::Key2){
        //     *handle = asset_server.load(credits_lib[1]);
        // }
    }
}