fn main() {
    let mut args = std::env::args().skip(1);

    match args.next() {
        Some(command) => match command.as_str() {
            "info" => match args.next() {
                Some(path) => match std::fs::read_to_string(path) {
                    Ok(s) => match nast_json::get_info(s.as_str()) {
                        Ok(info) => println!("{info}"),
                        Err(error) => println!("Error: {error}"),
                    },
                    Err(e) => println!("Error: {}", e.to_string()),
                },
                None => println!("Error: enter path"),
            },
            "run" => match args.next() {
                Some(path) => match std::fs::read_to_string(path) {
                    Ok(s) => match nast_json::run(s.as_str()) {
                        Ok(v) => println!("{v:?}"),
                        Err(error) => println!("Error: {error}"),
                    },
                    Err(e) => println!("Error: {}", e.to_string()),
                },
                None => println!("Error: enter path"),
            },
            _ => println!("Error: unknown command."),
        },
        None => println!("Error: enter command"),
    };
}
