
// Método inicial que define las condiciones iniciales

fn main() {
    let iter_max = 50000;
    let tol =1e-10;
    let x0 = 0.5;
    steffensen(x0, tol, iter_max);

}

// Función a evaluar
// 2x^2 + 2x - 5

fn f(x: f64) -> f64 {
    return (2.0 * x * x) + (2.0*x) - 5.0
}

/*
Función basada en el método de Steffensen
Se utiliza para encontrar los ceros de una función por medio de una convergencia rápida
Una gran ventaja de este método es no utilizar derivadas.

Sus entradas son:
x0 = punto inicial para comenzar a evaluar
tol = valor mínimo de la tolerancia para verificar si el método convergió
iter_max = Iteraciones máximas a realizar la recursividad del algoritmo.

*/

fn steffensen(mut x0: f64, tol: f64, iter_max: i32) {

    // Calculo de la primera iteracción
    let mut x1 = x0 + f(x0);
    let mut x2 = x1 + f(x1);

    // Cálculo del Xn+1
    let mut x_nuevo = x2 -  (x2-x1).powf(2.0) / (x2 - (2.0 * x1) + x0);

    let mut error = (x_nuevo - x0).abs();

    let mut i = 0;

    // Recursividad del algoritmo
    while (i <= iter_max) && (error >= tol)  {
        x0 = x_nuevo;
        x1 = x0 + f(x0);
        x2 = x1 + f(x1);

        x_nuevo = x2 - (x2-x1).powf(2.0) / (x2 - (2.0 * x1) + x0);
        error = (x_nuevo - x0).abs();


        i += 1;

    }

    println!("Función a analizar: 2x^2 + 2x - 5");
    println!("Cantidad de iteraciones necesarias: {}", i);
    println!("Un cero de la función corresponde a: {}", x0);
}

/*
Referencias:
https://arturoguillen90.wordpress.com/aceleracion-de-convergencia/steffensen/
https://github.com/Abhay1994s/1.0-PythonBasics/blob/master/steffensen_part2.py
 */