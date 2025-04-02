use bevy::prelude::*;
pub mod game;
pub mod pizza;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            game::GamePlugin
        ))
        .run();
}
