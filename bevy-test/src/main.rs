use bevy::{
    app::{App, Startup, Update},
    ecs::query,
    prelude::{Commands, Component, Query, Res, ResMut, Resource, With},
    time::{Time, Timer, TimerMode},
    DefaultPlugins, MinimalPlugins,
};
use rand::random;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .insert_resource(MyTimer(Timer::from_seconds(0.01, TimerMode::Repeating)))
        .add_systems(Startup, add_racers)
        .add_systems(Update, update_racer)
        .run();
}

#[derive(Resource)]
struct MyTimer(Timer);

#[derive(Component)]
struct Racer {
    name: String,
    energy: u32,
    x: f32,
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
