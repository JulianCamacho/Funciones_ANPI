#[path = "funcion.rs"] mod funcion;

pub fn secante(x0: f64,x1: f64, tol: f64, iter_max: i32) -> f64{
    let mut xk: f64 = x1;
    let mut xkm1: f64 = x0;
    let mut error: f64;
    let mut tmp: f64;
    let mut denom: f64;

    for _k in 0..iter_max {
    denom = funcion::f(xk) - funcion::f(xkm1);
    if denom.abs() > 0.0{
        tmp = xk;
        xk = xk - ((xk-xkm1)*funcion::f(xk) / denom);
        xkm1 = tmp;

        error= funcion::f(xk).abs();
        if error < tol {
            break;
        }
    } else{
        println!("El denominador se hizo cero");
        break;
    }
    }
    println!("{}", xk);
    return xk;
}