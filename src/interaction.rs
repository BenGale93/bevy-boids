use bevy::{
    app::{Plugin, Startup, Update},
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{component::Component, system::Commands},
    input::mouse::MouseWheel,
    prelude::{
        default, Camera, EventReader, Input, KeyCode, OrthographicProjection, Query, Res,
        Transform, Vec3, With,
    },
    time::Time,
};

use crate::config;

// Camera
pub const PAN_SPEED: f32 = 1000.0;
pub const ZOOM_SPEED: f32 = 0.1;

pub fn move_camera_system(
    kb_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut camera_query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
) {
    let time_delta = time.delta().as_secs_f32();
    let (mut transform, _) = camera_query.single_mut();
    // Panning.
    let mut x_direction = 0.0;
    let mut y_direction = 0.0;

    if kb_input.pressed(KeyCode::A) {
        x_direction -= 1.0;
    }
    if kb_input.pressed(KeyCode::D) {
        x_direction += 1.0;
    }

    if kb_input.pressed(KeyCode::S) {
        y_direction -= 1.0;
    }
    if kb_input.pressed(KeyCode::W) {
        y_direction += 1.0;
    }

    let new_x_position = (x_direction * time_delta).mul_add(PAN_SPEED, transform.translation.x);
    let new_y_position = (y_direction * time_delta).mul_add(PAN_SPEED, transform.translation.y);

    transform.translation.x = new_x_position;
    transform.translation.y = new_y_position;
}

pub fn camera_zooming_system(
    mut mouse_wheel_event_reader: EventReader<MouseWheel>,
    mut query: Query<(&Camera, &mut Transform)>,
) {
    let mut zoom_scalar = 1.0;
    for mouse_wheel_event in mouse_wheel_event_reader.read() {
        zoom_scalar *= ZOOM_SPEED.mul_add(-mouse_wheel_event.y, 1.0);
    }

    for (_, mut transform) in query.iter_mut() {
        // BUG: for some reason, when camera scale < 1.0, things just disappear!
        let zoomed = transform.scale * zoom_scalar;
        let limited = Vec3::new(zoomed.x.max(1.0), zoomed.y.max(1.0), zoomed.z.max(1.0));
        transform.scale = limited;
    }
}

#[derive(Component)]
struct CameraMarker;

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(100.0, 200.0, 0.0),
            ..default()
        },
        CameraMarker,
    ));
}

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(config::BACKGROUND)
            .add_systems(Startup, setup_camera)
            .add_systems(Update, (camera_zooming_system, move_camera_system));
    }
}
