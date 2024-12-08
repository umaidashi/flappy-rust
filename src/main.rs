use bevy::prelude::*;
use plugin::MyPlugin;

mod constants;
mod plugin;

fn main() {
    App::new().add_plugins(MyPlugin).run();
}

