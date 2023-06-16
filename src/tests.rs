use std::collections::HashMap;

use crate::{State, Value, build};

#[test]
fn factorial_test() {
    let functions: Vec<crate::Function> = serde_json::from_str(
        std::fs::read_to_string("tests/factorial.json")
            .unwrap()
            .as_str(),
    )
    .unwrap();

    let functions = functions.into_iter().map(|f| (f.name.clone(), f)).collect::<HashMap<_, _>>();
    let mut nodes = HashMap::new();

    build(&functions, &mut nodes).unwrap();

    println!("{:#?}", nodes);
    
    let mut state = State::new(functions.get("Main").unwrap().start, vec![]);
    match crate::run(&nodes, &mut state) {
        Ok(v) => {
            assert_eq!(v, Value::Number(720.0));
            println!("{v:?}")
        },
        Err(e) => assert!(false, "Error: {e}"),
    }
}
