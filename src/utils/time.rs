use chrono::{NaiveDate, NaiveDateTime};

pub fn display_timestamp(ts: u64) -> String {
    //todo -get nano part
    let datetime = NaiveDateTime::from_timestamp((ts / 1000) as i64, 0); 
    datetime.format("%b %d %Y %l:%M:%S").to_string()
}
