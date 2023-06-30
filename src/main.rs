use bevy::prelude::*;
use bevy::input::mouse::{MouseButtonInput, MouseMotion, MouseWheel};
#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
mod minesweeper_model;
// use minesweeper_model::*;

#[derive(Component)]
struct MineCell(usize, usize);

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_system(handle_mouse_clicks);
    #[cfg(feature = "debug")]
    // Debug hierarchy inspector
    app.add_plugin(WorldInspectorPlugin::new());
    // .add_system(print_mouse_events_system)
    app.add_startup_system(setup);
    app.run();
}

const CELL_WIDTH: f32 = 20f32;
const CELL_HEIGHT: f32 = CELL_WIDTH;

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    for x in 0..minesweeper_model::GRID_SIZE.0 {
        for y in 0..minesweeper_model::GRID_SIZE.0 {
            let size = Vec2::new(CELL_WIDTH,CELL_HEIGHT);
            let pos =  ((x as f32)*CELL_WIDTH, (y as f32)*CELL_HEIGHT);
            commands.spawn(
                (
                    MineCell(x,y),
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::YELLOW,
                            custom_size: Some(size),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(pos.0, pos.1, 0.0),
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
