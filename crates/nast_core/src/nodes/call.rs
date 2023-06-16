use crate::{Build, Eval, Res, State};

#[derive(Default, Debug)]
pub struct Call {
    start_id: usize,
    arguments: Vec<usize>,
}

impl Call {
    fn push_argument(&self, state: &mut State) -> Res<()> {
        if state.mark() != 0 {
            state.push_value();
        }
        state.set_state(state.id(), state.mark() + 1);
        state.push_state();
        state.set_state(self.arguments[state.mark() - 1], 0);
        Ok(())
    }

    fn push_function(&self, state: &mut State) -> Res<()> {
        state.push_value();
        let arguments = state.get_values(self.arguments.len());
        state.push_function(arguments);
        state.set_state(self.start_id, 0);
        Ok(())
    }
}

impl Eval for Call {
    fn eval(&self, state: &mut State) -> Res<()> {
        if state.mark() < self.arguments.len() {
            self.push_argument(state)
        } else if state.mark() == self.arguments.len() {
            self.push_function(state)
        } else {
            panic!()
        }
    }
}

impl Build for Call {
    fn push_input(&mut self, name: &String, id: usize, builder: &crate::Builder) -> Res<()> {
        if let Some(function) = builder.functions.get(&builder.desc.name) {
            self.start_id = function.start;
            if self.arguments.len() != function.arguments.len() {
                self.arguments.resize(function.arguments.len(), 0);
            }
            for i in 0..function.arguments.len() {
                if function.arguments[i].eq(name) {
                    self.arguments[i] = id;
                    return Ok(());
                }
            }
        }
        Err(format!(
            "Incompatible input name \"{}\" for node \"{}\" in function \"{}\"",
            name, builder.desc.name, builder.function.name
        ))
    }
}
