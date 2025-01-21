// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use device_query::{DeviceQuery, DeviceState, Keycode};
//use std::sync::{Arc, Mutex};
//use std::thread;

fn main() {
    // Create a new DeviceState object to query the keyboard state
    let device_state = DeviceState::new();
    //let gui_open = Arc::new(Mutex::new(false));
    //let should_stop = Arc::new(Mutex::new(false));

    loop {
        // Get the current state of the keys
        let keys: Vec<Keycode> = device_state.get_keys();

        // Check if ALT and 's' keys are pressed simultaneously
        if keys.contains(&Keycode::LAlt) && keys.contains(&Keycode::S) {
            println!("Keys pressed");
            app_lib::run();
            println!("GUI closed")
        }
    }
}