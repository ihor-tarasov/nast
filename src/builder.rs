use crate::{Desc, Function, Functions, Res, Content};

pub struct Builder<'a> {
    pub desc: &'a Desc,
    pub function: &'a Function,
    pub functions: &'a Functions,
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
