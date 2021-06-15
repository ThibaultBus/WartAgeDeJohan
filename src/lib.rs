use chrono::{Date, Local, TimeZone};

pub fn age(at: Date<Local>) -> i64 {
    let birth : Date<Local> = Local.ymd(2019, 11, 12);

    // Calculate the age of yohan

    let diff = at - birth;
    // We calculate his age : the number of days since 12/11/2019, minus the number of non-birthdays (every 12/11), plus his age the 12/11/2019
    (diff.num_days() - ((diff.num_days() - 1) / 365)) + 18
}
