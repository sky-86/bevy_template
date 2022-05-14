use bevy::{prelude::*, render::camera::ScalingMode, window::PresentMode};

pub const CLEAR: Color = Color::rgb(1.0, 1.0, 1.0);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const VECTOR_SIZE: usize = 255;
pub const WINDOW_HEIGHT: f32 = 900.0;

#[derive(Component)]
struct Data(Vec<u8>);

fn create_random_vec() -> Vec<u8> {
    let vec: Vec<u8> = (1..=VECTOR_SIZE as u8).collect();
    //    vec.shuffle(&mut thread_rng());
    vec
}

fn spawn_vector(mut commands: Commands) {
    let Data(vec) = Data(create_random_vec());
    let parent_box = commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.1, 0.1, 1.0),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(RESOLUTION * 2.0, 1.0 * 2.0, 1.0),

                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    //let mut x = ((WINDOW_HEIGHT * RESOLUTION) / -2.0) * -1.0;
    let mut x = -1.0;
    let col_width = 0.0;

    let mut color = 0.0;

    for i in vec {
        let col_height = i as f32 / VECTOR_SIZE as f32;

        let col = commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(color, color, color),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(x as f32, 0.0, 1.0),
                    scale: Vec3::new(col_width, col_height, 1.0),

                    ..Default::default()
                },
                ..Default::default()
            })
            .id();

        commands.entity(parent_box).push_children(&[col]);
        x += col_width;

        color += i as f32 / VECTOR_SIZE as f32;
    }
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;
    camera.orthographic_projection.right = 1.0 * RESOLUTION;
    camera.orthographic_projection.left = -1.0 * RESOLUTION;
    camera.orthographic_projection.scaling_mode = ScalingMode::None;
    commands.spawn_bundle(camera);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
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
        .run();
}
