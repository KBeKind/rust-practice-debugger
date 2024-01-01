fn main() {
    let mut input = String::new();
    while input.trim() != "exit" {
        input.clear();
        println!("Please input a word (type 'exit' to exit): ");
        std::io::stdin().read_line(&mut input).unwrap();
        println!("You entered: {}", input);
        

    }
    println!("Bye Bye");
}
