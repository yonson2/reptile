use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("John Doe".into())));
}

fn greet_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    people: Query<&Name, With<Person>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &people {
            println!("Hello, {}!", name.0);
        }
    }
}

fn update_people(mut people: Query<&mut Name, With<Person>>) {
    for mut name in &mut people {
        if name.0 == "John Doe" {
            name.0 = "Joahnna Doa".to_string();
            break;
        }
    }
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_people)
            .add_systems(Update, (update_people, greet_people).chain());
    }
}

#[derive(Resource)]
struct GreetTimer(Timer);
