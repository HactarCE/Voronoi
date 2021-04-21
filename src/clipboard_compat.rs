//! Implementation of imgui::ClipboardBackend using the clipboard crate.

use clipboard::{ClipboardContext, ClipboardProvider};
use imgui;
use std::sync::Mutex;

lazy_static! {
    static ref CTX: Mutex<Option<ClipboardContext>> = Mutex::new(ClipboardProvider::new().ok());
}

pub fn clipboard_get() -> Option<String> {
    match CTX.lock().unwrap().as_mut() {
        None => {
            eprintln!("No clipboard provider");
            None
        }
        Some(ctx) => match ctx.get_contents() {
            Err(e) => {
                eprintln!("Error fetching clipboard contents: {}", e);
                None
            }
            Ok(s) => Some(s),
        },
    }
}

pub fn clipboard_set(new_contents: String) {
    match CTX.lock().unwrap().as_mut() {
        None => {
            eprintln!("No clipboard provider");
        }
        Some(ctx) => {
            if let Err(e) = ctx.set_contents(new_contents) {
                eprintln!("Error setting clipboard contents: {}", e);
            }
        }
    }
}

pub struct ClipboardCompat;

impl imgui::ClipboardBackend for ClipboardCompat {
    fn get(&mut self) -> Option<imgui::ImString> {
        clipboard_get().map(imgui::ImString::new)
    }
    fn set(&mut self, value: &imgui::ImStr) {
        clipboard_set(value.to_string());
    }
}
