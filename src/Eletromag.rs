use std::io;

fn main(){
    println!("How many data points?");

    let mut data_amount = String::new();
    io::stdin().read_line(&mut data_amount);
    let data_amount: u32 = data_amount.trim().parse().expect("invalid number :(");

    for n in 0 .. data_amount {
        println!("θ{n}(°) = ");
        let mut teta = String::new();
        io::stdin().read_line(&mut teta);
        let mut teta: f32 = teta.trim().parse().expect("g");
        let tang: f32 = teta.to_radians().tan();
        let b_teta: f32 = 18.0 * tang;
        let b_error: f32 = (18.0 / teta.to_radians().cos().cos().powf(2.0)) * (std::f32::consts::PI/180.0);
        println!("tg(θ) = {tang}\nB(θ) = {b_teta}\nerrB(θ)={b_error}");
    }
}