use game_of_life_console::ConsoleRunner;

fn main() {
    match ConsoleRunner::run() {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    }
}
