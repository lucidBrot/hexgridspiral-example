use bevy::prelude::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (setup, serialize_my_struct, deserialize_my_struct).chain(),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    log::info!("set up.");
}

