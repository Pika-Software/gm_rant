#![feature(c_unwind)]
use std::sync::{Arc, Mutex};

use rant::Rant;

use crate::app_state::AppState;

mod state_container;
mod app_state;
mod utils;

#[macro_use] extern crate gmod;

mod lua_api {
    use crate::{state_container::get_state, utils};

    pub unsafe extern "C-unwind" fn rant_run(lua: gmod::lua::State) -> i32 {
        let state = get_state(lua).unwrap();
        if lua.lua_type(1) == gmod::lua::LUA_TSTRING {
            let code = lua.get_string(1).unwrap();
            let mut rant = state.rant.lock().unwrap();
            let program = rant.compile_quiet(&code).map_err(|e| lua.error(e.to_string())).unwrap();
            let result = rant.run(&program).map_err(|e| lua.error(e.to_string())).unwrap();
            utils::push_rant_result(lua, &result).map_err(|e| lua.error(e)).ok();
            return 1;
        }
        0
    }
}

fn initialize_state() -> Arc<AppState> {
    let mut rant = Rant::new();

    Arc::new(AppState {
        rant: Mutex::new(rant),
    })
}

#[gmod13_open]
unsafe extern "C-unwind" fn gmod13_open(lua: gmod::lua::State) -> i32 {
    gmod::gmcl::override_stdout();

    let state = initialize_state();
    state_container::add_state(lua, state);
    
    println!("Hello from binary module! {:?}", lua.0);
    lua.create_table(0, 0);
    lua.push_function(lua_api::rant_run); lua.set_field(-2, lua_string!("run"));
    lua.set_global(lua_string!("rant"));
    0
}

#[gmod13_close]
fn gmod13_close(lua: gmod::lua::State) -> i32 {
    println!("Goodbye from binary module!");
    state_container::remove_state(lua);
    0
}