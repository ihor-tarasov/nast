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
    Info {
        name: "Factorial",
        trigger: false,
        inputs: &["n"],
        flows: &[],
        outputs: &["result"],
        content: None,
        description: "Function call.",
    },
];
*/

use nast::{utils::IDMap, State, Connect};

fn main() {
    let mut nodes = IDMap::new();
    let one = nodes.insert(nast::number(1.0));
    let n = nodes.insert(nast::argument(0));
    let le = nodes.insert(nast::less_equals(n, one));
    let ret1 = nodes.insert(nast::return_node(one));
    let sub = nodes.insert(nast::subtract(n, one));
    let call = nodes.insert(nast::call(0, vec![sub]));
    let multiply = nodes.insert(nast::multiply(n, call));
    let retm = nodes.insert(nast::return_node(multiply));
    let ifc = nodes.insert(nast::if_node(le, ret1, retm));
    let start_f = nodes.insert(nast::start(ifc));
    nodes.get_mut(call).unwrap().connect(0, start_f);
    let n = nodes.insert(nast::number(6.0));
    let call = nodes.insert(nast::call(start_f, vec![n]));
    let ret = nodes.insert(nast::return_node(call));
    let start = nodes.insert(nast::start(ret));

    let mut state = State::new(start, vec![]);
    match nast::run(&nodes, &mut state) {
        Ok(v) => println!("{v:?}"),
        Err(e) => println!("Error: {e}"),
    }
}
