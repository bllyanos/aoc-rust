use std::env;
mod day_1;
mod day_2;
mod day_3;
mod day_4;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    if let Some(command) = arguments.get(1) {
        let path = arguments.get(2).unwrap();
        match command.as_str() {
            "day_1" => {
                day_1::run(path).unwrap();
            }
            "day_2" => {
                day_2::run(path).unwrap();
            }
            "day_3" => {
                day_3::run(path).unwrap();
            }
            "day_4" => {
                day_4::run(path).unwrap();
            }
            _ => {
                println!("unknown command: {}", command)
            }
        }
    } else {
        println!("no command specified");
    }
}
