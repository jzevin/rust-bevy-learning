#![allow(unused)]

use bevy::{ecs::query, prelude::*, state::app::StatesPlugin};

const NUM_RACERS: usize = 6;

fn main() {
    App::new()
        .add_plugins((MinimalPlugins, StatesPlugin))
        .init_state::<RaceState>()
        .add_systems(Startup, spawn_racers)
        .add_event::<RaceStartEvent>()
        .add_event::<RaceEndEvent>()
        .add_systems(
            Update,
            (
                start_race_system.run_if(in_state(RaceState::Ready)),
                translate_racers_system.run_if(in_state(RaceState::Racing)),
                change_race_state.run_if(in_state(RaceState::Ready)),
                // check_finish_system.run_if(in_state(RaceState::Racing)),
                // reset_race_system.run_if(in_state(RaceState::Finished)),
            ),
        )
        .run();
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum RaceState {
    #[default]
    Ready,
    Racing,
    Finished,
}

#[derive(Component)] // str
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

#[derive(Component)] // str
struct Radius(f32);

#[derive(Component, Debug)] // str
struct Name(String);

#[derive(Component)]
struct Racer;

#[derive(Bundle)] // str
struct RacerBundle {
    racer: Racer,
    name: Name,
    position: Position,
    color: Color,
    radius: Radius,
}
#[derive(Event, PartialEq, Debug)]
struct RaceStartEvent;

#[derive(Event, PartialEq, Debug)]
struct RaceEndEvent(String);

fn spawn_racers(mut commands: Commands) {
    for i in 0..NUM_RACERS {
        let racer = RacerBundle {
            racer: Racer,
            name: Name(format!("Racer-{}", { i })),
            position: Position { x: 0.0, y: 0.0 },
            color: Color {
                r: 0.5,
                g: 0.0,
                b: 0.25,
                a: 1.0,
            },
            radius: Radius(2.0),
        };
        println!("Spawning racer: {:?}", racer.name.0);
        commands.spawn(racer);
    }
}

fn translate_racers_system(mut query: Query<(&mut Position, &Name), With<Racer>>) {
    for (mut position, name) in query.iter_mut() {
        position.x += 1.0;
        // position.y += 1.0;
        // println!("Updated position for {}: ({}, {})", name.0, position.x, position.y);
    }
}



// fn check_finish_system(
//     mut query: Query<(&Position, &Name), With<Racer>>,
//     mut state: ResMut<State<RaceState>>,
//     mut evt_race_end: EventWriter<RaceEndEvent>,
// ) {
//     for (position, name) in query.iter() {
//         if position.x >= 10.0 {
//             println!("{} has finished the race!", name.0);
//             evt_race_end.send(RaceEndEvent(name.0.clone()));
//         }
//     }
// }

fn start_race_system(
    mut state: ResMut<State<RaceState>>,
    mut evt_race_start: EventWriter<RaceStartEvent>,
) {
    println!("Race started!");
    evt_race_start.send(RaceStartEvent);
}

// fn reset_race_system(mut state: ResMut<State<RaceState>>, mut query: Query<&mut Position, With<Racer>>) {
//     for mut position in query.iter_mut() {
//         position.x = 0.0;
//     }
//     println!("Race reset!");
//     // state.set(RaceState::Ready).unwrap();
// }


// function to change the RaceState if the event is received
fn change_race_state(
    mut next_state: ResMut<NextState<RaceState>>,
    mut evt: EventReader<RaceStartEvent>,
) {
    next_state.set(RaceState::Racing);
}