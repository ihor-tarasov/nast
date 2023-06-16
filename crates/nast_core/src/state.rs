use serde::{Deserialize, Serialize};

use crate::Value;

pub type Res<T> = Result<T, String>;

#[derive(Serialize, Deserialize, Clone, Copy)]
struct NodeState {
    id: usize,
    mark: usize,
}

#[derive(Serialize, Deserialize)]
struct FunctionState(Vec<Value>);

#[derive(Serialize, Deserialize)]
pub struct State {
    states_stack: Vec<NodeState>,
    values_stack: Vec<Value>,
    functions_stack: Vec<FunctionState>,
    current_state: NodeState,
    current_value: Value,
    current_function: FunctionState,
}

impl State {
    pub fn new(id: usize, args: Vec<Value>) -> Self {
        Self {
            states_stack: Vec::new(),
            values_stack: Vec::new(),
            functions_stack: Vec::new(),
            current_state: NodeState { id, mark: 0 },
            current_value: Value::Void,
            current_function: FunctionState(args),
        }
    }

    pub fn set_state(&mut self, id: usize, mark: usize) {
        self.current_state.id = id;
        self.current_state.mark = mark;
    }

    pub fn id(&self) -> usize {
        self.current_state.id
    }

    pub fn mark(&self) -> usize {
        self.current_state.mark
    }

    pub fn pop_state(&mut self) -> Res<()> {
        match self.states_stack.pop() {
            Some(s) => Ok(self.current_state = s),
            None => Err("".to_string()),
        }
    }

    pub fn push_state(&mut self) {
        self.states_stack.push(self.current_state)
    }

    pub fn set_value(&mut self, v: Value) {
        self.current_value = v;
    }

    pub fn get_value(&self) -> Value {
        self.current_value
    }

    pub fn pop_value(&mut self) {
        self.current_value = self.values_stack.pop().unwrap();
    }

    pub fn push_value(&mut self) {
        self.values_stack.push(self.current_value)
    }

    pub fn load_argument(&mut self, index: usize) {
        self.current_value = self.current_function.0[index];
    }

    pub fn push_function(&mut self, args: Vec<Value>) {
        self.functions_stack.push(std::mem::replace(
            &mut self.current_function,
            FunctionState(args),
        ));
    }

    pub fn pop_function(&mut self) -> Res<()> {
        match self.functions_stack.pop() {
            Some(d) => Ok(self.current_function = d),
            None => Err("".to_string()),
        }
    }

    pub fn get_values(&mut self, count: usize) -> Vec<Value> {
        let res = self
            .values_stack
            .iter()
            .rev()
            .take(count)
            .cloned()
            .collect();
        self.values_stack
            .resize(self.values_stack.len() - count, Value::Void);
        res
    }
}
