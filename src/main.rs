fn main() {
    loop {
        let mut s = String::new();

        println!("Guess somethn");
        std::io::stdin().read_line(&mut s).unwrap();
        println!("You got it wrong...");
    }
}
