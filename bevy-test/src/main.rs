use bevy::{
    app::{App, Startup, Update},
    ecs::query,
    prelude::*,
    state::app::StatesPlugin,
    time::{Time, Timer, TimerMode},
    DefaultPlugins, MinimalPlugins,
};
use rand::random;

fn main() {
    App::new()
        .add_plugins((MinimalPlugins, StatesPlugin))
        .init_state::<RaceState>() // because I'm using defaults otherwise insert_state
        .insert_resource(MyTimer(Timer::from_seconds(0.05, TimerMode::Repeating)))
        .insert_resource(AppState {current_race_state: RaceState::Finished, did_print: false})
        .add_systems(Startup, (add_racers, change_app_state))
        .add_systems(Update, (update_racer.run_if(in_state(RaceState::Racing))))
        .add_systems(OnEnter(RaceState::Waiting), log_state)
        .run();
}

#[derive(Resource)]
struct AppState {
    current_race_state: RaceState,
    did_print: bool,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum RaceState {
    #[default]
    Waiting,
    Racing,
    Finished,
}

#[derive(Resource)]
struct MyTimer(Timer);

#[derive(Component)]
struct Racer {
    name: String,
    energy: u32,
    x: f32,
}

fn change_app_state(mut appState: ResMut<AppState>) {
    appState.current_race_state = RaceState::Waiting;
}

fn add_racers(mut commands: Commands) {
    let racer = Racer {
        name: "Bleem".to_string(),
        energy: 100,
        x: 0.0,
    };
    let racer2 = Racer {
        name: "Flonk".to_string(),
        energy: 109,
        x: 0.0,
    };
    commands.spawn(racer);
    commands.spawn(racer2);
}

fn update_racer(time: Res<Time>, mut my_timer: ResMut<MyTimer>, mut racers: Query<&mut Racer>) {
    if my_timer.0.tick(time.delta()).just_finished() {
        for mut racer in &mut racers {
            consume_energy(&mut racer);
            move_racer(&mut racer);
        }
    }
}

fn consume_energy(racer: &mut Racer) {
    let has_energy = racer.energy > 0;
    let reached_finish = racer.x == 105.0;
    if has_energy & !reached_finish {
        racer.energy = racer.energy - 1;
        // println!("{}'s energy: {}", racer.name, racer.energy);
    }
}

fn move_racer(racer: &mut Racer) {
    let has_energy = racer.energy > 0;
    let reached_finish = racer.x == 105.0;
    if has_energy & !reached_finish {
        racer.x += 1.0;
        println!("{}'s x: {}", racer.name, racer.x);
    }
}

fn log_state(appState: Res<AppState>) {
    println!("The Current State: {:?}", appState.current_race_state);
}
