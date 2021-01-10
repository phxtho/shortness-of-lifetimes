use chrono::{Utc, Datelike};
use indicatif::{ProgressBar, ProgressStyle};
use structopt::StructOpt;

const AVG_LIFE: i32 = 72;

#[derive(StructOpt)]
struct Cli{
    year_of_birth: i32
}

fn main() {
    let args = Cli::from_args();
    println!("Life Progress");
    let bar = ProgressBar::new(AVG_LIFE as u64);
    bar.set_style(ProgressStyle::default_bar().template("{bar:40.cyan/blue} {percent}%"));
    bar.inc(age(args.year_of_birth) as u64);
    println!();
}

fn age(year_of_birth: i32) -> i32{
    let years_elapsed = Utc::now().year() - year_of_birth;
    return years_elapsed;
}
