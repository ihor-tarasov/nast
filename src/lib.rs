pub mod utils;

mod node;
mod state;
mod value;
mod nodes;

#[cfg(test)]
mod tests;

pub use node::*;
pub use state::*;
pub use value::*;

use utils::IDMap;

pub fn step(nodes: &IDMap<Node>, state: &mut State) -> Res<bool> {
    let id = state.id();
    match nodes.get(id).unwrap().eval(state) {
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

pub fn run(nodes: &IDMap<Node>, state: &mut State) -> Res<Value> {
    while step(nodes, state)? {}
    Ok(state.get_value())
}
