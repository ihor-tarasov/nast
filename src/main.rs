use std::{collections::HashMap, num::NonZeroUsize};

struct Info {
    name: &'static str,
    trigger: bool,
    inputs: &'static [&'static str],
    flows: &'static [&'static str],
    outputs: &'static [&'static str],
    content: Option<&'static str>,
    description: &'static str,
}

const STD_NODES: &[Info] = &[
    Info {
        name: "Start",
        trigger: false,
        inputs: &[],
        flows: &["flow"],
        outputs: &[],
        content: None,
        description: "Entry point of function.",
    },
    Info {
        name: "Return",
        trigger: true,
        inputs: &["result"],
        flows: &[],
        outputs: &[],
        content: None,
        description: "Return from function.",
    },
    Info {
        name: "Argument",
        trigger: false,
        inputs: &[],
        flows: &[],
        outputs: &["value"],
        content: Some("identifier"),
        description: "Provide an argument of current function.",
    },
    Info {
        name: "Number",
        trigger: false,
        inputs: &[],
        flows: &[],
        outputs: &["output"],
        content: Some("number"),
        description: "Provide constant number value.",
    },
    Info {
        name: "Subtract",
        trigger: false,
        inputs: &["left", "right"],
        flows: &[],
        outputs: &["result"],
        content: None,
        description: "Perform subtract operation.",
    },
    Info {
        name: "Less Equals",
        trigger: false,
        inputs: &["left", "right"],
        flows: &[],
        outputs: &["result"],
        content: None,
        description: "Perform less equals operation.",
    },
    Info {
        name: "Multiply",
        trigger: false,
        inputs: &["left", "right"],
        flows: &[],
        outputs: &["result"],
        content: None,
        description: "Perform multiply operation.",
    },
    Info {
        name: "If",
        trigger: true,
        inputs: &["condition"],
        flows: &["then", "else"],
        outputs: &[],
        content: None,
        description: "If statement.",
    },
];

enum Content {
    Number(f64),
    Identifier(String),
}

struct Instance {
    name: String,
    trigger: bool,
    inputs: Vec<Option<usize>>,
    flows: Vec<Option<usize>>,
    outputs: Vec<Option<usize>>,
    content: Option<Content>,
}

struct Function {
    instances: IDMap<Instance>,
    arguments: Vec<String>,
}

type Res<T> = Result<T, String>;

trait Eval {
    fn eval(&self, state: &mut State) -> Res<()>;
}

struct Start(usize);

impl Eval for Start {
    fn eval(&self, state: &mut State) -> Res<()> {
        debug_assert_eq!(state.mark(), 0);
        Ok(state.set_state(self.0, 0))
    }
}

struct Number(f64);

impl Eval for Number {
    fn eval(&self, state: &mut State) -> Res<()> {
        debug_assert_eq!(state.mark(), 0);
        state.pop_state()?;
        Ok(state.set_value(Value::Number(self.0)))
    }
}

enum Operator {
    Subtract,
    Multiply,
    LessEquals,
}

struct Binary {
    oper: Operator,
    left: usize,
    right: usize,
}

impl Eval for Binary {
    fn eval(&self, state: &mut State) -> Res<()> {
        match state.mark() {
            0 => {
                state.set_state(state.id(), 1);
                state.push_state();
                Ok(state.set_state(self.left, 0))
            }
            1 => {
                state.set_state(state.id(), 2);
                state.push_state();
                state.push_value();
                Ok(state.set_state(self.right, 0))
            }
            2 => {
                state.pop_state()?;
                let rhs = state.get_value();
                state.pop_value();
                let lhs = state.get_value();
                match self.oper {
                    Operator::Subtract => Ok(state.set_value(match (lhs, rhs) {
                        (Value::Number(lhs), Value::Number(rhs)) => Value::Number(lhs - rhs),
                        _ => {
                            return Err(format!(
                                "Unsupported input values {:?} and {:?} for Subtract node.",
                                lhs, rhs
                            ))
                        }
                    })),
                    Operator::Multiply => Ok(state.set_value(match (lhs, rhs) {
                        (Value::Number(lhs), Value::Number(rhs)) => Value::Number(lhs * rhs),
                        _ => {
                            return Err(format!(
                                "Unsupported input values {:?} and {:?} for Multiply node.",
                                lhs, rhs
                            ))
                        }
                    })),
                    Operator::LessEquals => Ok(state.set_value(match (lhs, rhs) {
                        (Value::Number(lhs), Value::Number(rhs)) => Value::Boolean(lhs <= rhs),
                        _ => {
                            return Err(format!(
                                "Unsupported input values {:?} and {:?} for LessEquals node.",
                                lhs, rhs
                            ))
                        }
                    })),
                }
            }
            _ => panic!(),
        }
    }
}

