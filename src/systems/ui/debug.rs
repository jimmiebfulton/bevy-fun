use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum DebugUIState {
    Enabled,
    Disabled,
}

impl Default for DebugUIState {
    fn default() -> Self {
        DebugUIState::Disabled
    }
}

const DEBUG_FONT_SIZE: f32 = 15.0;
const DEBUG_KEY_FONT: &str = "fonts/FiraSans-Bold.ttf";
const DEBUG_KEY_FONT_COLOR: Color = Color::WHITE;
const DEBUG_VALUE_FONT: &str = "fonts/FiraMono-Medium.ttf";
const DEBUG_VALUE_FONT_COLOR: Color = Color::WHITE;

#[derive(Default)]
pub struct DebugUIPlugin {
    pub start_state: DebugUIState,
}

impl Plugin for DebugUIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(self.start_state.clone());
        app.add_plugin(FrameTimeDiagnosticsPlugin::default());
        app.add_system(debug_state.system());
        app.add_system_set(
            SystemSet::on_enter(DebugUIState::Enabled).with_system(spawn_debug.system()),
        );
        app.add_system_set(
            SystemSet::on_enter(DebugUIState::Disabled).with_system(despawn_debug.system()),
        );
        app.add_system_set(
            SystemSet::on_update(DebugUIState::Enabled).with_system(debug_update_system.system()),
        );
    }
}

// A unit struct to help identify the FPS UI component, since there may be many Text components
pub struct FpsText;

fn debug_state(mut keys: ResMut<Input<KeyCode>>, mut app_state: ResMut<State<DebugUIState>>) {
    if keys.pressed(KeyCode::LWin) && keys.just_pressed(KeyCode::D) {
        match app_state.current() {
            DebugUIState::Enabled => {
                app_state
                    .set(DebugUIState::Disabled)
                    .expect("Failed to disable Debug UI");
            }
            DebugUIState::Disabled => {
                app_state
                    .set(DebugUIState::Enabled)
                    .expect("Failed to enable Debug UI");
            }
        }

        keys.reset(KeyCode::LWin);
        keys.reset(KeyCode::D)
    }
}

fn despawn_debug(mut commands: Commands, mut query: Query<(Entity, &Text), With<FpsText>>) {
    if let Ok((id, _text)) = query.single_mut() {
        commands.entity(id).despawn();
    }
}

fn spawn_debug(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Rich text with multiple sections
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    TextSection {
                        value: "FPS: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load(DEBUG_KEY_FONT),
                            font_size: DEBUG_FONT_SIZE,
                            color: DEBUG_KEY_FONT_COLOR,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load(DEBUG_VALUE_FONT),
                            font_size: DEBUG_FONT_SIZE,
                            color: DEBUG_VALUE_FONT_COLOR,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(FpsText);
}

fn debug_update_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                // Update the value of the second section
                text.sections[1].value = format!("{:.2}", average);
            }
        }
    }
}
