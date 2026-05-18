fn main() {
    let ip_addresses = [
        IpAddr::V4(127, 0, 0, 1),
        IpAddr::V6(String::from("::1")),
        IpAddr::V6(String::from("14a3:f0ce:63b9:6733:adc9:b77d:bad1:adc0")),
        IpAddr::V4(1, 1, 1, 1),
    ];

    for ip_address in ip_addresses {
        match ip_address {
            IpAddr::V4(a, b, c, d) => println!("IPv4: {}.{}.{}.{}", a, b, c, d),
            IpAddr::V6(addr) => println!("IPv6: {}", addr),
        };
    }

    let country_code = "ES";
    let meal = most_famous_meal(country_code);
    println!("Most famous meal in {:?}: {:?}", country_code, meal);
}

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
