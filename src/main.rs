use rdev::{listen, Event, EventType, Key, simulate};
use EventType::{KeyPress, KeyRelease};
use std::{thread, time};

fn main() {
	// This will block.
	if let Err(error) = listen(callback) {
	    println!("Error: {:?}", error)
	}

	fn callback(event: Event) {
	    match event.event_type {
	         KeyRelease(key) => {
	        	if key == Key::Alt {
	        		send(&KeyPress(Key::Alt));
	        		send(&KeyPress(Key::Tab));
	        		send(&KeyRelease(Key::Tab));
	        		// send(&KeyPress(Key::Tab));
	        		// send(&KeyRelease(Key::Tab));
	        		send(&KeyRelease(Key::Alt));
	        	}
	        },
	        _ => (),
	    }
	}
}

fn send(event_type: &EventType) {
    // 	let delay = time::Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    // // Let ths OS catchup (at least MacOS)
    // thread::sleep(delay);
}