struct Argument(usize);

impl Eval for Argument {
    fn eval(&self, state: &mut State) -> Res<()> {
        debug_assert_eq!(state.mark(), 0);
        state.pop_state()?;
        Ok(state.load_argument(self.0))
    }
}

struct If {
    condition: usize,
    on_then: usize,
    on_else: usize,
}

impl Eval for If {
    fn eval(&self, state: &mut State) -> Res<()> {
        match state.mark() {
            0 => {
                state.set_state(state.id(), 1);
                state.push_state();
                Ok(state.set_state(self.condition, 0))
            }
            1 => {
                let condition = state.get_value();
                let condition = match condition {
                    Value::Boolean(v) => v,
                    _ => {
                        return Err(format!(
                            "For If node Boolean type expected for 'condition' input, found {:?}",
                            condition
                        ))
                    }
                };
                if condition {
                    Ok(state.set_state(self.on_then, 0))
                } else {
                    Ok(state.set_state(self.on_else, 0))
                }
            }
            _ => panic!(),
        }
    }
}

struct Return(usize);

impl Eval for Return {
    fn eval(&self, state: &mut State) -> Res<()> {
        debug_assert_eq!(state.mark(), 0);
        state.pop_function()?;
        Ok(state.set_state(self.0, 0))
    }
}

struct Call {
    start_id: usize,
    arguments: Vec<usize>,
}

impl Eval for Call {
    fn eval(&self, state: &mut State) -> Res<()> {
        if state.mark() < self.arguments.len() {
            if state.mark() != 0 {
                state.push_value();
            }
            state.set_state(state.id(), state.mark() + 1);
            state.push_state();
            state.set_state(self.arguments[state.mark()], 0);
            Ok(())
        } else if state.mark() == self.arguments.len() {
            state.push_value();
            let arguments = state.get_values(self.arguments.len());
            state.push_function(arguments);
            state.set_state(self.start_id, 0);
            Ok(())
        } else {
            panic!()
        }
    }
}

enum Node {
    Start(Start),
    Number(Number),
    Binary(Binary),
    Argument(Argument),
    If(If),
    Return(Return),
    Call(Call),
}

impl Eval for Node {
    fn eval(&self, state: &mut State) -> Res<()> {
        match self {
            Node::Start(s) => s.eval(state),
            Node::Number(n) => n.eval(state),
            Node::Binary(b) => b.eval(state),
            Node::Argument(a) => a.eval(state),
            Node::If(i) => i.eval(state),
            Node::Return(r) => r.eval(state),
            Node::Call(c) => c.eval(state),
        }
    }
}

#[derive(Clone, Copy)]
struct NodeState {
    id: usize,
    mark: usize,
}

#[derive(Debug, Clone, Copy)]
enum Value {
    Void,
    Boolean(bool),
    Number(f64),
}

struct FunctionState(Vec<Value>);

struct State {
    states_stack: Vec<NodeState>,
    values_stack: Vec<Value>,
    functions_stack: Vec<FunctionState>,
    current_state: NodeState,
    current_value: Value,
    current_function: FunctionState,
}

impl State {
    fn new(id: usize, args: Vec<Value>) -> Self {
        Self {
            states_stack: Vec::new(),
            values_stack: Vec::new(),
            functions_stack: Vec::new(),
            current_state: NodeState { id, mark: 0 },
            current_value: Value::Void,
            current_function: FunctionState(args),
        }
    }

    fn set_state(&mut self, id: usize, mark: usize) {
        self.current_state.id = id;
        self.current_state.mark = mark;
    }

    fn id(&self) -> usize {
        self.current_state.id
    }

    fn mark(&self) -> usize {
        self.current_state.mark
    }

    fn pop_state(&mut self) -> Res<()> {
        match self.states_stack.pop() {
            Some(s) => Ok(self.current_state = s),
            None => Err("".to_string()),
        }
    }

