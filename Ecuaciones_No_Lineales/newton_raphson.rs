#[path = "funcion.rs"] mod funcion;

//Metodo de Newton-Raphson para la aproximacion de la solucion de ecuaciones no lineales
//Entradas:
//      x0: valor inicial
//      tol: tolerancia del error de la aproximacion
//      iter_max: cantidad de iteraciones maximas del metodo
//Salida:
//      x: aproximacion de un cero de la ecuacion

pub fn newton_raphson(x0: f64, tol: f64, iter_max: i32) -> f64{
    let mut x: f64;
    let mut xk: f64 = 0.0;
    let mut error: f64;
    
    x = x0;
    for _k in 0..iter_max  {
        if funcion::dfdx(x) != 0.0 { //Evaluar que el denominador no sea cero
            x = x - (funcion::f(x)/funcion::dfdx(x)); //Calculo siguiente aproximacion
            error = funcion::f(xk).abs(); //Calculo del error
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