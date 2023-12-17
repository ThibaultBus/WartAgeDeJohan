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

//Takes the arguments of the command line, and extracts a date out of them if one is specified
fn parse_date(dt_str: &str) -> Result<NaiveDateTime, &'static str> {
    let vec_date : Vec<u32> = dt_str.split('/')
        .into_iter()
        .map(|s| -> u32 { s.parse().unwrap() } )
        .collect();

    //tries to return a date
    match NaiveDate::from_ymd_opt(vec_date[2] as i32, vec_date[1], vec_date[0]) {
        Some(dt) => Ok(dt.and_hms_opt(0, 0, 0).unwrap()),
        _ => Err("Incorrect syntax, use dd/mm/yyyy to specify a date"),
    }
}
