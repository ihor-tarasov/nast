mod node;
mod state;
mod value;
mod nodes;
mod build;

#[cfg(test)]
mod tests;

use std::collections::HashMap;

pub use node::*;
pub use state::*;
pub use value::*;
pub use build::*;

pub fn step(nodes: &HashMap<usize, Node>, state: &mut State) -> Res<bool> {
    let id = state.id();
    match nodes.get(&id).unwrap().eval(state) {
        Ok(_) => Ok(true),
        Err(e) => {
            if e.is_empty() {
                Ok(false)
            } else {
                Err(e)
            }
        }
    }
}

pub fn run(nodes: &HashMap<usize, Node>, state: &mut State) -> Res<Value> {
    while step(nodes, state)? {}
    Ok(state.get_value())
}
