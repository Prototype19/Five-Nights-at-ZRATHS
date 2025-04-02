use bevy::prelude::*;

pub struct PizzaPlugin;

#[derive(Component)]
struct Pizza;

impl Plugin for PizzaPlugin {
    fn build(&self, app: &mut App){
        app.add_systems(Startup, setup_pizza);
    }
}