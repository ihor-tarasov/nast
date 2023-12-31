use crate::{Build, Eval, Res, State};

#[derive(Default, Debug)]
pub struct Start(usize);

impl Eval for Start {
    fn eval(&self, state: &mut State) -> Res<()> {
        debug_assert_eq!(state.mark(), 0);
        Ok(state.set_state(self.0, 0))
    }
}

impl Build for Start {
    fn push_flow(&mut self, name: &String, id: usize, builder: &crate::Builder) -> Res<()> {
        match name.as_str() {
            "flow" => Ok(self.0 = id),
            _ => Err(format!(
                "Incompatible flow name \"{}\" for node \"{}\" in function \"{}\"",
                name, builder.desc.name, builder.function.name
            )),
        }
    }
}
