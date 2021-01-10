use chrono::{Utc, Datelike};
use indicatif::{ProgressBar, ProgressStyle};

const AVG_LIFE: i32 = 72;

fn main() {
    println!("Life Progress");
    let bar = ProgressBar::new(AVG_LIFE as u64);
    bar.set_style(ProgressStyle::default_bar().template("{bar:40.cyan/blue} {percent}%"));
    bar.inc(age(1997) as u64);
    println!();
}

fn age(year_of_birth: i32) -> i32{
    let years_elapsed = Utc::now().year() - year_of_birth;
    return years_elapsed;
}
