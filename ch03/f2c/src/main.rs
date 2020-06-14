use std::io;

fn main() {
    let units = input_units();
    let from_degree = input_degree();
    let to_degree = convert(units, from_degree);
    println!("{} ({}) -> {} ({})", from_degree, units.0, to_degree, units.1)
}

const CC: (char,char) = ('C','C');
const FF: (char,char) = ('F','F');
const CF: (char,char) = ('C','F');
const FC: (char,char) = ('F','C');

const CF_FACTOR      : f32 = 9.0 / 5.0;
const CF_DISPLACEMENT: f32 = 32.0;

fn convert(units: (char,char), from_degree: f32) -> f32 {
    match units {
        CF =>  (from_degree * CF_FACTOR) + CF_DISPLACEMENT,
        FC => (from_degree - CF_DISPLACEMENT) / CF_FACTOR,
        CC => from_degree,
        FF => from_degree,
        _ => panic!("Bad units")
    }
}

fn input_degree() -> f32 {
    println!("Enter degree to convert");

    let mut degree = String::new();
    io::stdin().read_line(&mut degree)
        .expect("Failed to read line");
    degree.trim().parse().expect("Bad float")
}

fn input_units() -> (char,char) {
    let from_unit: char = input_unit("source");
    let to_unit:   char = input_unit("destination");

    (from_unit, to_unit)
}

fn input_unit(description: &str) -> char {
    println!("Enter {} unit (C or F)", description);

    let mut unit = String::new();
    io::stdin().read_line(&mut unit)
        .expect("Failed to read line");
    let unit: char = unit.trim().to_uppercase().to_string().parse()
        .expect("Expected single character");
    
    if unit == 'C' || unit == 'F' {
        unit
    } else {
        panic!("Bad unit");
    }
}