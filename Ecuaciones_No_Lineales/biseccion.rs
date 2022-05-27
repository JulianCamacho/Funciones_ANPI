#[path = "funcion.rs"] mod funcion;

//Metodo de la biseccion para la aproximacion de la solucion de ecuaciones no lineales
//Entradas:
//      a: valor inicial del intervalo
//      b: valor final del intervalo
//      tol: tolerancia del error de la aproximacion
//      iter_max: cantidad de iteraciones maximas del metodo
//Salida:
//      x: aproximacion de un cero de la ecuacion

pub fn biseccion(mut a: f64, mut b: f64, tol: f64, iter_max: i32) -> f64{
    let mut x: f64 = 0.0;
    let mut error: f64;

    //Verificar Teorema de Bolzano
    if funcion::f(a) * funcion::f(b) < 0.0 {      //Paso 1
        for _k in 1..iter_max{
            x = (a +b)/2.0;      //Paso 2: Dividir el intervalo
            if funcion::f(a) * funcion::f(x) < 0.0 {    //Paso 3: asignar nuevo intervalo
                b = x;
            } else{
                a = x;
            } 
            error = funcion::f(x).abs(); //Calcular error de la función
            if error < tol {     //Paso 4
                break;
            } 
        } 
    }       
    else{
        println!("La función en el intervalo no cumple con el teorema de Bolzano");
    }
    println!("{}", x);
    return x;
}