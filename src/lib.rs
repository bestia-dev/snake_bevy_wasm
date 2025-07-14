use bevy::{ input::common_conditions::input_just_pressed, prelude::*};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main() {
    let mut app = bevy::app::App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            // provide the ID selector string here
            canvas: Some("#snake_bevy_canvas".into()),
            resolution:
                bevy::window::WindowResolution::new(1000., 1000.).with_scale_factor_override(1.0),
            // ... any other window properties ...
            ..default()
        }),
        ..default()
    }));

    app.add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                snake_move,
                event_up.run_if(input_just_pressed(KeyCode::ArrowUp)),
                event_down.run_if(input_just_pressed(KeyCode::ArrowDown)),
                event_left.run_if(input_just_pressed(KeyCode::ArrowLeft)),
                event_right.run_if(input_just_pressed(KeyCode::ArrowRight)),
            ),
        )
        .run();
}

#[derive(Component)]
struct FoodPosition {
    x: i32,
    y: i32,
}

#[derive(Component)]
enum SnakeDirection {
    Up,
    Down,
    Left,
    Right,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let color = Color::hsl(300., 0.95, 0.7);

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(50.0, 50.0))),
        Transform::from_xyz(0., 0., 0.),
        MeshMaterial2d(materials.add(color)),
        SnakeDirection::Down,
    ));

    let color = Color::hsl(2., 0.95, 0.7);

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(50.0))),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(0., 100.0, 0.0),
        FoodPosition { x: 50, y: 50 },
    ));
}

fn snake_move(time: Res<Time>, mut snake_position: Query<(&mut SnakeDirection, &mut Transform)>) {
    if let Ok((direction, mut transform)) = snake_position.single_mut() {
        match direction.as_ref() {
            SnakeDirection::Up => transform.translation.y += 50. * time.delta_secs(),
            SnakeDirection::Down => transform.translation.y -= 50. * time.delta_secs(),
            SnakeDirection::Left => transform.translation.x -= 50. * time.delta_secs(),
            SnakeDirection::Right => transform.translation.x += 50. * time.delta_secs(),
        }
    }
}

fn event_up(mut snake_direction: Query<&mut SnakeDirection>) {
    if let Ok(mut d) = snake_direction.single_mut() {
        *d = SnakeDirection::Up;
    }
}
fn event_down(mut snake_direction: Query<&mut SnakeDirection>) {
    if let Ok(mut d) = snake_direction.single_mut() {
        *d = SnakeDirection::Down;
    }
}
fn event_left(mut snake_direction: Query<&mut SnakeDirection>) {
    if let Ok(mut d) = snake_direction.single_mut() {
        *d = SnakeDirection::Left;
    }
}
fn event_right(mut snake_direction: Query<&mut SnakeDirection>) {
    if let Ok(mut d) = snake_direction.single_mut() {
        *d = SnakeDirection::Right;
    }
}
