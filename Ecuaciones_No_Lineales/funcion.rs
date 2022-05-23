pub fn f(x: f64) -> f64{
    //f(x) = exp(x)-2*x-10;
    let e = std::f64::consts::E;
    let y = f64::powf(e, x) - 2.0*x - 10.0;
    //println!("{}", y);
    return y;
}

pub fn dfdx(x: f64) -> f64{
    //f(x) = exp(x)-2;
    let e = std::f64::consts::E;
    let y = f64::powf(e, x) - 2.0;
    return y;
}

