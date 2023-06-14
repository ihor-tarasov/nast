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
        Ok(state.set_value(self.0))
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
                    Operator::Subtract => Ok(state.set_value(lhs - rhs)),
                    Operator::Multiply => Ok(state.set_value(lhs * rhs)),
                    Operator::LessEquals => todo!(),
                }
            }
            _ => panic!(),
        }
    }
}

struct Argument(usize);

struct If {
    condition: usize,
    on_then: usize,
    on_else: usize,
}

struct Return(usize);

impl Eval for Return {
    fn eval(&self, state: &mut State) -> Res<()> {
        debug_assert_eq!(state.mark(), 0);
        Ok(state.set_state(self.0, 0))
    }
}

struct Call {
    function: usize,
    arguments: Vec<usize>,
}

enum Node {
    Start(Start),
    Number(Number),
    Binary(Binary),
    Argument(Argument),
    If(If),
    Return(Return),
}

impl Eval for Node {
    fn eval(&self, state: &mut State) -> Res<()> {
        match self {
            Node::Start(s) => s.eval(state),
            Node::Number(n) => n.eval(state),
            Node::Binary(b) => b.eval(state),
            Node::Argument(_) => todo!(),
            Node::If(_) => todo!(),
            Node::Return(r) => r.eval(state),
        }
    }
}

#[derive(Clone, Copy)]
struct NodeState {
    id: usize,
    mark: usize,
}

struct FunctionState(Vec<f64>);

struct State {
    states_stack: Vec<NodeState>,
    values_stack: Vec<f64>,
    functions_stack: Vec<FunctionState>,
    current_state: NodeState,
    current_value: f64,
    current_function: FunctionState,
}

impl State {
    fn new(id: usize, args: Vec<f64>) -> Self {
        Self {
            states_stack: Vec::new(),
            values_stack: Vec::new(),
            functions_stack: Vec::new(),
            current_state: NodeState { id, mark: 0 },
            current_value: 0.0,
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

    fn set_value(&mut self, v: f64) {
        self.current_value = v;
    }

    fn get_value(&self) -> f64 {
        self.current_value
    }

    fn pop_value(&mut self) {
        self.current_value = self.values_stack.pop().unwrap();
    }

    fn push_value(&mut self) {
        self.values_stack.push(self.current_value)
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

fn run(nodes: &IDMap<Node>, state: &mut State) -> Res<f64> {
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
    let two = nodes.insert(Node::Number(Number(2.0)));
    let multiply = nodes.insert(Node::Binary(Binary {
        oper: Operator::Multiply,
        left: two,
        right: two,
    }));
    let subtract = nodes.insert(Node::Binary(Binary {
        oper: Operator::Subtract,
        left: multiply,
        right: two,
    }));
    let ret = nodes.insert(Node::Return(Return(subtract)));
    let start = nodes.insert(Node::Start(Start(ret)));

    let mut state = State::new(start, vec![]);
    match run(&nodes, &mut state) {
        Ok(v) => println!("{v}"),
        Err(e) => println!("Error: {e}"),
    }
}
