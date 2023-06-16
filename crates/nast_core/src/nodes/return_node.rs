use crate::{Build, Eval, Res, State};

#[derive(Default, Debug)]
pub struct Return(usize);

impl Return {
    fn push_return_state(&self, state: &mut State) -> Res<()> {
        state.set_state(state.id(), 1);
        state.push_state();
        state.set_state(self.0, 0);
        Ok(())
    }

    fn pop(&self, state: &mut State) -> Res<()> {
        state.pop_function()?;
        state.pop_state()?;
        Ok(())
    }
}

impl Eval for Return {
    fn eval(&self, state: &mut State) -> Res<()> {
        match state.mark() {
            0 => self.push_return_state(state),
            1 => self.pop(state),
            _ => panic!(),
        }
    }
}

impl Build for Return {
    fn push_input(&mut self, name: &String, id: usize, builder: &crate::Builder) -> Res<()> {
        match name.as_str() {
            "result" => Ok(self.0 = id),
            _ => Err(format!(
                "Incompatible input name \"{}\" for node \"{}\" in function \"{}\"",
                name, builder.desc.name, builder.function.name
            )),
        }
    }
}
