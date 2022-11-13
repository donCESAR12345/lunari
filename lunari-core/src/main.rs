use chrono::NaiveDateTime;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    date: String,
}

fn main() {
    // Constantes
    let lunardays = 29.53058770576;
    let lunarsecs = lunardays * (24.0 * 60.0 * 60.0);
    let args = Args::parse();
    let new_2000 = NaiveDateTime::parse_from_str("2000-01-06 18:14", "%Y-%m-%d %H:%M")
        .unwrap()
        .timestamp();
    let date = NaiveDateTime::parse_from_str(&args.date, "%Y-%m-%d %H:%M").unwrap();
    let date_timestamp = date.timestamp();
    let total_secs = date_timestamp - new_2000;
    let current_secs = lunarsecs % total_secs;

    println!("Para la fecha {} la luna estará en...", date);
}
