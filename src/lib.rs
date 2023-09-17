#![feature(c_unwind)]
use std::sync::{Arc, Mutex};

use rant::Rant;

use crate::app_state::AppState;

mod state_container;
mod app_state;
mod utils;
mod rant_api;

#[macro_use] extern crate gmod;

mod lua_api {
    use crate::{state_container::get_state, utils, rant_api, app_state::AppState};

    pub unsafe extern "C-unwind" fn __gc<T: Sized>(lua: gmod::lua::State) -> i32 {
        let userdata = lua.to_userdata(1) as *mut T;
        std::ptr::read(userdata);
        0
    }

    pub unsafe extern "C-unwind" fn rant_program_tostring<T: Sized>(lua: gmod::lua::State) -> i32 {
        let program = &*(lua.to_userdata(1) as *mut rant::RantProgram);
        let s = format!("RantProgram {:?}", program.name().or(program.path()).or(Some("<unnamed>")));
        lua.push_string(&s);
        1
    }

    pub unsafe fn is_program_userdata(lua: gmod::lua::State, state: &AppState, index: i32) -> bool {
        if lua.lua_type(index) != gmod::lua::LUA_TUSERDATA { return false; }
        if lua.get_metatable(index) == 0 { return false; }
        lua.from_reference(state.program_meta);
        let eq = lua.equal(-2, -1);
        lua.pop_n(2);
        eq
    }

    pub unsafe extern "C-unwind" fn rant_compile(lua: gmod::lua::State) -> i32 {
        let state = match get_state(lua) { Some(v) => v, None => return 0 };
        let code = lua.check_string(1);
        let program = match rant_api::compile(&state, &code) {
            Ok(v) => v,
            Err(e) => { lua.push_nil(); lua.push_string(&e.to_string()); return 2; },
        };

        lua.from_reference(state.program_meta);
        lua.new_userdata(program, Some(-1));
        1
    }

    unsafe fn rant_run_inner(lua: gmod::lua::State, state: &AppState) -> Result<rant::RantValue, anyhow::Error> {
        let args = utils::parse_lua_args(lua, 2)?;
        if lua.lua_type(1) == gmod::lua::LUA_TSTRING {
            let code = lua.get_string(1).unwrap();
            return rant_api::compile_and_run(&state, &code, args);
        } else if is_program_userdata(lua, &state, 1) {
            let program = &*(lua.to_userdata(1) as *mut rant::RantProgram);
            return rant_api::run(&state, program, args);
        } else {
            return Err(anyhow::anyhow!("Invalid argument #1: expected string or RantProgram"));
        }
    }

    pub unsafe extern "C-unwind" fn rant_run(lua: gmod::lua::State) -> i32 {
        let state = match get_state(lua) { Some(v) => v, None => return 0 };
        match rant_run_inner(lua, &state) {
            Ok(v) => {
                utils::push_rant_result(lua, &v).map_err(|e| lua.error(e.to_string())).ok();
                lua.push_nil();
            },
            Err(e) => {
                lua.push_nil();
                lua.push_string(&e.to_string());
            },
        };
        return 2;
    }
}

fn initialize_state(program_meta: gmod::lua::LuaReference) -> Arc<AppState> {
    let rant = Rant::new();

    Arc::new(AppState {
        rant: Mutex::new(rant),
        program_meta,
    })
}

#[gmod13_open]
unsafe extern "C-unwind" fn gmod13_open(lua: gmod::lua::State) -> i32 {
    gmod::gmcl::override_stdout();

    lua.new_metatable(lua_string!("RantProgram"));
    lua.push_function(lua_api::__gc::<rant::RantProgram>); lua.set_field(-2, lua_string!("__gc"));
    let program_meta = lua.reference();

    let state = initialize_state(program_meta);
    state_container::add_state(lua, state);
    
    lua.create_table(0, 0);
    lua.push_function(lua_api::rant_compile); lua.set_field(-2, lua_string!("compile"));
    lua.push_function(lua_api::rant_run); lua.set_field(-2, lua_string!("run"));
    lua.push_string(env!("CARGO_PKG_VERSION")); lua.set_field(-2, lua_string!("VERSION"));
    lua.set_global(lua_string!("rant"));
    0
}

#[gmod13_close]
fn gmod13_close(lua: gmod::lua::State) -> i32 {
    state_container::remove_state(lua);
    0
}