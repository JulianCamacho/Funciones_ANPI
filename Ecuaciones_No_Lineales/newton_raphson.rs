#[path = "funcion.rs"] mod funcion;

pub fn newton_raphson(x0: f64, tol: f64, iter_max: i32) -> f64{
    let mut x: f64;
    let mut xk: f64 = 0.0;
    let mut error: f64;
    
    x = x0;
    for _k in 0..iter_max  {
        if funcion::dfdx(x) != 0.0 {
            x = x - (funcion::f(x)/funcion::dfdx(x));
            error = funcion::f(xk).abs(); 
            if error < tol {
                break;
            }
            xk = x;
        } else{
            println!("La derivada de la función se hizo cero");
        }
    }
    println!("{}", xk);
    return xk;
}