use crate::{Eval, State, Res};

pub struct Start(pub usize);

impl Eval for Start {
    fn eval(&self, state: &mut State) -> Res<()> {
        debug_assert_eq!(state.mark(), 0);
        Ok(state.set_state(self.0, 0))
    }
}
