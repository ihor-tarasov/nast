use serde::Deserialize;

use crate::{Res, Node, call};

use std::collections::HashMap;

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

pub trait Build {
    fn push_input(&mut self, name: &String, _id: usize, builder: &Builder) -> Res<()> {
        Err(format!("Incompatible input name \"{}\" for node \"{}\" in function \"{}\"", name, builder.desc.name, builder.function.name))
    }

    fn push_flow(&mut self, name: &String, _id: usize, builder: &Builder) -> Res<()> {
        Err(format!("Incompatible flow name \"{}\" for node \"{}\" in function \"{}\"", name, builder.desc.name, builder.function.name))
    }

    fn push_content(&mut self, content: &Content, builder: &Builder) -> Res<()> {
        Err(format!("Incompatible content \"{:?}\" for node \"{}\" in function \"{}\"", content, builder.desc.name, builder.function.name))
    }
}

#[derive(Debug, Deserialize)]
pub struct Function {
    pub name: String,
    #[serde(default)]
    pub arguments: Vec<String>,
    pub start: usize,
    pub descs: Vec<Desc>,
}

pub type Functions = HashMap<String, Function>;
pub type Nodes = HashMap<usize, Node>;

pub struct Builder<'a> {
    pub desc: &'a Desc,
    pub function: &'a Function,
    pub functions: &'a Functions,
}

fn build_desc(builder: Builder, nodes: &mut Nodes) -> Res<()> {
    let mut node = if let Ok(node) = Node::try_from(&builder.desc.name) {
        node
    } else {
        if builder.functions.contains_key(&builder.desc.name) {
            call()
        } else {
            return Err(format!("Node \"{}\" is not supported in function \"{}\"", &builder.desc.name, &builder.function.name))
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
        Some(_) => return Err(format!("Same node id {} used multiple times", builder.desc.id)),
        None => (),
    }

    Ok(())
}

fn build_function(function: &Function, functions: &Functions, nodes: &mut Nodes) -> Res<()> {
    // TODO: More validation checks.

    for desc in function.descs.iter() {
        let builder = Builder {
            desc,
            function,
            functions,
        };
        build_desc(builder, nodes)?;
    }

    Ok(())
}

pub fn build(functions: &Functions, nodes: &mut Nodes) -> Res<()> {
    for function in functions.values() {
        build_function(function, functions, nodes)?;
    }
    Ok(())
}
