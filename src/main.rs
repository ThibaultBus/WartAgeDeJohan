use chrono::prelude::*;
use chrono::offset::LocalResult;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    //Allows to specify a date instead of taking today's one
    #[structopt(short = "d", long = "date")]
    date_option : Option<String>,
    //Make the command prints raw data of writing a sentence
    #[structopt(short = "r", long = "raw")]
    raw_data : bool,
}

fn main() {
    let args = Cli::from_args();

    let date = get_date_from_args(&args);

    let birth : Date<Local> = Local.ymd(2019, 11, 12);

    //Calculates and prints the age of yohan
    
    let diff = date - birth;
    //We calculate his age : the number of days since 12/11/2019, minus the number of non-birthdays (every 12/11), plus his age the 12/11/2019
    let age = (diff.num_days() - ((diff.num_days() - 1) / 365)) + 18;
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
