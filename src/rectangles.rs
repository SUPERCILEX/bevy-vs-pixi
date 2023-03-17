use std::cmp::max;

use bevy::{
    ecs::event::Events,
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

const BORDER_COLOR: Color = Color::BLACK;
// Workaround for poor batching with mixed WHITE and other-colored sprites.
// TODO https://github.com/bevyengine/bevy/issues/8100
const FILL_COLOR: Color = Color::rgb(1.0 - f32::EPSILON, 1.0, 1.0);

pub struct RectanglesPlugin;

impl Plugin for RectanglesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Stats>();
        app.init_resource::<PseudoRng>();
        app.add_startup_system(setup);
        app.add_systems((bounds_updater, movement, collision_detection).chain());
        app.add_system(mouse_handler);
    }
}

#[derive(Resource)]
pub struct Stats {
    pub count: u32,
}

impl Default for Stats {
    fn default() -> Self {
        Self { count: 250 }
    }
}

#[derive(Resource)]
pub struct PseudoRng(Xoshiro256PlusPlus);

impl Default for PseudoRng {
    fn default() -> Self {
        Self(Xoshiro256PlusPlus::seed_from_u64(395992934456271))
    }
}

#[derive(Component)]
struct RectangleObject {
    velocity: f32,
    width: f32,
    teleport_target: f32,
}

fn setup(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    stats: Res<Stats>,
    mut rng: ResMut<PseudoRng>,
) {
    let Ok(window) = window.get_single() else {
        return;
    };

    spawn_rectangles(&mut commands, window, &mut rng.0, stats.count);
}

fn mouse_handler(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut stats: ResMut<Stats>,
    rectangles: Query<Entity, With<RectangleObject>>,
    mut rng: ResMut<PseudoRng>,
) {
    let Ok(window) = window.get_single() else {
        return;
    };

    let old = stats.count;
    if mouse_button_input.just_released(MouseButton::Left) {
        stats.count = max(1, stats.count * 2);
        spawn_rectangles(&mut commands, window, &mut rng.0, stats.count - old);
    }
    if mouse_button_input.just_released(MouseButton::Right) {
        stats.count /= 2;
        despawn_rectangles(&mut commands, rectangles, old - stats.count);
    }
}

fn spawn_rectangles(
    commands: &mut Commands,
    window: &Window,
    rng: &mut Xoshiro256PlusPlus,
    num: u32,
) {
    let (width, height) = (window.width(), window.height());
    let teleport_target = -(width / 2.);

    for _ in 0..num {
        let dimensions = Vec2::splat(rng.gen::<f32>().mul_add(40., 10.));
        commands
            .spawn((
                RectangleObject {
                    velocity: rng.gen_range(60.0..120.0),
                    width: dimensions.x,
                    teleport_target: teleport_target - dimensions.x,
                },
                SpriteBundle {
                    sprite: Sprite {
                        color: BORDER_COLOR,
                        custom_size: Some(dimensions),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        (rng.gen::<f32>() - 0.5) * width,
                        (rng.gen::<f32>() - 0.5) * height,
                        rng.gen::<f32>(),
                    ),
                    ..default()
                },
            ))
            .with_children(|children| {
                children.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: FILL_COLOR,
                        custom_size: Some(dimensions - Vec2::splat(3.)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0., 0., f32::EPSILON),
                    ..default()
                });
            });
    }
}

fn despawn_rectangles(
    commands: &mut Commands,
    rectangles: Query<Entity, With<RectangleObject>>,
    num: u32,
) {
    for r in rectangles.iter().take(num as usize) {
        commands.entity(r).despawn_recursive();
    }
}

fn bounds_updater(
    window: Query<Entity, With<PrimaryWindow>>,
    resize_event: Res<Events<WindowResized>>,
    mut rectangles_query: Query<&mut RectangleObject>,
) {
    let Ok(window_id) = window.get_single() else {
        return;
    };

    let mut reader = resize_event.get_reader();
    if let Some(e) = reader
        .iter(&resize_event)
        .filter(|e| e.window == window_id)
        .last()
    {
        let teleport_target = -(e.width / 2.);
        rectangles_query.par_iter_mut().for_each_mut(|mut r| {
            r.teleport_target = teleport_target - r.width;
        });
    }
}

fn movement(time: Res<Time>, mut rectangles_query: Query<(&RectangleObject, &mut Transform)>) {
    rectangles_query
        .par_iter_mut()
        .for_each_mut(|(r, mut transform)| {
            transform.translation.x -= r.velocity * time.delta_seconds();
        });
}

fn collision_detection(mut rectangles_query: Query<(&RectangleObject, &mut Transform)>) {
    rectangles_query
        .par_iter_mut()
        .for_each_mut(|(r, mut transform)| {
            if transform.translation.x < r.teleport_target {
                transform.translation.x = -transform.translation.x;
            }
        });
}
