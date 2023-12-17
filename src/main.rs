use chrono::prelude::*;
use chrono::offset::LocalResult;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// Manually override the current date [default: today]
    #[clap(short, long = "date")]
    date_option: Option<String>,

    /// Print raw data instead of writing a sentence
    #[clap(short, long = "raw")]
    raw_data: bool,
}

fn main() {
    let args = Cli::parse();

    let date = get_date_from_args(&args);

    let age = howoldisyohan::age(date);
    if args.raw_data {
        println!("{}",age);
    } else {
        println!("Johan is {} years old", age)
    }
}

//Takes the arguments of the command line, and extracts a date out of them if one is specified
fn get_date_from_args(args : &Cli) -> Date<Local> {
    match &args.date_option {
        //if a date was given as an argument
        Some(dt_str) => {
            //Parses the date
            let vec_date : Vec<u32> = dt_str.split('/')
                .into_iter()
                .map(|s| -> u32 { s.parse().unwrap() } )
                .collect();

            //tries to return a date
            match Local.ymd_opt(vec_date[2] as i32, vec_date[1], vec_date[0]) {
                LocalResult::Single(dt) => dt,
                _ => panic!("Incorrect syntax, use dd/mm/yyyy to specify a date"),
            }
        }
        //if no argument is given, return today's date
        None => Local::today(),
    }
}
