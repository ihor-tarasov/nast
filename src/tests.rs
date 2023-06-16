use crate::{utils::IDMap, State, Connect, Value};

#[test]
fn factorial_test() {
    let mut nodes = IDMap::new();
    let one = nodes.insert(crate::number(1.0));
    let n = nodes.insert(crate::argument(0));
    let le = nodes.insert(crate::less_equals(n, one));
    let ret1 = nodes.insert(crate::return_node(one));
    let sub = nodes.insert(crate::subtract(n, one));
    let call = nodes.insert(crate::call(0, vec![sub]));
    let multiply = nodes.insert(crate::multiply(n, call));
    let retm = nodes.insert(crate::return_node(multiply));
    let ifc = nodes.insert(crate::if_node(le, ret1, retm));
    let start_f = nodes.insert(crate::start(ifc));
    nodes.get_mut(call).unwrap().connect(1, start_f);
    let n = nodes.insert(crate::number(6.0));
    let call = nodes.insert(crate::call(start_f, vec![n]));
    let ret = nodes.insert(crate::return_node(call));
    let start = nodes.insert(crate::start(ret));

    let mut state = State::new(start, vec![]);
    match crate::run(&nodes, &mut state) {
        Ok(v) => {
            assert_eq!(v, Value::Number(720.0));
            println!("{v:?}")
        },
        Err(e) => assert!(false, "Error: {e}"),
    }
}
