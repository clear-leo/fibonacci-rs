use shittyinput;


fn main() {
    println!("Input the nth number for the fibonacci formula:");

    let times = shittyinput::float::f32();

    println!("{}", fibonacci(times));
}

fn fibonacci(mut n:f32) -> f32 {
    let phi: f32 = (1. + 5_f32.sqrt())/2.;
    n = n - 1.0;
    return ((phi.powf(n) - (-phi).powf(-n)) / (5_f32).sqrt()).round();
}
