use device_query::{DeviceQuery, DeviceState, Keycode};
use get_selected_text::get_selected_text;
use std::{thread, time::Duration};

fn main() {
    let device_state = DeviceState::new();

    println!("Listening for key presses. Press ESC to exit.");

    loop {
        let keys = device_state.get_keys();

        if keys.contains(&Keycode::Command)
            && keys.contains(&Keycode::F)
            && keys.contains(&Keycode::LOption)
            && keys.contains(&Keycode::LShift)
        {
            match get_selected_text() {
                Ok(selected_text) => {
                    println!("selected text: {}", selected_text);
                }
                Err(..) => {
                    println!("error occurred while getting the selected text");
                }
            }
        }

        // Exit if ESC is pressed
        if keys.contains(&Keycode::Escape) {
            println!("ESC pressed. Exiting.");
            break;
        }

        thread::sleep(Duration::from_millis(100));
    }
}
