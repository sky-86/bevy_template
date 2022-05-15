use bevy::{prelude::*, render::camera::ScalingMode, window::PresentMode};

pub const CLEAR: Color = Color::rgb(1.0, 1.0, 1.0);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const WINDOW_HEIGHT: f32 = 900.0;
pub const WINDOW_WIDTH: f32 = WINDOW_HEIGHT * RESOLUTION;
pub const VECTOR_SIZE: usize = 255;
pub const GRID_WIDTH: u32 = VECTOR_SIZE as u32;
pub const GRID_HEIGHT: u32 = VECTOR_SIZE as u32;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut transform) in q.iter_mut() {
        println!(
            "width: {}, height: {}, transform: {:?}",
            sprite_size.width, sprite_size.height, transform
        );
        transform.scale = Vec3::new(
            sprite_size.width / GRID_WIDTH as f32 * window.width() as f32,
            sprite_size.height / GRID_HEIGHT as f32 * window.height() as f32,
            1.0,
        );
    }
}

fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.0) + (tile_size / 2.0)
    }

    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, GRID_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, GRID_HEIGHT as f32),
            0.0,
        );
    }
}

#[derive(Component)]
struct Data(Vec<u8>);

fn create_random_vec() -> Vec<u8> {
    let vec: Vec<u8> = (1..=VECTOR_SIZE as u8).collect();
    //    vec.shuffle(&mut thread_rng());
    vec
}

fn spawn_vector(mut commands: Commands) {
    //let Data(vec) = Data(create_random_vec());
    let parent_box = commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.1, 0.1, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Position { x: 0, y: 0 })
        .insert(Size::square(1.0))
        .id();

    println!("Spawned snake {:?}", parent_box);

    /*
    for i in vec {
        let col = commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.1, 0.1, 1.0),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 1.0),
                    scale: Vec3::new(0.0, 0.0, 1.0),

                    ..Default::default()
                },
                ..Default::default()
            })
            .id();

        commands.entity(parent_box).push_children(&[col]);
    }
        */
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: WINDOW_HEIGHT * RESOLUTION,
            height: WINDOW_HEIGHT,
            title: "Sorting Visualizer".to_string(),
            resizable: false,
            present_mode: PresentMode::Fifo,
            ..default()
        })
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_vector)
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation)
                .with_system(size_scaling),
        )
        .add_plugins(DefaultPlugins)
        .run();
}
