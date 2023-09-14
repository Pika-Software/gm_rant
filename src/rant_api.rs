use rant::{RantValue, RantProgram};

use crate::app_state::AppState;

pub fn compile(state: &AppState, code: &str) -> Result<RantProgram, String> {
    let rant = state.rant.lock().map_err(|e| e.to_string())?;
    let program = rant.compile_quiet(code).map_err(|e| e.to_string())?;
    Ok(program)
}

pub fn compile_and_run(state: &AppState, code: &str) -> Result<RantValue, String> {
    let mut rant = state.rant.lock().map_err(|e| e.to_string())?;
    let program = rant.compile_quiet(&code).map_err(|e| e.to_string())?;
    let result = rant.run(&program).map_err(|e| e.to_string())?;
    Ok(result)
}