    fn push_state(&mut self) {
        self.states_stack.push(self.current_state)
    }

    fn set_value(&mut self, v: Value) {
        self.current_value = v;
    }

    fn get_value(&self) -> Value {
        self.current_value
    }

    fn pop_value(&mut self) {
        self.current_value = self.values_stack.pop().unwrap();
    }

    fn push_value(&mut self) {
        self.values_stack.push(self.current_value)
    }

    fn load_argument(&mut self, index: usize) {
        self.current_value = self.current_function.0[index];
    }

    fn push_function(&mut self, args: Vec<Value>) {
        self.functions_stack.push(std::mem::replace(
            &mut self.current_function,
            FunctionState(args),
        ));
    }

    fn pop_function(&mut self) -> Res<()> {
        match self.functions_stack.pop() {
            Some(d) => Ok(self.current_function = d),
            None => Err("".to_string()),
        }
    }

    fn get_values(&mut self, count: usize) -> Vec<Value> {
        let res = self.values_stack.iter().rev().take(count).cloned().collect();
        self.values_stack.resize(self.values_stack.len() - count, Value::Void);
        res
    }
}

struct IDProvider(usize);

impl IDProvider {
    fn new() -> Self {
        Self(0)
    }

    fn next(&mut self) -> usize {
        self.0 = self.0.wrapping_add(1);
        self.0
    }
}

struct IDMap<T> {
    map: HashMap<usize, T>,
    provider: IDProvider,
}

impl<T> IDMap<T> {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            provider: IDProvider::new(),
        }
    }

    fn insert(&mut self, t: T) -> usize {
        let mut id = self.provider.next();
        if self.map.contains_key(&id) || id == 0 {
            id = self.provider.next();
        }
        self.map.insert(id, t);
        id
    }

    fn remove(&mut self, id: usize) -> Option<T> {
        self.map.remove(&id)
    }

    fn get(&self, id: usize) -> Option<&T> {
        self.map.get(&id)
    }

    fn get_mut(&mut self, id: usize) -> Option<&mut T> {
        self.map.get_mut(&id)
    }
}

fn step(nodes: &IDMap<Node>, state: &mut State) -> Res<bool> {
    let id = state.id();
    match nodes.get(id).unwrap().eval(state) {
        Ok(_) => Ok(true),
        Err(e) => {
            if e.is_empty() {
                Ok(false)
            } else {
                Err(e)
            }
        }
    }
}

fn run(nodes: &IDMap<Node>, state: &mut State) -> Res<Value> {
    while step(nodes, state)? {}
    Ok(state.get_value())
}

struct NodeStorage(IDMap<Node>);

impl NodeStorage {
    fn new() -> Self {
        Self(IDMap::new())
    }
}

fn main() {
    let mut nodes = IDMap::new();
    let one = nodes.insert(Node::Number(Number(1.0)));
    let n = nodes.insert(Node::Argument(Argument(0)));
    let le = nodes.insert(Node::Binary(Binary { oper: Operator::LessEquals, left: n, right: one }));
    let ret1 = nodes.insert(Node::Return(Return(one)));
    let sub = nodes.insert(Node::Binary(Binary { oper: Operator::Subtract, left: n, right: one }));
    let call = nodes.insert(Node::Call(Call { start_id: 0, arguments: vec![sub] }));
    let multiply = nodes.insert(Node::Binary(Binary {
        oper: Operator::Multiply,
        left: n,
        right: call,
    }));
    let retm = nodes.insert(Node::Return(Return(multiply)));
    let ifc = nodes.insert(Node::If(If { condition: le, on_then: ret1, on_else: retm }));
    let start_f = nodes.insert(Node::Start(Start(ifc)));
    if let Node::Call(c) = nodes.get_mut(call).unwrap() {
        c.start_id = start_f;
    } else {
        panic!();
    }
    let n = nodes.insert(Node::Number(Number(6.0)));
    let call = nodes.insert(Node::Call(Call { start_id: start_f, arguments: vec![n] }));
    let ret = nodes.insert(Node::Return(Return(call)));
    let start = nodes.insert(Node::Start(Start(ret)));

    let mut state = State::new(start, vec![]);
    match run(&nodes, &mut state) {
        Ok(v) => println!("{v:?}"),
        Err(e) => println!("Error: {e}"),
    }
}
