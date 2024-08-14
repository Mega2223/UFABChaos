
fn main(){
    println!("\n\nColisão elástica:\n");
	let mut v1:f64 = 0.094;
    let mut m1:f64 = 0.1102;
    let mut v2:f64 = 1.87;
    let mut m2:f64 = 0.11;

    let mut v1err : f64 = 0.006;
    let mut v2err : f64 = 0.018;

    println!("v1 = {v1}, m1 = {m1}\nv2 = {v2}, m2 = {m2}");
    println!("errV1 = {v1err}, errV2 = {v2err}\n\n");
    println!("Pré colisão: ");
    let mut kinC1 = calculate_kin_energy(v1,m1);
    let mut errC1 = simple_exponential_error(v1err,v1,kinC1,2.0,"(Erro K)/2");
    errC1/=2.0;
    println!("(Erro K) = {errC1}");
    calculate_linear_momentum(v1,m1);
    println!("(Erro P) = (Erro V) = {v1err}");

    println!("\nPós colisão: ");
    kinC1 = calculate_kin_energy(v2,m2);
    errC1 = simple_exponential_error(v1err,v1,kinC1,2.0,"(Erro K)/2");
    errC1/=2.0;
    println!("(Erro K) = {errC1}");
    calculate_linear_momentum(v2,m2);
    println!("(Erro P) = (Erro V) = {v2err}");



    println!("\n\n\n\nColisão inelástica\n");
    m1 = 1.085; m2 = 1.304 + m1;
    v1 = 0.0451; v2 = 0.0638;
    v1err = 0.0083; v2err = 0.0008;
    println!("v1 = {v1}, m1 = {m1}\nv2 = {v2}, m2 = {m2}");
    println!("errV1 = {v1err}, errV2 = {v2err}\n\n");
    println!("Pré colisão: ");
    kinC1 = calculate_kin_energy(v1,m1);
    errC1 = simple_exponential_error(v1err,v1,kinC1,2.0,"(Erro K)/2");
    errC1/=2.0;
    println!("(Erro K) = {errC1}");
    calculate_linear_momentum(v1,m1);
    println!("(Erro P) = (Erro V) = {v1err}");

    println!("\nPós colisão: ");
    kinC1 = calculate_kin_energy(v2,m2+m1);
    errC1 = simple_exponential_error(v1err,v1,kinC1,2.0,"(Erro K)/2");
    errC1/=2.0;
    println!("(Erro K) = {errC1}");
    calculate_linear_momentum(v2,m2+m1);
    println!("(Erro P) = (Erro V) = {v2err}");


}

fn calculate_kin_energy (vel:f64,mass:f64) -> f64{
    print!("Energia cinética = {vel}² * {mass} * (1/2) =");
    let v_sq = vel * vel;
    let ret = (v_sq * mass) * 0.5;
    println!(" {ret}");
    ret
}

fn calculate_linear_momentum(vel:f64,mass:f64) -> f64{
    print!("Momento linear = {vel} * {mass} =");
    let ret = vel * mass;
    println!(" {ret}");
    ret
}

fn add_error_constant(error:f64,constant:f64,prefix:&str) -> f64{
    print!("{prefix} = |{constant}| {error} =");
    let ret =  f64::abs(constant) * error;
    println!(" {ret}");
    ret
}

fn simple_exponential_error(initialError:f64,initialValue:f64,finalValue:f64,exponent:f64,prefix:&str) -> f64 {
    let mut res:f64 = initialError/initialValue;
    res = f64::abs(res);
    print!("{prefix} = |{initialError}/{initialValue}*{exponent}|*{finalValue} = ");
    res *= exponent;
    res *= finalValue;
    println!("{res}");
    return res;
}
