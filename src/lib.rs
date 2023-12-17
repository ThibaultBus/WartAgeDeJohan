use chrono::{NaiveDate};

// TODO: Unwrap the option when const panicking is stabilized
const BIRTH: Option<NaiveDate> = NaiveDate::from_ymd_opt(2019, 11, 12);

pub fn age(at: NaiveDate) -> i64 {
    let birth = BIRTH.unwrap();

    // Calculate the age of yohan

    let diff = at - birth;
    // We calculate his age : the number of days since 12/11/2019, minus the number of non-birthdays (every 12/11), plus his age the 12/11/2019
    (diff.num_days() - ((diff.num_days() - 1) / 365)) + 18
}
