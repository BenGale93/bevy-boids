use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

fn setup_system(mut commands: Commands) {
    let points = [
        Vec2::new(0.0, 1.0),
        Vec2::new(-0.4, 0.0),
        Vec2::new(0.4, 0.0),
    ]
    .map(|x| x * 100.);

    let shape = shapes::Polygon {
        points: points.into_iter().collect(),
        closed: true,
    };

    let collider = Collider::convex_hull(&points).unwrap();

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            ..default()
        },
        Fill::color(Color::CYAN),
        Stroke::new(Color::BLACK, 1.0),
        collider,
    ));
}

pub struct BoidPlugin;

impl Plugin for BoidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_system);
    }
}
