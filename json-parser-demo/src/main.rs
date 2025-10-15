use json_parser_demo::{parser::Parser, token};

fn main() {
    println!("Hello, world!");
    let json = r#"{"Hello": "world"}"#;
    let mut parser = Parser::new();
    parser.parse(json);

    let tokens = token::tokenize(json);
    for token in tokens {
        println!("{}", token);
    }
}
