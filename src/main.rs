use bevy::prelude::*;
use bevy::input::mouse::{MouseButtonInput, MouseMotion, MouseWheel};
mod minesweeper_model;
// use minesweeper_model::*;

#[derive(Component)]
struct MineCell(usize, usize);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(handle_mouse_clicks)
        // .add_system(print_mouse_events_system)
        .add_startup_system(setup)
        .run()
}

const CELL_WIDTH: f32 = 20f32;
const CELL_HEIGHT: f32 = CELL_WIDTH;

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
    for x in 0..minesweeper_model::GRID_SIZE.0 {
        for y in 0..minesweeper_model::GRID_SIZE.0 {
            commands.spawn(
                (
                    MineCell(x,y),
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::YELLOW,
                            custom_size: Some(Vec2::new(CELL_WIDTH,CELL_HEIGHT)),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz((x as f32)*CELL_WIDTH, (y as f32)*CELL_HEIGHT, 0.0),
                        ..Default::default()
                    }
                )
            );
        }
    }
}

fn handle_mouse_clicks(mouse_input: Res<Input<MouseButton>>, windows_query: Query<&Window>) {
    let win = windows_query.single();
    if mouse_input.just_pressed(MouseButton::Left) {
        println!("click at {:?}", win.cursor_position());
    }
}


#[warn(dead_code)]
fn print_mouse_events_system(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
) {
    for event in mouse_button_input_events.iter() {
        info!("{:?}", event);
    }

    for event in mouse_motion_events.iter() {
        info!("{:?}", event);
    }

    for event in cursor_moved_events.iter() {
        info!("{:?}", event);
    }

    for event in mouse_wheel_events.iter() {
        info!("{:?}", event);
    }
}
