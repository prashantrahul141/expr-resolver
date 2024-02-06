fn main() {
    let input_string = std::env::args().collect::<Vec<String>>()[1..]
        .concat()
        .to_string();

    println!("Input String : {}", &input_string);
}
