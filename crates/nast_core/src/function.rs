use crate::{builder::Builder, desc, Desc, Nodes, Res};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Function {
    pub name: String,
    #[serde(default)]
    pub arguments: Vec<String>,
    pub start: usize,
    pub descs: Vec<Desc>,
    #[serde(default)]
    pub description: String,
}

pub type Functions = HashMap<String, Function>;

pub fn build(function: &Function, functions: &Functions, nodes: &mut Nodes) -> Res<()> {
    // TODO: More validation checks.

    for desc in function.descs.iter() {
        let builder = Builder {
            desc,
            function,
            functions,
        };
        desc::build(builder, nodes)?;
    }

    Ok(())
}
