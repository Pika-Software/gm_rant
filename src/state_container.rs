use std::{hash::{Hash, Hasher}, sync::{Mutex, Arc}, collections::HashMap};

use once_cell::sync::Lazy;

use crate::app_state::AppState;

#[derive(Clone, Copy, Debug)]
pub struct StateContainer(pub gmod::lua::State);

impl PartialEq for StateContainer {
    fn eq(&self, other: &StateContainer) -> bool { self.0.0 == other.0.0 }
}
impl Eq for StateContainer {}

impl std::ops::Deref for StateContainer {
    type Target = gmod::lua::State;
    fn deref(&self) -> &gmod::lua::State { &self.0 }
}

impl Hash for StateContainer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.0.hash(state);
    }
}

static mut STATES: Lazy<Mutex<HashMap<StateContainer, Arc<AppState>>>> = Lazy::new(|| { Mutex::new(HashMap::new()) });

pub fn add_state(lua: gmod::lua::State, state: Arc<AppState>) {
    unsafe { STATES.lock().unwrap().insert(StateContainer(lua), state.clone()); }
}

pub fn get_state(lua: gmod::lua::State) -> Option<Arc<AppState>> {
    unsafe { STATES.lock().unwrap().get(&StateContainer(lua)).cloned() }
}

pub fn remove_state(lua: gmod::lua::State) -> Option<Arc<AppState>> {
    unsafe { STATES.lock().unwrap().remove(&StateContainer(lua)) }
}
