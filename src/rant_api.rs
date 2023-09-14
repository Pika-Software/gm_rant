use rant::{RantValue, RantProgram};

use crate::app_state::AppState;

pub fn compile(state: &AppState, code: &str) -> Result<RantProgram, anyhow::Error> {
    let rant = state.rant.lock().map_err(|e| anyhow::anyhow!(".rant mutex is poisoned"))?;
    let program = rant.compile_quiet(code)?;
    Ok(program)
}

pub fn run(state: &AppState, program: &RantProgram) -> Result<RantValue, anyhow::Error> {
    let mut rant = state.rant.lock().map_err(|e| anyhow::anyhow!(".rant mutex is poisoned"))?;
    let result = rant.run(program)?;
    Ok(result)
}

pub fn compile_and_run(state: &AppState, code: &str) -> Result<RantValue, anyhow::Error> {
    return run(state, &compile(state, code)?);
}