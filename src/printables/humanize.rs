use grp_core::chrono::Utc;
use grp_core::chrono::DateTime;


pub trait Humanize {
    fn to_human(&self) -> String;
}

impl Humanize for DateTime<Utc> {
    fn to_human(&self) -> String {
        let now = Utc::now();
        let duration = now.signed_duration_since(*self);
        let total_seconds = duration.num_seconds();
        let direction = if total_seconds <= 0 { "ahead" } else { "ago" };
        
        match total_seconds.abs() {
            ts if ts < 60 => format!("{} sec {}", ts, direction),
            ts if ts < 60*60 => {
                let minutes = ts / 60;
                let seconds = ts % 60;
                format!("{} min {} sec {}", minutes, seconds, direction)
            },
            ts if ts < 60*60*24 => {
                let hours = ts / 3600;
                let minutes = (ts % 3600) / 60;
                format!("{} hours {} min {}", hours, minutes, direction)
            },
            ts if ts < 60*60*24*30 => {
                let days = ts / (60*60*24);
                format!("{} days {}", days, direction)
            },
            _ => self.format("%Y-%m-%d").to_string(),
        }
    }
}
