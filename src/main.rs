use std::io;


fn main() {
    println!("Input the nth number for the fibonacci formula:");

    let mut times = String::new();
    io::stdin().read_line(&mut times);
    let times: f32 = times.trim().parse().expect("Please enter a number!"); 

    println!("{}", fibonacci(times));
}

fn fibonacci(mut n:f32) -> f32 {
    let phi: f32 = (1. + 5_f32.sqrt())/2.;
    n = n - 1.0;
    return ((phi.powf(n) - (-phi).powf(-n)) / (5_f32).sqrt()).round();
}
