use ::std::io;

fn main() {
    println!("enter weight in kg");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let weight: f32 = input.trim().parse().unwrap(); 
    // dbg!(weight);

    let weight_on_mars = calculate_weight_on_mars(weight);

    println!("weight on earth {}", input);
    println!("weight on mars is {}", weight_on_mars);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
