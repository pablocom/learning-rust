fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("{:?}", home);
    println!("{:?}", loopback);

    let country_code = "ES";
    let meal = most_famous_meal(country_code);
    println!("Most famous meal in {:?}: {:?}", country_code, meal);
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn most_famous_meal(country_code: &str) -> Option<&str> {
    match country_code {
        "US" => Some("Something borrowed from another country I guess"),
        "UK" => Some("Fish and Chips"),
        "FR" => Some("Croissant"),
        "ES" => Some("Paella"),
        "NL" => None,
        other => {
            println!("No famous meal registered for '{:?}'", other);
            None
        }
    }
}
