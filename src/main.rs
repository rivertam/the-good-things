use std::io::Write;


fn get_good_thing() -> String {
    print!("Say something nice to me: ");
    std::io::stdout().flush().ok().expect("whazza");
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => input,
        Err(_) => {
            println!("That ain't right!");
            get_good_thing()
        }
    }
}

fn main() {
    for _ in 0..5 {
        let good_thing = get_good_thing();
        println!("Got good thing {}", good_thing);
    }
}
