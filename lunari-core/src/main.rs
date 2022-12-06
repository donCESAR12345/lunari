pub mod lunar_phases;

pub(crate) use clap::Parser;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("phase"))]
    command: String,
    #[arg(short, long, default_value_t = String::from("now"))]
    date: String,
}

fn main() {
    let arguments = Args::parse();
    match &arguments.command as &str {
        "phase" => {
            println!("{}", lunar_phases::LunarPhases::new(&arguments.date));
        },
        _ => panic!("Unknown command.")
    }
}
