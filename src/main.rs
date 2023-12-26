use std::str::FromStr;
use chrono::prelude::*;
use chrono_tz::Tz;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// Manually override the current date [default: today]
    #[clap(short, long = "date", value_parser = parse_date)]
    date_option: Option<NaiveDateTime>,

    /// Manually override the timezone for "today" (IANA names) [default: local]
    #[clap(short, long, alias = "tz")]
    timezone: Option<Tz>,

    /// Print raw data instead of writing a sentence
    #[clap(short, long = "raw")]
    raw_data: bool,
}

fn main() {
    let args = Cli::parse();

    let age = match (args.date_option, args.timezone) {
        (None, None) => howoldisyohan::age(Local::now().date_naive()),
        (None, Some(tz)) => howoldisyohan::age(Local::now().with_timezone(&tz).date_naive()),
        (Some(date), None) => howoldisyohan::age(Local.from_local_datetime(&date).unwrap().date_naive()),
        (Some(date), Some(tz)) => howoldisyohan::age(tz.from_local_datetime(&date).unwrap().date_naive()),
    };

    if args.raw_data {
        println!("{}",age);
    } else {
        println!("Johan is {} years old", age)
    }
}

#[derive(Debug, thiserror::Error)]
enum ParseDateError {
    #[error("cannot parse one of the date components: {0}")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("missing component, the date should be written as DD/MM/YYYY")]
    MissingComponent,

    #[error("date \"{0}\" doesn't exist")]
    OutOfBounds(String),
}

//Takes the arguments of the command line, and extracts a date out of them if one is specified
fn parse_date(dt_str: &str) -> Result<NaiveDateTime, ParseDateError> {
    let mut components = dt_str.split('/').map(u32::from_str).fuse();

    let day = components.next().ok_or(ParseDateError::MissingComponent)??;
    let month = components.next().ok_or(ParseDateError::MissingComponent)??;
    let year = components.next().ok_or(ParseDateError::MissingComponent)??;

    match NaiveDate::from_ymd_opt(year as i32, month, day) {
        Some(dt) => Ok(dt.and_hms_opt(0, 0, 0).unwrap()),
        _ => Err(ParseDateError::OutOfBounds(dt_str.into())),
    }
}
