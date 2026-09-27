use std::{
    fs::File, 
    io::{
        self, 
        Write,
    }, 
    process::{
        Command,
        Stdio
    }, 
    env
};


const MAIN_ROUTE: &str = "/home/Zukunter/rush/src/main.rs";

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();

        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }
        
        if input == "exit\n" {
            break;
        }
        if input.is_empty() {
            continue;
        }
        let mut main_file = File::create(MAIN_ROUTE).unwrap();

        let mut intial = String::from("fn main() {");
        intial.push_str(&input);
        intial.push('}');
        write!(&mut main_file, "{intial}").unwrap();

        Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .arg("--manifest-path")
        .arg("/home/Zukunter/rush/Cargo.toml")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();
    }
}
