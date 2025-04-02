use bevy::prelude::*;

pub struct GamePlugin;

#[derive(Component)]
#[require(Camera2d)]
pub struct MainCamera;


impl Plugin for GamePlugin {
    fn build(&self,app: &mut App){
        app.add_systems(Startup,setup_scene);
    }

}


fn setup_scene (mut commands: Commands){
    commands.spawn(MainCamera);
}

