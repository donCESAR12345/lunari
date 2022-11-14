use chrono::NaiveDateTime;
use clap::Parser;

// Fases de la luna
const PHASES: [&str; 9] = [
    "luna nueva",
    "luna creciente",
    "primer cuarto",
    "luna menguante",
    "luna llena",
    "gibosa menguante",
    "tercer cuarto",
    "creciente menguante",
    "luna nueva",
];

// Intervalos de edad de la luna
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

// Constantes del período lunar
const LUNAR_DAYS: f64 = 29.53058770576;
const LUNAR_SECS: f64 = LUNAR_DAYS * (24.0 * 60.0 * 60.0);

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    date: String,
}

fn main() {
    let new_2000: f64 = NaiveDateTime::parse_from_str("2000-01-06 18:14", "%Y-%m-%d %H:%M")
        .unwrap()
        .timestamp() as f64;
    let args = Args::parse();
    let date = NaiveDateTime::parse_from_str(&args.date, "%Y-%m-%d %H:%M").unwrap();
    let date_timestamp: f64 = date.timestamp() as f64;
    let total_secs: f64 = date_timestamp - new_2000;
    let mut current_secs: f64 = total_secs % LUNAR_SECS;

    if current_secs < 0.0 {
        current_secs += LUNAR_SECS;
    }
    // Calculate the fraction of the moon cycle
    let currentfrac: f64 = current_secs / LUNAR_SECS;

    // Calculate days in current cycle (moon age)
    let currentdays = currentfrac * LUNAR_DAYS;

    let mut phase: &str = "";

    // Encuentra el intervalo en el que se encuentra
    for i in 0..AGES.len() - 1 {
        if currentdays > AGES[i] && currentdays < AGES[i + 1] {
            phase = PHASES[i];
        }
    }

    println!("Para la fecha {} la luna estará en {}.", date, phase);
}
