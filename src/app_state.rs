use std::sync::Mutex;

use rant::Rant;

#[derive(Debug)]
pub struct AppState {
    pub rant: Mutex<Rant>,
    pub program_meta: gmod::lua::LuaReference,
}