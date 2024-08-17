use std::io;
// Programa usado para desobrir valores da questao 3 do experimento 3 dee eletromag
fn main(){
    println!("b0 = ");
    let b0: f32 = collectFloat();
    let mut n: u32 = 1;
    while true {
        println!("θ{n}(°) = ");
        let mut teta = String::new();
        io::stdin().read_line(&mut teta);
        let mut teta: f32 = teta.trim().parse().expect("g");
        let tang: f32 = teta.to_radians().tan();
        let b_teta: f32 = b0 * tang;
        let b_error: f32 = (b0 / teta.to_radians().cos().cos().powf(2.0)) * (std::f32::consts::PI/180.0);
        println!("tg(θ) = {tang}\nB(θ) = {b_teta}\nerrB(θ)={b_error}\n");
        n+=1;
    }

    fn collectFloat() -> f32{
        let mut res = String::new();
        io::stdin().read_line(&mut res);
        let res: f32 = res.trim().parse().expect("invalid number :(");
        res
    }
}