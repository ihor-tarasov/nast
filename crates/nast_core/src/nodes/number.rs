use crate::{Build, Eval, Res, State, Value};

#[derive(Default, Debug)]
pub struct Number(f64);

impl Eval for Number {
    fn eval(&self, state: &mut State) -> Res<()> {
        debug_assert_eq!(state.mark(), 0);
        state.pop_state()?;
        Ok(state.set_value(Value::Number(self.0)))
    }
}

impl Build for Number {
    fn push_content(&mut self, content: &crate::Content, builder: &crate::Builder) -> Res<()> {
        match content {
            crate::Content::Number(v) => Ok(self.0 = *v),
            _ => Err(format!(
                "Incompatible content \"{:?}\" for node \"{}\" in function \"{}\"",
                content, builder.desc.name, builder.function.name
            )),
        }
    }
}
