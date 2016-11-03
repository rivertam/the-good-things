use std::io::Write;
use std::fs::OpenOptions;

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

fn data_dir_path() -> String {
    std::env::home_dir()
        .map(|home_dir| home_dir.display().to_string() + "/.config/the-good-things")
        .unwrap_or("/usr/share/the-good-things".to_string())
}

fn write_good_thing(string: String) {
    let dir_path = data_dir_path();
    let res = std::fs::create_dir_all(&dir_path)
        .and_then(|_| {
            println!("{}/good-things.txt", dir_path);
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(dir_path + "/good-things.txt")
        })
    .and_then(|mut file| {
        file.write_all(string.as_bytes())
    });

    match res {
        Ok(_) => println!("Success?"),
        Err(error) => println!("Couldn't write to file {}", error)
    }
}

fn main() {
    for _ in 0..5 {
        let good_thing = get_good_thing();
        write_good_thing(good_thing);
    }
}
