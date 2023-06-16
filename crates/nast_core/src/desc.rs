use serde::Deserialize;
use std::collections::HashMap;

use crate::{
    builder::{Build, Builder},
    call, Node, Nodes, Res,
};

#[derive(Debug, Default, Deserialize)]
pub enum Content {
    #[default]
    Empty,
    Number(f64),
    Identifier(String),
}

#[derive(Debug, Deserialize)]
pub struct Desc {
    pub id: usize,
    pub name: String,
    #[serde(default)]
    pub inputs: HashMap<String, usize>,
    #[serde(default)]
    pub flows: HashMap<String, usize>,
    #[serde(default)]
    pub content: Content,
}

pub fn build(builder: Builder, nodes: &mut Nodes) -> Res<()> {
    let mut node = if let Ok(node) = Node::try_from(&builder.desc.name) {
        node
    } else {
        if builder
            .functions
            .iter()
            .find(|f| &builder.desc.name == &f.name)
            .is_some()
        {
            call()
        } else {
            return Err(format!(
                "Node \"{}\" is not supported in function \"{}\"",
                &builder.desc.name, &builder.function.name
            ));
        }
    };

    for (name, id) in &builder.desc.inputs {
        node.push_input(name, *id, &builder)?;
    }

    for (name, id) in &builder.desc.flows {
        node.push_flow(name, *id, &builder)?;
    }

    match builder.desc.content {
        Content::Empty => (),
        _ => node.push_content(&builder.desc.content, &builder)?,
    }

    match nodes.insert(builder.desc.id, node) {
        Some(_) => {
            return Err(format!(
                "Same node id {} used multiple times",
                builder.desc.id
            ))
        }
        None => (),
    }

    Ok(())
}
