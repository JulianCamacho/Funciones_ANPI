#[path = "funcion.rs"] mod funcion;

pub fn falsa_posicion(mut a: f64, mut b: f64, tol: f64, iter_max: i32) -> f64{
    let x0: f64;
    let x1: f64;
    let mut xk: f64 = 0.0;
    let mut xkp1: f64;
    let mut error: f64;

    if funcion::f(a) * funcion::f(b) < 0.0 {
        x0 = a;
        x1 = b;
        xk = x1 - ((x1-x0)*funcion::f(x1) / (funcion::f(x1) - funcion::f(x0)));
        // Paso 3
        for _k in 2..iter_max{
            if funcion::f(a)*funcion::f(xk) < 0.0 { // Pasos 3.1 y 3.2
            xkp1 = xk - ((xk-a)*funcion::f(xk) / (funcion::f(xk) - funcion::f(a)));
            b = xk;
            } else{
            xkp1 = xk - ((xk-b)*funcion::f(xk) / (funcion::f(xk) - funcion::f(b)));
            a = xk;
            }
            xk = xkp1;
            //Paso 3.3
            error = funcion::f(xk).abs(); 
            if error < tol {
            break;
            }
        }
    } else{
    println!("No cumple las condiciones")
    }
    println!("{}", xk);
    return xk;
}