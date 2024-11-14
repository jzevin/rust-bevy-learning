use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::*;

// const WINDOW_WIDTH: f32 = 800.0;
// const WINDOW_HEIGHT: f32 = 600.0;
// const BLOB_SIZE: f32 = 128.0;
// const BLOB_SPEED: f32 = 300.0;
const NUM_BLOBS: usize = 5;
const START_X: f32 = -540.0;
const START_Y: f32 = 270.0;
const BLOB_SIZE: f32 = 108.0;
const BLOB_SPEED: f32 = 18.0;
const BLOB_GAP: f32 = 136.0;
const FINISH_LINE: f32 = START_X * -1.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (translate_blob, increment_tally))
        .run();
}

#[derive(Component)]
struct Blob {
    index: usize,
}

// Add Parent/Child components to link text with blobs
#[derive(Component)]
struct BlobLabel;

#[derive(Component)]
struct TallyLabel(usize);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    for index in 0..NUM_BLOBS {
        spawn_blob(&mut commands, &mut meshes, &mut materials, index);
        spawn_tallies(&mut commands, index);
    }
}

fn get_blob_color(index: usize) -> Color {
    match index % 5 {
        0 => Color::srgb(0.118, 0.565, 1.0),   // DODGER_BLUE
        1 => Color::srgb(0.863, 0.078, 0.235), // CRIMSON
        2 => Color::srgb(0.133, 0.545, 0.133), // FOREST_GREEN
        3 => Color::srgb(1.0, 0.843, 0.0),     // GOLD
        _ => Color::srgb(0.502, 0.0, 0.502),   // PURPLE
    }
}


fn spawn_blob(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    index: usize,
) {
    let color = get_blob_color(index);
    let position = Vec3::new(START_X, START_Y - (index as f32 * BLOB_GAP), 0.0);

    // Spawn blob and label as parent/child
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(BLOB_SIZE/2.0)).into(), // Radius is half the desired size
            transform: Transform::default()
                .with_translation(position),
            material: materials.add(color),
            ..default()
        },
        Blob { index },
    )).with_children(|parent| {
        parent.spawn((
            Text2dBundle {
                text: Text::from_section(
                    index.to_string(),
                    TextStyle {
                        font_size: 54.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                transform: Transform::from_xyz(0.0, 0.0, 1.0),  // No need for scale compensation
                ..default()
            },
            BlobLabel,
        ));
    });
}

fn spawn_tallies(
    commands: &mut Commands,
    index: usize,
) {
    let position = Vec3::new(FINISH_LINE, START_Y - (index as f32 * BLOB_GAP), 0.0);
    
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "0",  // Start at zero
                TextStyle {
                    font_size: 54.0,
                    color: get_blob_color(index),  // Match blob color
                    ..default()
                },
            ),
            transform: Transform::from_translation(position),
            ..default()
        },
        TallyLabel(index),  // Track which blob this tally belongs to
    ));
}

// No changes needed to translate_blob - child entities automatically move with parent
fn translate_blob(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Blob>>, // Only move Blob components
) {
    // First check if any blob crossed threshold
    let reset_needed = query
        .iter()
        .any(|transform| transform.translation.x >= FINISH_LINE);

    if reset_needed {
        // Reset all blobs
        for mut transform in query.iter_mut() {
            transform.translation.x = START_X;
        }
    } else {
        // Normal movement
        for mut transform in query.iter_mut() {
            let step = random::<f32>() * BLOB_SPEED;
            // let delta = time.delta_seconds() * step;
            transform.translation.x += step;
        }
    }
}


fn increment_tally(
    mut query: Query<(&mut Text, &TallyLabel)>,
    blob_query: Query<(&Transform, &Blob)>,
) {
    for (transform, blob) in blob_query.iter() {
        if transform.translation.x >= FINISH_LINE && transform.translation.x < FINISH_LINE + 5.0 {
            for (mut text, tally) in query.iter_mut() {
                if tally.0 == blob.index {
                    if let Some(value) = text.sections[0].value.parse::<i32>().ok() {
                        text.sections[0].value = (value + 1).to_string();
                    }
                }
            }
        }
    }
}