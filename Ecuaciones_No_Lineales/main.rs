mod biseccion;
mod secante;
mod falsa_posicion;
mod newton_raphson;
mod funcion;

fn main() {    
    let tol = f64::powf(10.0, -5.0);
    let iter_max: i32 = 1000;

    let a = 2 as f64;
    let b = 3 as f64;

    println!("Aproximación de un cero de la función: f(x) = exp(x)-2*x-10");
    println!("en el intervalo [2,3] con tolerancia de 10^-5");

    println!("Aproximación por el método de la bisección");
    biseccion::biseccion(a, b, tol, iter_max);

    println!("Aproximación por el método de la secante");
    secante::secante(a, b, tol, iter_max);
    
    println!("Aproximación por el método de la falsa posición");
    falsa_posicion::falsa_posicion(a, b, tol, iter_max);
    
    println!("Aproximación por el método de Newton Raphson");
    newton_raphson::newton_raphson(a, tol, iter_max);




}
