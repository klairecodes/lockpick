use std::io;
use std::io::Write;
use std::collections::HashMap;
use rand::{
    distributions::{Distribution, Standard},
    Rng, thread_rng,
};
use rust_decimal::prelude::*;

//TODO: make these the keys in the texture map
#[derive(Debug)] // allows printing of this object
enum PinType {
    KEY,
    SPOOL,
    SERRATED,
    DRIVER,
}

#[derive(Debug)]
struct Pin {
    pin_type: PinType,
    height: Decimal,
    diameter: Decimal,
    unicode_repr: char,

}

#[derive(Debug)]
struct Spring {
    height: Decimal,
    diameter: Decimal,
    unicode_repr: char,
    weight: Decimal,
}

/*Returns a random pin type, excluding key pins. */
impl Distribution<PinType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PinType {
        match rng.gen_range(0..=3) {
            0 => PinType::SPOOL,
            1 => PinType::SERRATED,
            _ => PinType::DRIVER,
        }
    }
}

// generates a pin of a random type
fn get_random_pin() -> Pin {
    // let pt: PinType = rand::random();
    let mut rng = thread_rng();

    // This is done for proper rounding
    let mut rand_h = rng.gen_range(0.1..5.0);    
    let rand_height = Decimal::from_f64(rand_h);
    let mut rand_d = rng.gen_range(0.077..0.115);    
    let rand_diameter = Decimal::from_f64(rand_d);

    let pin = Pin {
        pin_type: rand::random(),
        height: rand_height.unwrap().round_dp_with_strategy(1, RoundingStrategy::ToZero),
        diameter: rand_diameter.unwrap().round_dp_with_strategy(3, RoundingStrategy::ToZero),
        unicode_repr: '█',
    };
    return pin;
}

fn main() {
    // let mut input = String::new();

    // io::stdin()
        // .read_line(&mut input)
        // .expect("Failed to read line, skill issue.");


    io::stdout().flush().unwrap();
    // println!("You entered: {}.", input.trim());
    // let pin_texture_map: HashMap::from([PinType::KEY, '█',
    // ]);
    // map.insert(Pin_type::Key, key_char)

    println!("{:?}", get_random_pin());
}
