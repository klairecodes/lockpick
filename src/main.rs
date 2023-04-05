use std::io;
use std::io::Write;
use std::collections::HashMap;

//TODO: make these the keys in the texture map
enum PinType {
    KEY,
    DRIVER,
    SPOOL,
    SERRATED,
}

struct Pin {
    pin_type: PinType,
    height: f64,
    diameter: f64,
    unicode_repr: char,

}

struct Spring {
    height: f64,
    diameter: f64,
    unicode_repr: char,
    weight: f64,
}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line, skill issue.");


    io::stdout().flush().unwrap();
    println!("You entered: {}.", input.trim());
    // let pin_texture_map: HashMap::from([PinType::KEY, '█',
    // ]);
    // map.insert(Pin_type::Key, key_char)
}
