use crate::{Build, Builder, Content, Eval, Res, State};

#[derive(Default, Debug)]
pub struct Argument(usize);

impl Eval for Argument {
    fn eval(&self, state: &mut State) -> Res<()> {
        debug_assert_eq!(state.mark(), 0);
        state.pop_state()?;
        Ok(state.load_argument(self.0))
    }
}

impl Build for Argument {
    fn push_content(&mut self, content: &Content, builder: &Builder) -> Res<()> {
        match content {
            Content::Identifier(name) => {
                if let Some((index, _)) = builder
                    .function
                    .arguments
                    .iter()
                    .enumerate()
                    .find(|(_, n)| n == &name)
                {
                    Ok(self.0 = index)
                } else {
                    Err(format!(
                        "Argument \"{}\" not exist for node \"{}\" in function \"{}\"",
                        name, builder.desc.name, builder.function.name
                    ))
                }
            }
            _ => Err(format!(
                "Expected \"Identifier\" content for node \"{}\" in function \"{}\"",
                builder.desc.name, builder.function.name
            )),
        }
    }
}
