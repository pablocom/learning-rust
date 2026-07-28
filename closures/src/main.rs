#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            return ShirtColor::Red;
        }
        return ShirtColor::Blue;
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_preference1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_preference1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_preference1, giveaway1
    );

    let user_preference2 = None;
    let giveaway2 = store.giveaway(user_preference2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_preference2, giveaway2
    );

    let mut numbers = vec![1, 2, 3];

    let print = || println!("From closure: {numbers:?}");

    println!("Before calling closure: {numbers:?}");
    print();
    numbers.push(4);
    println!("After calling closure: {numbers:?}");
}
