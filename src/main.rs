//! Demonstrate that you can have arrays of dyn references w/o heap allocation
//! through careful use of lifetimes.

trait FizzBuzzHelper {
    fn activate(&self, turn: u64) -> bool;

    fn print_value(&self) -> &'static str;
}

struct Fizz;

impl FizzBuzzHelper for Fizz {
    fn activate(&self, turn: u64) -> bool {
        turn.is_multiple_of(3)
    }

    fn print_value(&self) -> &'static str {
        "Fizz"
    }
}

struct Buzz;

impl FizzBuzzHelper for Buzz {
    fn activate(&self, turn: u64) -> bool {
        turn.is_multiple_of(5)
    }

    fn print_value(&self) -> &'static str {
        "Buzz"
    }
}

struct Baz;

impl FizzBuzzHelper for Baz {
    fn activate(&self, turn: u64) -> bool {
        turn.is_multiple_of(7)
    }

    fn print_value(&self) -> &'static str {
        "Baz"
    }
}

fn main() {
    #[allow(clippy::disallowed_names)]
    let (fizz, buzz, baz) = (Fizz, Buzz, Baz);

    let turns = 100;
    let helpers: Vec<&dyn FizzBuzzHelper> = Vec::from([
        &fizz as &dyn FizzBuzzHelper,
        &buzz as &dyn FizzBuzzHelper,
        &baz as &dyn FizzBuzzHelper,
    ]);

    for turn in 0..=turns {
        let mut print_value = String::new();

        for helper in &helpers {
            if helper.activate(turn) {
                print_value.push_str(helper.print_value());
            }
        }

        if print_value.is_empty() {
            println!("{turn}");
        } else {
            println!("{turn}: {print_value}");
        }
    }
}
