use bevy::diagnostic::{Diagnostics, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum DiagnosticsUIState {
    Enabled,
    Disabled,
}

impl Default for DiagnosticsUIState {
    fn default() -> Self {
        DiagnosticsUIState::Disabled
    }
}

const DIAGNOSTICS_FONT_SIZE: f32 = 15.0;
const DIAGNOSTICS_KEY_FONT: &str = "fonts/FiraSans-Bold.ttf";
const DIAGNOSTICS_KEY_FONT_COLOR: Color = Color::WHITE;
const DIAGNOSTICS_VALUE_FONT: &str = "fonts/FiraMono-Medium.ttf";
const DIAGNOSTICS_VALUE_FONT_COLOR: Color = Color::WHITE;

#[derive(Default)]
pub struct DiagnosticsUIPlugin {
    pub start_state: DiagnosticsUIState,
}

impl Plugin for DiagnosticsUIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(self.start_state.clone());
        app.add_plugin(FrameTimeDiagnosticsPlugin::default());
        app.add_plugin(EntityCountDiagnosticsPlugin::default());
        app.add_system(diagnostics_display_state.system());
        app.add_system_set(
            SystemSet::on_enter(DiagnosticsUIState::Enabled)
                .with_system(spawn_diagnostics.system()),
        );
        app.add_system_set(
            SystemSet::on_enter(DiagnosticsUIState::Disabled)
                .with_system(despawn_diagnostics.system()),
        );
        app.add_system_set(
            SystemSet::on_update(DiagnosticsUIState::Enabled)
                .with_system(update_diagnostics_system.system()),
        );
    }
}

// Debug Text
pub struct DiagnosticsText;

fn diagnostics_display_state(
    mut keys: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<DiagnosticsUIState>>,
) {
    if keys.pressed(KeyCode::LWin) && keys.just_pressed(KeyCode::D) {
        match app_state.current() {
            DiagnosticsUIState::Enabled => {
                app_state
                    .set(DiagnosticsUIState::Disabled)
                    .expect("Failed to disable Diagnostics UI");
            }
            DiagnosticsUIState::Disabled => {
                app_state
                    .set(DiagnosticsUIState::Enabled)
                    .expect("Failed to enable Diagnostics UI");
            }
        }

        keys.reset(KeyCode::LWin);
        keys.reset(KeyCode::D)
    }
}

fn despawn_diagnostics(
    mut commands: Commands,
    mut query: Query<(Entity, &Text), With<DiagnosticsText>>,
) {
    if let Ok((id, _text)) = query.single_mut() {
        commands.entity(id).despawn();
    }
}

fn spawn_diagnostics(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                            font: asset_server.load(DIAGNOSTICS_KEY_FONT),
                            font_size: DIAGNOSTICS_FONT_SIZE,
                            color: DIAGNOSTICS_KEY_FONT_COLOR,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load(DIAGNOSTICS_VALUE_FONT),
                            font_size: DIAGNOSTICS_FONT_SIZE,
                            color: DIAGNOSTICS_VALUE_FONT_COLOR,
                        },
                    },
                    TextSection {
                        value: " / Frame: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load(DIAGNOSTICS_KEY_FONT),
                            font_size: DIAGNOSTICS_FONT_SIZE,
                            color: DIAGNOSTICS_KEY_FONT_COLOR,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load(DIAGNOSTICS_VALUE_FONT),
                            font_size: DIAGNOSTICS_FONT_SIZE,
                            color: DIAGNOSTICS_VALUE_FONT_COLOR,
                        },
                    },
                    TextSection {
                        value: " / Entities: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load(DIAGNOSTICS_KEY_FONT),
                            font_size: DIAGNOSTICS_FONT_SIZE,
                            color: DIAGNOSTICS_KEY_FONT_COLOR,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load(DIAGNOSTICS_VALUE_FONT),
                            font_size: DIAGNOSTICS_FONT_SIZE,
                            color: DIAGNOSTICS_VALUE_FONT_COLOR,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(DiagnosticsText);
}

fn update_diagnostics_system(
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<DiagnosticsText>>,
) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                // Update the value of the second section
                text.sections[1].value = format!("{:.2}", average);
            }
        }
        if let Some(frame_time) = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME) {
            if let Some(value) = frame_time.value() {
                // Update the value of the second section
                text.sections[3].value = format!("{:.6}{}", value, frame_time.suffix);
            }
        }
        if let Some(entity_count) = diagnostics.get(EntityCountDiagnosticsPlugin::ENTITY_COUNT) {
            if let Some(count) = entity_count.value() {
                // Update the value of the second section
                text.sections[5].value = format!("{}", count);
            }
        }
    }
}
