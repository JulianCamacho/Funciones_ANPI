#[path = "funcion.rs"] mod funcion;

//Metodo de la secante para la aproximacion de la solucion de ecuaciones no lineales
//Entradas:
//      x0: valor inicial del intervalo
//      x1: valor final del intervalo
//      tol: tolerancia del error de la aproximacion
//      iter_max: cantidad de iteraciones maximas del metodo
//Salida:
//      x: aproximacion de un cero de la ecuacion

pub fn secante(x0: f64,x1: f64, tol: f64, iter_max: i32) -> f64{
    let mut xk: f64 = x1;
    let mut xkm1: f64 = x0;
    let mut error: f64;
    let mut tmp: f64;
    let mut denom: f64;

    for _k in 0..iter_max {
    denom = funcion::f(xk) - funcion::f(xkm1);
    if denom.abs() > 0.0{ //Evaluar que el denominador no sea cero
        tmp = xk;
        xk = xk - ((xk-xkm1)*funcion::f(xk) / denom); //Calculo siguiente aproximacion
        xkm1 = tmp;

        error= funcion::f(xk).abs(); //Calculo del error
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