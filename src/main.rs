use bevy::{prelude::*, transform};
use bevy::input::mouse::{MouseButtonInput, MouseMotion, MouseWheel};
#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use std::collections::HashMap;
mod minesweeper_model;
// use minesweeper_model::*;

#[derive(Component)]
struct MineCell{
    x: usize, 
    y: usize, 
    hidden: bool,
}

#[derive(Resource)]
pub struct Board<const H: usize, const W: usize>(minesweeper_model::Minesweeper<H, W>);

#[derive(Resource)]
pub struct MapBecauseWeReLazy(HashMap<(usize, usize), Entity>);

#[derive(Resource)]
pub enum WeLost {
    Yes, No
}

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
    app.insert_resource(Board(minesweeper_model::random_grid::<20,20>(0.20)));
    app.insert_resource(MapBecauseWeReLazy(HashMap::new()));
    app.insert_resource(WeLost::No);
    app.add_system(show_cells);
    app.add_system(my_cursor_system);
    app.add_system(reveal_zeros_neighbours);
    app.add_system(reveal_when_lost);
    app.run();
}

const CELL_WIDTH: f32 = 20f32;
const CELL_HEIGHT: f32 = CELL_WIDTH;
const CELL_SIZE: Vec2 = Vec2::new(CELL_WIDTH,CELL_HEIGHT);

fn setup(
    mut commands: Commands,
    board: Res<Board<20,20>>,
    mut map: ResMut<MapBecauseWeReLazy>,
) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
    for x in 0..minesweeper_model::GRID_SIZE.0 {
        for y in 0..minesweeper_model::GRID_SIZE.0 {
            let gap = 1f32;
            let pos =  ((x as f32)*(CELL_WIDTH + gap), (y as f32)*(CELL_HEIGHT + gap));
            let entity = commands.spawn(
                (
                    MineCell { x, y, hidden:true },
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
            map.0.insert((x,y), entity);
        }
    }
    println!("{:?}", board.0);
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
    mut map: ResMut<MapBecauseWeReLazy>,
    // need to get window dimensions
    windows: Query<&Window>,
    // query to get camera transform
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    cells: Query<(Entity, &Transform, &MineCell)>,
    mouse_input: Res<Input<MouseButton>>,
    board: Res<Board<20,20>>,
    mut we_lost: ResMut<WeLost>,
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
        let cursor = (world_position.x, world_position.y);
        if mouse_input.just_pressed(MouseButton::Left){
            // eprintln!("World coords: ({},{})", world_position.x, world_position.y);
            for (entity, transform, &MineCell{x,y,..}) in cells.into_iter() {
                let cell = {
                    let Vec3{x,y, z: _} = transform.translation;
                    (x,y)
                };
                if (cell.0 - cursor.0).abs() <= CELL_WIDTH/2.0 && (cell.1 - cursor.1).abs() <= CELL_HEIGHT/2.0 {
                    commands.entity(entity).insert(MineCell{x,y, hidden: false});
                    if board.0.is_bomb(&(x,y)){
                        *we_lost = WeLost::Yes;
                    }
                }
            }
        }
    }
}

fn reveal_when_lost(
    mut commands: Commands,
    cells: Query<(Entity,&MineCell)>,
    we_lost: Res<WeLost>,
) {
    match *we_lost {
        WeLost::Yes => {
            for (entity, mc) in cells.into_iter() {
                commands.entity(entity).insert(MineCell{hidden: false, ..*mc});
            }
        },
        _ => (),
    }
}

fn reveal_zeros_neighbours(
    mut commands: Commands,
    mut map: ResMut<MapBecauseWeReLazy>,
    board: Res<Board<20,20>>,
    cells: Query<(&MineCell, Entity)>,
) {
    for (&MineCell{x,y,hidden}, entity) in cells.into_iter() {
        let p = (x,y);
        if board.0.neighbours_count(p) == 0 && !hidden {
            for neigh in board.0.neighbours(p).into_iter() {
                match map.0.entry(neigh) {
                    std::collections::hash_map::Entry::Occupied(entry) => {
                        let entity = entry.get();
                        commands.entity(*entity).insert(MineCell{x: neigh.0, y: neigh.1, hidden: false});
                    },
                    std::collections::hash_map::Entry::Vacant(_) => {

                    },
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
    for (entity, &MineCell{x,y,hidden}) in cells.into_iter(){
        if !hidden {
            if board.0.is_bomb(&(x,y)) {
                commands.entity(entity).insert(Sprite { color: Color::RED, custom_size: Some(CELL_SIZE), ..Default::default()});
            } else {
                commands.entity(entity).insert(Sprite { color: Color::GREEN, custom_size: Some(CELL_SIZE), ..Default::default()});
            }
        }
    }
}