use std::collections::HashMap;

use nast_core::Function;
pub use nast_core::Res;
use nast_core::State;
pub use nast_core::Value;

pub fn get_info(functions: &str) -> Res<String> {
    match serde_json::from_str::<Vec<Function>>(functions) {
        Ok(functions) => {
            let functions = functions
                .into_iter()
                .map(|f| (f.name.clone(), f))
                .collect::<HashMap<_, _>>();
            match serde_json::to_string(&nast_core::get_info(&functions)) {
                Ok(s) => Ok(s),
                Err(e) => Err(e.to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

pub fn run(functions: &str) -> Res<Value> {
    match serde_json::from_str::<Vec<Function>>(functions) {
        Ok(functions) => {
            let functions = functions
                .into_iter()
                .map(|f| (f.name.clone(), f))
                .collect::<HashMap<_, _>>();
            let mut nodes = HashMap::new();

            nast_core::build(&functions, &mut nodes)?;
            nast_core::check(&functions, &nodes)?;

            //println!("{:#?}", nodes);

            let mut state = State::new(functions.get("Main").unwrap().start, vec![]);
            nast_core::run(&nodes, &mut state)
        }
        Err(err) => Err(err.to_string()),
    }
}
