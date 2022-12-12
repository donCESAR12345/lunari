use std::fmt::Display;
use chrono::NaiveDateTime;

pub(crate) enum LunarPhases {
    NewMoon,
    WaxingCrescent,
    FirstQuarterMoon,
    WaxingGibbous,
    FullMoon,
    WaningGibbous,
    LastQuarterMoon,
    WaningCrescent,
}

impl Display for LunarPhases {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::NewMoon => write!(f, "luna nueva"),
            Self::WaxingCrescent => write!(f, "luna creciente"),
            Self::FirstQuarterMoon => write!(f, "cuarto creciente"),
            Self::WaxingGibbous => write!(f, "luna menguante"),
            Self::FullMoon => write!(f, "luna llena"),
            Self::WaningGibbous => write!(f, "gibosa menguante"),
            Self::LastQuarterMoon => write!(f, "tercer cuarto"),
            Self::WaningCrescent => write!(f, "creciente menguante")
        }
    }
}

impl LunarPhases {
    /// Creates a new [`LunarPhases`].
    ///
    /// # Panics
    ///
    /// Panics if wrong date format is given.
    pub fn new(date_str: &str) -> Self {
        // Constantes del período lunar
        const LUNAR_DAYS: f64 = 29.53058770576;
        const LUNAR_SECS: f64 = LUNAR_DAYS * (24.0 * 60.0 * 60.0);
        let date_fmt = String::from("%Y-%m-%d %H:%M");
        let new_2000 = NaiveDateTime::parse_from_str("2000-01-06 18:14", &date_fmt) 
                .unwrap()
                .timestamp();

        let date_timestamp: i64;

        if date_str == "now" {
            let date = chrono::Utc::now();
            date_timestamp = date.timestamp();
        }
        else {
            let Ok(date) = NaiveDateTime::parse_from_str(date_str, &date_fmt) else { panic!("Error parsing date.") };
            date_timestamp = date.timestamp();
        }

        let total_secs = (date_timestamp - new_2000) as f64;
        let mut current_secs = total_secs % LUNAR_SECS;

        if current_secs < 0.0 {
            current_secs += LUNAR_SECS;
        }
        
        // Calculate days in current cycle (moon age)
        let currentdays = current_secs / LUNAR_SECS * LUNAR_DAYS;

        match currentdays {
            x if x < 1.             => Self::NewMoon,
            x if x < 6.38264692644  => Self::WaxingCrescent,
            x if x < 8.38264692644  => Self::FirstQuarterMoon,
            x if x < 13.76529385288 => Self::WaxingGibbous,
            x if x < 15.76529385288 => Self::FullMoon,
            x if x < 21.14794077932 => Self::WaningGibbous,
            x if x < 28.53058770576 => Self::LastQuarterMoon,
            _                       => Self::WaningCrescent
        }
    }
}
