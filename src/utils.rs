use rant::{runtime::RuntimeResult, RantValue};

pub unsafe fn push_rant_result(lua: gmod::lua::State, result: &RantValue) -> Result<(), &'static str> {
    lua.push_string(&result.to_string());
    Ok(())
}