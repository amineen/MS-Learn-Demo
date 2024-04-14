#![allow(unused)]

#[derive(Debug)]
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age, u32),
}

#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

#[derive(PartialEq, Debug)]
enum Age {
    New,
    Used,
}

fn car_quality(miles: u32) -> (Age, u32) {
    if miles > 0 {
        return (Age::Used, miles);
    }

    (Age::New, miles)
}

fn car_factory(order: i32, miles: u32) -> Car {
    let colors = ["Blue", "Green", "Red", "Silver"];
    let color_index = order % 4;

    let color = colors[color_index as usize];

    Car {
        color: color.to_string(),
        motor: Transmission::Manual,
        roof: true,
        age: car_quality(miles),
    }
}
fn main() {
    let car = car_factory(2, 0);
    println!("{:?}", car);
}
