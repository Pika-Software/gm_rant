use std::sync::Mutex;

use rant::Rant;

#[derive(Debug)]
pub struct AppState {
    pub rant: Mutex<Rant>,
}