// CLI program?
fn main() {
    let twelve = Hours::Twelve;
    println!("The hour is {:?}", increment_hours(twelve));
}

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

// TODO: refactor to impl; handle errors?
fn increment_hours(hr: Hours) -> Hours {
    match hr {
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

// TODO: write tests :)