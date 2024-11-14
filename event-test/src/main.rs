use bevy::{prelude::*, state::app::StatesPlugin};
use rand::*;

fn main() {
    App::new()
    .add_plugins((MinimalPlugins, StatesPlugin))
        .insert_state::<AppState>(AppState::Loading)
        .add_event::<StateChangeEvent>()
        .add_systems(Update, (event_logger, event_changer))
        .run();
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone)]
enum AppState {
    Loading,
    Running,
    Exiting,
}

#[derive(Event)]
struct StateChangeEvent(AppState);


fn event_changer(mut evt: EventWriter<StateChangeEvent>) {
    let mut rnd = rand::thread_rng();
    if rnd.gen_bool(0.0005) {
        let new_state = match rnd.gen_range(0..3) {
            0 => AppState::Loading,
            1 => AppState::Running,
            2 => AppState::Exiting,
            _ => unreachable!(),
        };
        evt.send(StateChangeEvent(new_state));
    }
}

fn event_logger(mut events: EventReader<StateChangeEvent>) {
    for state_change in events.read() {
        println!("AppState --> {:?}", state_change.0);
    }
}
