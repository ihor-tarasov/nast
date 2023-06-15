
/*
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
*/

use nast::{utils::IDMap, Node, nodes::*, State};

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
    match nast::run(&nodes, &mut state) {
        Ok(v) => println!("{v:?}"),
        Err(e) => println!("Error: {e}"),
    }
}
