mod builder;
mod desc;
mod function;
mod node;
mod nodes;
mod state;
mod value;

#[cfg(test)]
mod tests;

use std::collections::HashMap;

pub use desc::*;
pub use function::*;
pub use node::*;
pub use state::*;
pub use value::*;

use builder::*;

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

pub fn build(functions: &Functions, nodes: &mut Nodes) -> Res<()> {
    for function in functions.values() {
        function::build(function, functions, nodes)?;
    }
    Ok(())
}

pub fn check(functions: &Functions, nodes: &Nodes) -> Res<()> {
    for function in functions.values() {
        if !nodes.contains_key(&function.start) {
            return Err(format!(
                "Invalid start ID for function \"{}\"",
                &function.name
            ));
        }
        for desc in &function.descs {
            for (name, input) in &desc.inputs {
                if !nodes.contains_key(&input) {
                    return Err(format!(
                        "Invalid input ID for function \"{}\", node ID: {}, input: \"{}\"",
                        &function.name, desc.id, name,
                    ));
                }
            }
            for (name, flow) in &desc.flows {
                if !nodes.contains_key(&flow) {
                    return Err(format!(
                        "Invalid flow ID for function \"{}\", node ID: {}, flow: \"{}\"",
                        &function.name, desc.id, name,
                    ));
                }
            }
        }
    }
    Ok(())
}
