use lexer::Lexer;
pub mod lexer;

fn main() {
    let input_string = std::env::args().collect::<Vec<String>>()[1..]
        .concat()
        .to_string();

    println!("Input String : {}", &input_string);

    let mut lexer = Lexer::new(input_string);
    lexer.parse();

    for token in lexer.tokens {
        println!("{:?}", token);
    }
}
