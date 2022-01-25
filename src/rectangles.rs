use bevy::{
    app::Events,
    core::Time,
    ecs::{
        component::Component,
        system::{Commands, Query, Res},
    },
    math::{Vec2, Vec3},
    render::color::Color,
    transform::components::Transform,
    window::{WindowResized, Windows},
};
use bevy_prototype_lyon::{
    draw::{DrawMode, FillMode, StrokeMode},
    geometry::GeometryBuilder,
    prelude::FillOptions,
    shapes,
    shapes::RectangleOrigin,
};
use rand::{thread_rng, Rng};

#[derive(Component)]
pub struct RectangleObject {
    velocity: f32,
    width: f32,
    teleport_target: f32,
}

pub fn setup(mut commands: Commands, windows: Res<Windows>) {
    let mut rng = thread_rng();
    let window = windows.get_primary().unwrap();
    let (width, height) = (window.width(), window.height());
    let teleport_target = -(width / 2.);

    let default_shape = shapes::Rectangle {
        extents: Vec2::ZERO,
        origin: RectangleOrigin::BottomLeft,
    };
    let default_draw_mode = DrawMode::Outlined {
        fill_mode: FillMode {
            options: FillOptions::default().with_intersections(false),
            color: Color::WHITE,
        },
        outline_mode: StrokeMode::new(Color::BLACK, 1.5),
    };

    for _ in 0..1000 {
        let dimensions = Vec2::splat(10. + rng.gen::<f32>() * 40.);
        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &shapes::Rectangle {
                    extents: dimensions,
                    ..default_shape
                },
                default_draw_mode,
                Transform::from_translation(Vec3::new(
                    (rng.gen::<f32>() - 0.5) * width,
                    (rng.gen::<f32>() - 0.5) * height,
                    0.,
                )),
            ))
            .insert(RectangleObject {
                velocity: rng.gen_range(60.0..120.0),
                width: dimensions.x,
                teleport_target: teleport_target - dimensions.x,
            });
    }
}

pub fn bounds_updater(
    resize_event: Res<Events<WindowResized>>,
    mut rectangles_query: Query<&mut RectangleObject>,
) {
    let target_event = resize_event
        .get_reader()
        .iter(&resize_event)
        .filter(|e| e.id.is_primary())
        .last();

    if let Some(e) = target_event {
        let teleport_target = -(e.width / 2.);
        for mut r in rectangles_query.iter_mut() {
            r.teleport_target = teleport_target - r.width;
        }
    }
}

pub fn movement(time: Res<Time>, mut rectangles_query: Query<(&RectangleObject, &mut Transform)>) {
    for (r, mut transform) in rectangles_query.iter_mut() {
        transform.translation.x -= r.velocity * time.delta_seconds();
    }
}

pub fn collision_detection(mut rectangles_query: Query<(&RectangleObject, &mut Transform)>) {
    for (r, mut transform) in rectangles_query.iter_mut() {
        if transform.translation.x < r.teleport_target {
            transform.translation.x = -transform.translation.x;
        }
    }
}
