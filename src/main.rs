use bevy::prelude::*;
use bevy_editor_pls::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(EditorPlugin)
    .run();
}
