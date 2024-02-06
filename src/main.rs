use lexer::Lexer;
pub mod lexer;

fn main() {
    // take input string as command line argument.
    let input_string = std::env::args().collect::<Vec<String>>()[1..]
        .concat()
        .to_string();

    println!("Input String : {}", &input_string);

    // start lexer.
    let mut lexer = Lexer::new(input_string);
    lexer.scan();

    for token in lexer.tokens {
        println!("{:?}", token);
    }
}
