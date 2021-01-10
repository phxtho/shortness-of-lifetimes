use chrono::{Utc, Datelike};
use indicatif::ProgressBar;


fn main() {
    let bar = ProgressBar::new(72);
    bar.inc(50);
    progress(1970);
}

fn progress(year_of_birth: i32) {
    let avg_lifespan = 72;
    let age = Utc::now().year() - year_of_birth;
    println!("age {}", age);
    println!("years left {}",avg_lifespan - age);
}
