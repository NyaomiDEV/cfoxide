use std::str::FromStr;

use cfoxide::CF;

fn main() {
    // Try it out here!
    let cf = CF::from_str("Test").unwrap();

    println!("{}", cf.get_name_surname_block());
    println!("{}", if cf.get_gender() { "Man" } else { "Woman" } );
    println!("{}/{}/{}", cf.get_day(), cf.get_month(), cf.get_year());
    println!("{} {}", cf.get_municipality(), cf.get_check_letter());
}