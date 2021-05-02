use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;

pub struct InputsPlugin;

impl Plugin for InputsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(keyboard_inputs.system());
        app.add_system(keyboard_events.system());
        app.add_system(mouse_inputs.system());
        app.add_system(mouse_events.system());
    }
}

fn keyboard_inputs(keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Key1) {
        println!("One pressed!");
    }

    if keys.just_released(KeyCode::Key2) {
        println!("Two released!");
    }

    if keys.pressed(KeyCode::Key3) {
        println!("Three is currently pressed!");
    }
}

fn keyboard_events(mut key_events: EventReader<KeyboardInput>) {
    use bevy::input::ElementState;

    for event in key_events.iter() {
        match event.state {
            ElementState::Pressed => {
                println!("Key press: {:?} ({})", event.key_code, event.scan_code);
            }
            ElementState::Released => {
                println!("Key release: {:?} ({})", event.key_code, event.scan_code);
            }
        }
    }
}

fn mouse_inputs(buttons: Res<Input<MouseButton>>) {
    if buttons.just_pressed(MouseButton::Left) {
        println!("Left button pressed!");
    }

    if buttons.just_released(MouseButton::Right) {
        println!("Right button release!");
    }

    if buttons.pressed(MouseButton::Middle) {
        println!("Middle button held!");
    }
}

fn mouse_events(mut button_events: EventReader<MouseButtonInput>) {
    use bevy::input::ElementState;

    for event in button_events.iter() {
        match event.state {
            ElementState::Pressed => {
                println!("Button press: {:?}", event.button);
            }
            ElementState::Released => {
                println!("Button release: {:?}", event.button);
            }
        }
    }
}
