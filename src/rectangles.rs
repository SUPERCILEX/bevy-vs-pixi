use std::fmt::Write;

use bevy::{
    app::{App, Events, Plugin},
    asset::AssetServer,
    core::Time,
    ecs::{
        component::Component,
        query::With,
        system::{Commands, Query, Res, ResMut},
    },
    input::{mouse::MouseButton, Input},
    math::{Rect, Vec2, Vec3},
    render::color::Color,
    text::{Text, TextSection, TextStyle},
    transform::components::Transform,
    ui::{entity::TextBundle, PositionType, Style, Val},
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

pub struct RectanglesPlugin;

impl Plugin for RectanglesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Stats::default())
            .add_startup_system(setup)
            .add_system(bounds_updater)
            .add_system(movement)
            .add_system(collision_detection)
            .add_system(mouse_handler)
            .add_system(stats_system);
    }
}

struct Stats {
    count: u32,
}

impl Default for Stats {
    fn default() -> Self {
        Stats { count: 250 }
    }
}

#[derive(Component)]
struct StatsText;

#[derive(Component)]
struct RectangleObject {
    velocity: f32,
    width: f32,
    teleport_target: f32,
}

fn setup(
    mut commands: Commands,
    windows: Res<Windows>,
    stats: Res<Stats>,
    asset_server: Res<AssetServer>,
) {
    spawn_rectangles(&mut commands, &windows, stats.count);

    commands
        .spawn_bundle(TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Rectangle count: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::BLACK,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::BLACK,
                        },
                    },
                ],
                ..Default::default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(0.),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(StatsText);
}

fn mouse_handler(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut stats: ResMut<Stats>,
) {
    if mouse_button_input.just_released(MouseButton::Left) {
        spawn_rectangles(&mut commands, &windows, stats.count);
        stats.count *= 2;
    }
}

fn spawn_rectangles(commands: &mut Commands, windows: &Windows, num: u32) {
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

    for _ in 0..num {
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

fn bounds_updater(
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
        rectangles_query.for_each_mut(|mut r| {
            r.teleport_target = teleport_target - r.width;
        });
    }
}

fn movement(time: Res<Time>, mut rectangles_query: Query<(&RectangleObject, &mut Transform)>) {
    rectangles_query.for_each_mut(|(r, mut transform)| {
        transform.translation.x -= r.velocity * time.delta_seconds();
    });
}

fn collision_detection(mut rectangles_query: Query<(&RectangleObject, &mut Transform)>) {
    rectangles_query.for_each_mut(|(r, mut transform)| {
        if transform.translation.x < r.teleport_target {
            transform.translation.x = -transform.translation.x;
        }
    });
}

fn stats_system(stats: Res<Stats>, mut query: Query<&mut Text, With<StatsText>>) {
    if !stats.is_changed() {
        return;
    }

    let mut text = query.single_mut();
    text.sections[1].value.clear();
    write!(text.sections[1].value, "{}", stats.count).unwrap();
}
