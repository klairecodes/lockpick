use std::io;
use std::io::Write;
use std::collections::HashMap;
use rand::{
    distributions::{Distribution, Standard},
    Rng, thread_rng,
};

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
    height: f64,
    diameter: f64,
    unicode_repr: char,

}

#[derive(Debug)]
struct Spring {
    height: f64,
    diameter: f64,
    unicode_repr: char,
    weight: f64,
}

impl Distribution<PinType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PinType {
        match rng.gen_range(0..=3) {
            0 => PinType::KEY,
            1 => PinType::SPOOL,
            2 => PinType::SERRATED,
            _ => PinType::DRIVER,
        }
    }
}

// generates a pin of a random type
fn get_random_pin() -> Pin {
    // let pt: PinType = rand::random();
    let mut rng = thread_rng();

    // This is done for proper rounding
    let x: f64 = rng.gen_range(0.1..5.0);
    let h: f64 = x * 100.0_f64.round() / 100.0;
    let x: f64 = rng.gen_range(0.077..0.115);
    let d: f64 = x * 100.0_f64.round() / 100.0;

    let pin = Pin {
        pin_type: rand::random(),
        height: h,
        diameter: d,
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
