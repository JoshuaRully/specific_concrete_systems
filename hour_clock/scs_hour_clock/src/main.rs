#[derive(Debug)]
enum Hours {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
}
struct HoursClock {
    hour: Hours,
}

impl HoursClock {
    fn new() -> Self {
        HoursClock { hour: Hours::Twelve}
    }

    fn increment_hour(&self) -> Hours {
        match self.hour {
            Hours::One => Hours::Two,
            Hours::Two => Hours::Three,
            Hours::Three => Hours::Four,
            Hours::Four => Hours::Five,
            Hours::Five => Hours::Six,
            Hours::Six => Hours::Seven,
            Hours::Seven => Hours::Eight,
            Hours::Eight => Hours::Nine,
            Hours::Nine => Hours::Ten,
            Hours::Ten => Hours::Eleven,
            Hours::Eleven => Hours::Twelve,
            Hours::Twelve => Hours::One,
        }
    }
}


// CLI program?
fn main() {
    let hours_clock = HoursClock::new();
    println!("The hour is {:?}", hours_clock.increment_hour());
}

// TODO: write tests :)