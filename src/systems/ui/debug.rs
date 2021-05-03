use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

const DEBUG_FONT_SIZE: f32 = 15.0;
const DEBUG_KEY_FONT: &str = "fonts/FiraSans-Bold.ttf";
const DEBUG_KEY_FONT_COLOR: Color = Color::WHITE;
const DEBUG_VALUE_FONT: &str = "fonts/FiraMono-Medium.ttf";
const DEBUG_VALUE_FONT_COLOR: Color = Color::WHITE;

#[derive(Default)]
pub struct DebugPlugin {}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default());
        app.add_startup_system(init.system());
        app.add_system(debug_update_system.system());
        app.add_system(text_color_system.system());
    }
}

// A unit struct to help identify the FPS UI component, since there may be many Text components
pub struct FpsText;

// A unit struct to help identify the color-changing Text component
pub struct ColorText;

fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    text_color_spawn(&mut commands, &asset_server);
    spawn_debug(&mut commands, &asset_server);
}

fn spawn_debug(commands: &mut Commands, asset_server: &Res<AssetServer>) {
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

fn text_color_spawn(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            // Use the `Text::with_section` constructor
            text: Text::with_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "hello\nbevy!",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 100.0,
                    color: Color::WHITE,
                },
                // Note: You can use `Default::default()` in place of the `TextAlignment`
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..Default::default()
                },
            ),
            ..Default::default()
        })
        .insert(ColorText);
}

fn text_color_system(time: Res<Time>, mut query: Query<&mut Text, With<ColorText>>) {
    for mut text in query.iter_mut() {
        let seconds = time.seconds_since_startup() as f32;
        // We used the `Text::with_section` helper method, but it is still just a `Text`,
        // so to update it, we are still updating the one and only section
        text.sections[0].style.color = Color::Rgba {
            red: (1.25 * seconds).sin() / 2.0 + 0.5,
            green: (0.75 * seconds.clone()).sin() / 2.0 + 0.5,
            blue: (0.50 * seconds.clone()).sin() / 2.0 + 0.5,
            alpha: 1.0,
        };
    }
}
