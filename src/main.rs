use std::io::Write;
use std::fs::OpenOptions;
use std::io::BufRead;

fn get_good_thing() -> String {
    print!("Say something nice to me: ");
    std::io::stdout().flush().expect("Couldn't flush stdout");
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => input,
        Err(_) => {
            println!("That ain't right!");
            get_good_thing()
        }
    }
}

static DATA_FILE_NAME: &'static str = "good-things.txt";

fn data_dir_path() -> String {
    std::env::home_dir()
        .map(|home_dir| home_dir.display().to_string() + "/.config/the-good-things")
        .unwrap_or("/usr/share/the-good-things".to_string())
}

fn data_file_path() -> String {
    data_dir_path() + "/" + DATA_FILE_NAME
}

fn fetch_good_things() -> Vec<String> {
    let res = std::fs::File::open(data_file_path())
        .and_then(|f| {
            let file = std::io::BufReader::new(&f);
            let mut vec = std::vec::Vec::new();
            for line in file.lines() {
                match line {
                    Ok(l) => vec.push(l),
                    Err(_) => continue,
                }
            }

            return Ok(vec)
        });

    match res {
        Ok(v) => v,
        Err(e) => {
            println!("Couldn't open good things file");
            println!("Located at {}", data_file_path());
            println!("Error: {}", e);
            std::vec::Vec::new()
        },
    }
}

fn write_good_thing(string: String) {
    let dir_path = data_dir_path();
    let res = std::fs::create_dir_all(&dir_path)
        .and_then(|_| {
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(data_file_path())
        }).and_then(|mut file| {
            file.write_all(string.as_bytes())
        });

    match res {
        Ok(_) => return,
        Err(error) => println!("Couldn't write to file {}", error)
    }
}

fn main() {
    for good_thing in fetch_good_things() {
        println!("{}", good_thing);
    }


    for _ in 0..5 {
        let good_thing = get_good_thing();
        write_good_thing(good_thing);
    }
}
