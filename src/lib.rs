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
    use std::sync::Arc;
    use crate::{state_container::get_state, utils, rant_api};

    #[derive(Debug)]
    pub struct Test(u32);
    impl Drop for Test {
        fn drop(&mut self) {
            println!("dropped test");
        }
    }

    pub(crate) unsafe extern "C-unwind" fn __gc<T: Sized>(lua: gmod::lua::State) -> i32 {
        let userdata = lua.to_userdata(1) as *mut T;
        std::ptr::read(userdata);
        0
    }

    pub unsafe extern "C-unwind" fn rant_compile(lua: gmod::lua::State) -> i32 {
        let state = match get_state(lua) { Some(v) => v, None => return 0 };
        let code = lua.check_string(1);
        let program = match rant_api::compile(&state, &code) {
            Ok(v) => v,
            Err(e) => { lua.push_nil(); lua.push_string(&e); return 2; },
        };

        lua.from_reference(state.program_meta);
        lua.new_userdata( Test(123), Some(-1));
        1

        // let a = lua.check_userdata(1, lua_string!("RantProgram")).read().data.cast::<Arc<RantProgram>>().read();
    }

    pub unsafe extern "C-unwind" fn rant_test(lua: gmod::lua::State) -> i32 {
        let userdata = lua.to_userdata(1) as *mut Test;
        let data = &*userdata;
        println!("test: {:#?}", data);
        0
    }

    pub unsafe extern "C-unwind" fn rant_run(lua: gmod::lua::State) -> i32 {
        let state = match get_state(lua) { Some(v) => v, None => return 0 };
        if lua.lua_type(1) == gmod::lua::LUA_TSTRING {
            let code = lua.get_string(1).unwrap();
            match rant_api::compile_and_run(&state, &code) {
                Ok(v) => {
                    lua.push_nil();
                    utils::push_rant_result(lua, &v).map_err(|e| lua.error(e)).ok();
                },
                Err(e) => {
                    lua.push_string(&e);
                    lua.push_nil();
                },
            };
            return 2;
        }
        0
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
    lua.push_function(lua_api::__gc::<lua_api::Test>); lua.set_field(-2, lua_string!("__gc"));
    let program_meta = lua.reference();

    let state = initialize_state(program_meta);
    state_container::add_state(lua, state);
    
    lua.create_table(0, 0);
    lua.push_function(lua_api::rant_compile); lua.set_field(-2, lua_string!("compile"));
    lua.push_function(lua_api::rant_run); lua.set_field(-2, lua_string!("run"));
    lua.push_function(lua_api::rant_test); lua.set_field(-2, lua_string!("test"));
    lua.set_global(lua_string!("rant"));
    0
}

#[gmod13_close]
fn gmod13_close(lua: gmod::lua::State) -> i32 {
    state_container::remove_state(lua);
    0
}