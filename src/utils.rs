use rant::{runtime::RuntimeResult, RantValue};

pub unsafe fn push_rant_result(lua: gmod::lua::State, result: &RantValue) -> Result<(), anyhow::Error> {
    match result {
        RantValue::String(v) => lua.push_string(v.as_str()),
        RantValue::Float(v) => lua.push_number(*v),
        RantValue::Int(v) => lua.push_integer((*v).try_into()?),
        RantValue::Boolean(v) => lua.push_boolean(*v),
        RantValue::List(_) => todo!(),
        // RantValue::Tuple(_) => todo!(),
        // RantValue::Map(_) => todo!(),
        // RantValue::Range(_) => todo!(),
        // RantValue::Selector(_) => todo!(),
        RantValue::Nothing => lua.push_nil(),
        other => lua.push_string(other.to_string().as_str()),
    }
    Ok(())
}