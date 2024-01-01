use bevy::prelude::*;
use bevy_prototype_lyon::plugin::ShapePlugin;
use bevy_rapier2d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};

mod boid;
mod config;
mod interaction;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugins((
            ShapePlugin,
            boid::BoidPlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1000.0),
            interaction::InteractionPlugin,
        ))
        .add_systems(Startup, config::physics_setup)
        .insert_resource(Msaa::Sample4);

    #[cfg(debug_assertions)]
    app.add_plugins(RapierDebugRenderPlugin::default());

    app.run();
}
