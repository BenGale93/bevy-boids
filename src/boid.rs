use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

fn boid_points() -> [Vec2; 3] {
    let tip = Vec2::new(0.0, 1.0);
    let left = Vec2::new(-0.4, 0.0);
    let right = Vec2::new(0.4, 0.0);

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
    fn spawn_boid(center: (f32, f32), angle: f32, color: Color) -> Self {
        let points = boid_points();

        let shape = shapes::Polygon {
            points: points.into_iter().collect(),
            closed: true,
        };

        let collider = Collider::convex_hull(&points).unwrap();
        Self {
            shape: ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                spatial: SpatialBundle {
                    transform: Transform {
                        translation: Vec3::new(center.0, center.1, 0.0),
                        rotation: Quat::from_rotation_z(angle),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            color: Fill::color(color),
            stroke: Stroke::new(Color::BLACK, 1.0),
            collider,
        }
    }
}

fn setup_system(mut commands: Commands) {
    let spawn_data = [
        ((0.0, 0.0), 0.0, Color::CYAN),
        ((100.0, 0.0), PI, Color::CRIMSON),
    ];
    for data in &spawn_data {
        commands
            .spawn(BoidBundle::spawn_boid(data.0, data.1, data.2))
            .insert(RigidBody::Dynamic)
            .insert(Damping {
                linear_damping: 1.0,
                angular_damping: 1.0,
            })
            .insert(Velocity::zero());
    }
}

fn movement_system(mut query: Query<(&Transform, &mut Velocity)>) {
    for (transform, mut velocity) in query.iter_mut() {
        //velocity.angvel = 0.0;
        velocity.linvel = (100.0 * transform.local_y()).truncate();
    }
}

pub struct BoidPlugin;

impl Plugin for BoidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_system)
            .add_systems(Update, movement_system);
    }
}
