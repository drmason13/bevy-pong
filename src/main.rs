use bevy::prelude::*;

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(Game)
        .run();
}

// --------------------
// Plugins
// --------------------

pub struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(setup.system())
            .add_resource(GameTimer(Timer::from_seconds(2.0)))
            .add_system(greet_people.system());
    }
}

// --------------------
// Resources
// --------------------

struct GameTimer(Timer);

// --------------------
// Components
// --------------------

struct Name(String);

struct Person;

// --------------------
// Startup Systems
// --------------------

fn setup(mut commands: Commands) {
    commands
        .spawn((Person, Name("Elaina Proctor".to_string())))
        .spawn((Person, Name("Renzo Hume".to_string())))
        .spawn((Person, Name("Zayna Nieves".to_string())));
}

// --------------------
// Runtime Systems
// --------------------

fn greet_people(time: Res<Time>, mut timer: ResMut<GameTimer>, mut query: Query<(&Person, &Name)>) {
    // update our timer with the time elapsed since the last update
    timer.0.tick(time.delta_seconds);

    // check to see if the timer has finished. if it has, print our message
    if timer.0.finished {
        for (_person, name) in &mut query.iter() {
            println!("hello {}!", name.0);
        }
        // reset the timer. it will start counting from 0
        timer.0.reset();
    }
}
