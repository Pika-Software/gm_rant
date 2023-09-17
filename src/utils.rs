use std::{ops::Deref, collections::HashMap};

use rant::{runtime::RuntimeResult, RantValue, RantListHandle, RantList, RantMap};

unsafe fn push_rant_list(lua: gmod::lua::State, list: &Vec<RantValue>) -> Result<(), anyhow::Error> {
    lua.create_table(list.len() as i32, 0);
    for (i, item) in list.iter().enumerate() {
        push_rant_result(lua, item)?;
        lua.raw_seti(-2, i as i32);
    }
    Ok(())
}

unsafe fn push_rant_map(lua: gmod::lua::State, map: &RantMap) -> Result<(), anyhow::Error> {
    lua.create_table(0, map.raw_len() as i32);
    for key in map.raw_keys().iter().map(|v| v.to_string() ) {
        lua.push_string(&key);
        if let Some(value) = map.raw_get(&key) {
            push_rant_result(lua, value)?;
        } else {
            lua.push_nil();
        }
        lua.set_table(-3);
    }
    Ok(())
}

pub unsafe fn push_rant_result(lua: gmod::lua::State, result: &RantValue) -> Result<(), anyhow::Error> {
    match result {
        RantValue::String(v) => lua.push_string(v.as_str()),
        RantValue::Float(v) => lua.push_number(*v),
        RantValue::Int(v) => lua.push_integer((*v).try_into()?),
        RantValue::Boolean(v) => lua.push_boolean(*v),
        RantValue::List(v) => push_rant_list(lua, &v.borrow())?,
        RantValue::Tuple(v) => push_rant_list(lua, v)?,
        RantValue::Map(v) => push_rant_map(lua, &v.borrow())?,
        RantValue::Range(v) => push_rant_list(lua, &v.to_rant_list())?,
        // RantValue::Selector(_) => todo!(),
        RantValue::Nothing => lua.push_nil(),
        other => lua.push_string(other.to_string().as_str()),
    };
    Ok(())
}

unsafe fn parse_lua_value(lua: gmod::lua::State, stack_pos: i32) -> Result<RantValue, anyhow::Error> {
    let value = match lua.lua_type(stack_pos) {
        gmod::lua::LUA_TSTRING => RantValue::String(lua.get_string(stack_pos).unwrap().to_string().into()),
        gmod::lua::LUA_TNUMBER => RantValue::Float(lua.to_number(stack_pos)),
        gmod::lua::LUA_TBOOLEAN => RantValue::Boolean(lua.get_boolean(stack_pos)),
        gmod::lua::LUA_TTABLE => {
            if lua.len(stack_pos) > 0 {
                let mut list = RantList::new();
                for i in 0..lua.len(stack_pos) {
                    lua.raw_geti(stack_pos, i + 1);
                    list.push(parse_lua_value(lua, -1)?);
                    lua.pop();
                }
                RantValue::List(list.into())
            } else {
                let mut map = RantMap::new();
                lua.push_nil();
                while lua.next(if stack_pos < 0 {stack_pos-1} else {stack_pos}) != 0 {
                    if let Some(key) = lua.get_string(-2) {
                        map.raw_set(&key, parse_lua_value(lua, -1)?)
                    }
                    lua.pop();
                }
                RantValue::Map(map.into())
            }
        }
        _ => RantValue::Nothing,
    };
    Ok(value)
}

pub unsafe fn parse_lua_args(lua: gmod::lua::State, stack_pos: i32) -> Result<Option<HashMap<String, RantValue>>, anyhow::Error> {
    if lua.lua_type(stack_pos) != gmod::lua::LUA_TTABLE { return Ok(None); }
    let mut args = HashMap::new();
    lua.push_nil();
    while lua.next(if stack_pos < 0 {stack_pos-1} else {stack_pos}) != 0 {
        if let Some(key) = lua.get_string(-2) {
            args.insert(key.to_string(), parse_lua_value(lua, -1)?);
        }
        lua.pop();
    }

    Ok(Some(args))
}