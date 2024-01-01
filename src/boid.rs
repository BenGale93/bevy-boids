use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

fn boid_points(center: (f32, f32), angle: f32) -> [Vec2; 3] {
    let (t_y, t_x) = angle.sin_cos();
    let tip = Vec2::new(center.0 + t_x, center.1 + t_y);
    let left = Vec2::new(center.0 - t_y * 0.4, center.1 - t_x * 0.4);
    let right = Vec2::new(center.0 + t_y * 0.4, center.1 + t_x * 0.4);

    [tip, left, right].map(|x| x * 100.)
}

#[derive(Bundle)]
pub struct BoidBundle {
    shape: ShapeBundle,
    color: Fill,
    stroke: Stroke,
    collider: Collider,
}

impl BoidBundle {
    fn spawn_boid(center: (f32, f32), angle: f32) -> Self {
        let points = boid_points(center, angle);

        let shape = shapes::Polygon {
            points: points.into_iter().collect(),
            closed: true,
        };

        let collider = Collider::convex_hull(&points).unwrap();
        Self {
            shape: ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                ..default()
            },
            color: Fill::color(Color::CYAN),
            stroke: Stroke::new(Color::BLACK, 1.0),
            collider,
        }
    }
}

fn setup_system(mut commands: Commands) {
    let locations = [((0.0, 0.0), PI / 2.0), ((0.0, 0.0), PI)];
    for location in &locations {
        commands
            .spawn(BoidBundle::spawn_boid(location.0, location.1))
            .insert(RigidBody::Dynamic);
    }
}

pub struct BoidPlugin;

impl Plugin for BoidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_system);
    }
}
