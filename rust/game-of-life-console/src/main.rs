use clap::Parser;
use game_of_life_console::run;

#[derive(Parser)]
#[command()]
pub struct Args {
    /// File to read the initial set of active cells from. If omitted, cells will be read from standard input
    #[arg(short, long)]
    file: Option<String>,

    /// Size of the square grid
    #[arg(short, long, default_value = "1000")]
    size: u32,
}

fn main() {
    let args = Args::parse();
    match run(args.file, args.size) {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    }
}
