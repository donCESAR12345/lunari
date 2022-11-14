use chrono::NaiveDateTime;
use clap::Parser;

const PHASES: [&str; 8] = [
    "luna nueva",
    "luna creciente",
    "primer cuarto",
    "luna menguante",
    "luna llena",
    "gibosa menguante",
    "último cuarto",
    "creciente menguante",
];

const AGES: [f64; 10] = [
    0.0,
    1.0,
    6.38264692644,
    8.38264692644,
    13.76529385288,
    15.76529385288,
    21.14794077932,
    23.14794077932,
    28.53058770576,
    29.53058770576,
];

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    date: String,
}

fn main() {
    // Constantes
    let lunardays: f64 = 29.53058770576;
    let lunarsecs: f64 = lunardays * (24.0 * 60.0 * 60.0);
    let args = Args::parse();
    let new_2000: f64 = NaiveDateTime::parse_from_str("2000-01-06 18:14", "%Y-%m-%d %H:%M")
        .unwrap()
        .timestamp() as f64;
    let date = NaiveDateTime::parse_from_str(&args.date, "%Y-%m-%d %H:%M").unwrap();
    let date_timestamp: f64 = date.timestamp() as f64;
    let total_secs: f64 = date_timestamp - new_2000;
    let mut current_secs: f64 = total_secs % lunarsecs;

    if current_secs < 0.0 {
        current_secs += lunarsecs;
    }
    // Calculate the fraction of the moon cycle
    let currentfrac: f64 = current_secs / lunarsecs;

    // Calculate days in current cycle (moon age)
    let currentdays = currentfrac * lunardays;

    let mut index: usize = 0;

    for i in 0..AGES.len() - 1 {
        if currentdays > AGES[i] && currentdays < AGES[i + 1] {
            index = i;
        }
    }

    let phase = PHASES[index];
    println!("Para la fecha {} la luna estará en {}", date, phase);
}
