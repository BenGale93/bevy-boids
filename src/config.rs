use bevy::{
    core_pipeline::clear_color::ClearColor, ecs::system::ResMut, math::Vec2, render::color::Color,
};
use bevy_rapier2d::plugin::RapierConfiguration;

pub const BACKGROUND: ClearColor = ClearColor(Color::rgb(0.004, 0.09, 0.15));

pub fn physics_setup(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}
