use bevy::ecs::world;
use bevy::{prelude::*, transform};
use bevy::input::mouse::{MouseButtonInput, MouseMotion, MouseWheel};
use bevy::render::camera::RenderTarget;
#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
mod minesweeper_model;
// use minesweeper_model::*;

#[derive(Component)]
struct MineCell(usize, usize);

#[derive(Resource)]
struct Board<const H: usize, const W: usize>(minesweeper_model::Minesweeper<H, W>);

#[derive(Component)]
struct MainCamera;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_system(my_cursor_system);
    #[cfg(feature = "debug")]
    // Debug hierarchy inspector
    app.add_plugin(WorldInspectorPlugin::new());
    // .add_system(print_mouse_events_system)
    app.add_startup_system(setup);
    app.insert_resource(Board(minesweeper_model::random_grid::<20,20>(20.0)));
    app.add_system(show_cells);
    app.run();
}

const CELL_WIDTH: f32 = 20f32;
const CELL_HEIGHT: f32 = CELL_WIDTH;
const CELL_SIZE: Vec2 = Vec2::new(CELL_WIDTH,CELL_HEIGHT);

pub fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
    for x in 0..minesweeper_model::GRID_SIZE.0 {
        for y in 0..minesweeper_model::GRID_SIZE.0 {
            let gap = 1f32;
            let pos =  ((x as f32)*(CELL_WIDTH + gap), (y as f32)*(CELL_HEIGHT + gap));
            let entitity = commands.spawn(
                (
                    MineCell(x,y),
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::YELLOW,
                            custom_size: Some(CELL_SIZE),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(pos.0, pos.1, 0.0),
                        ..Default::default()
                    }
                )
            ).id();
        }
    }
}


fn handle_mouse_clicks(
    mouse_input: Res<Input<MouseButton>>,
    windows_query: Query<&Window>
) {
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

fn my_cursor_system (
    mut commands: Commands,
    // need to get window dimensions
    windows: Query<&Window>,
    // query to get camera transform
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    cells: Query<(Entity, &Transform, &MineCell)>,
    mouse_input: Res<Input<MouseButton>>,
    board: Res<Board<20,20>>
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = camera_q.single();

    // get the window that the camera is displaying to (or the primary window)
    let window = windows.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window.cursor_position()
    .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
    .map(|ray| ray.origin.truncate())
    {
        if mouse_input.just_pressed(MouseButton::Left){
            let cursor = (world_position.x, world_position.y);
            // eprintln!("World coords: ({},{})", world_position.x, world_position.y);
            for (entity, transform, MineCell(x,y)) in cells.into_iter() {
                let cell = {
                    let Vec3{x,y, z: _} = transform.translation;
                    (x,y)
                };
                if (cell.0 - cursor.0).abs() <= CELL_WIDTH/2.0 && (cell.1 - cursor.1).abs() <= CELL_HEIGHT/2.0 {
                    // commands.entity(entity).insert(Sprite{color: Color::ORANGE, custom_size: Some(Vec2::new(CELL_WIDTH,CELL_HEIGHT)), ..Default::default()});
                    // CLICKED
                }
            }
        }
    }
}

fn show_cells(
    mut commands: Commands,
    cells: Query<(Entity,&MineCell)>,
    board: Res<Board<20,20>>,
){
    for (entity, &MineCell(x,y)) in cells.into_iter(){
        if board.0.is_bomb(&(x,y)) {
            commands.entity(entity).insert(Sprite { color: Color::RED, custom_size: Some(CELL_SIZE), ..Default::default()});
        } else {
            commands.entity(entity).insert(Sprite { color: Color::GREEN, custom_size: Some(CELL_SIZE), ..Default::default()});
        }
    }
}