use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Resource, Debug, Default)]
struct InspectorVisible(bool);

fn inspector_visible(visible: Res<InspectorVisible>) -> bool {
    visible.0
}

fn inspector_show_hide_system(
    mut visible: ResMut<InspectorVisible>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::F1) {
        visible.0 = !visible.0;
    }
}

pub struct InspectorPlugin;

impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((WorldInspectorPlugin::new().run_if(inspector_visible),))
            .insert_resource(InspectorVisible(false))
            .add_systems(First, (inspector_show_hide_system,));
    }
}
