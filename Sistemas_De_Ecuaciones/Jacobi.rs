const tam: usize = 3;


// La funcion main controla todo lo que se va a ejecutar , crea los parametros de entrada y
//los envia a la funcion Jacobi

fn main() {

    // Constantes para inicializar la funcion
    let mut a =[[5.0,1.0,0.0],[1.0,5.0,1.0],[0.0,1.0,5.0]];
    let mut b=[6.0,7.0,6.0];
    let mut x0=[0.0,0.0,0.0];
    let iter_max = 1000;
    let tol =1e-10;
    jacobi(a, b, x0, iter_max, tol)


}

fn jacobi(A: [[f64;tam]; tam], b: [f64;tam], x0: [f64;tam],  iter_max: i32, tol: f64){
    let mut xk = x0;

    //Paso 1. Obtener la matriz D, la cual es la diagonal de a
    let d = diagonal_vector(A);
    let D = diagonal(d);
    //Paso 2. Calcular L + V = A - D
    let LmU = resta_de_matriz(A, D);

    //Paso 3. Calcular la inversa de D
    let D_in = matrix_inversa_diagonal(D);
    // Paso 4. Obtener z = D^1*b y M =
    let z =  multiplicacion_matriz_vector(D_in, b);
    let M = multiplicacion_matrices_cuadradas(matriz_negativa(D_in), LmU);
    let mut iter = 0;
    let mut errorPrevio = 0.0;
    let mut error = 0.0;
    // Paso 5
    for k in 0..iter_max {
        xk = suma_de_vectores(multiplicacion_matriz_vector(M, xk), z);
        let n_b = vector_negativa(b);
        errorPrevio  = error;
        error = norma_simple(suma_de_vectores(multiplicacion_matriz_vector(A, xk), n_b));
        if (error < tol) | (error == errorPrevio) {
            iter = k;
            break
        }
    }
    println!("Iteraciones: {}", iter);
    println!("Error: {}", error);
    println!("Xk");
    imprimir_vector_simple(&xk);

}

//Retorna la diagonal de un matriz de forma de vector
fn diagonal_vector(A: [[f64;tam]; tam])-> [f64; tam]{
    let mut x = [0f64; tam];
    for i in 0..tam {
        x[i] =  A[i][i];
    }
    return x;
}

//Retorna una matriz de 0 con su diagonal rellenada con los valores de un vector
fn diagonal(vector: [f64;tam])-> [[f64;tam]; tam]{
    let mut x = [[0f64; tam]; tam];
    for i in 0..tam {
        x[i][i] =  vector[i];
    }
    return x;
}

// Funcion util para imprimir un vector de forma amigable
fn imprimir_vector_simple(slice: &[f64;tam]){
    for i in 0..tam {
        print!("{}  ", slice[i]);
    }
    println!();
}

// Funcion util para imprimir una matriz de forma amigable
fn imprimir_matriz(A: [[f64;tam];tam]){
    for i in 0..tam {
        imprimir_vector_simple(&A[i]);
    }
    println!();
}

//Esta funcion resta dos matrices del mismo tamaño
fn resta_de_matriz(A: [[f64;tam];tam],B: [[f64;tam];tam])-> [[f64;tam];tam]{
    let mut resultado = [[0f64;tam];tam];
    for i in 0..tam {
        for j in 0..tam{
            resultado[i][j]=A[i][j]-B[i][j];
        }
    }
    return resultado;
}

//Esta funcion suma dos matrices del mismo tamaño
fn suma_de_matriz(A: [[f64;tam];tam],B: [[f64;tam];tam])-> [[f64;tam];tam]{
    let mut resultado = [[0f64;tam];tam];
    for i in 0..tam {
        for j in 0..tam{
            resultado[i][j]=A[i][j]+B[i][j];
        }
    }
    return resultado;
}

//Esta funcion suma dos matrices del mismo tamaño
fn suma_de_vectores(A: [f64;tam],B: [f64;tam])-> [f64;tam]{
    let mut resultado = [0f64;tam];
    for i in 0..tam {
        resultado[i] = A[i]+B[i];
    }
    return resultado;
}


// Obtiene la matriz inversa de la diagonal de una matriz
fn matrix_inversa_diagonal(A: [[f64; tam]; tam]) -> [[f64;tam]; tam]{
    let mut x = [[0f64; tam]; tam];
    for i in 0..tam {
        x[i][i] =  1.0 / A[i][i];
    }
    return x;
}

// Multiplica dos matrices cuadradas de 3x3
fn multiplicacion_matrices_cuadradas(A: [[f64;tam];tam],B: [[f64;tam];tam])-> [[f64;tam];tam]{
    let mut x = [[0f64; tam]; tam];
    for i in 0..tam {
        for j in 0..tam {
            for k in 0..tam {
                x[i][j] += A[i][k] * B[k][j]
            }
        }
    }

    return x;
}

//  Esta funcion multiplica una matriz 3x3 con un vector 3x1. Obteniendo un vector de 3x1
fn multiplicacion_matriz_vector(A: [[f64;tam]; tam] ,xk:[f64;tam])-> [f64; tam]{
    let mut resultado = [0f64; tam];
    let mut aux=0.0;
    for i in 0..tam {
        aux=0.0;
        for j in 0..tam {
            aux=aux+xk[j]*A[i][j];
        }
        resultado[i]=aux;

    }
    return resultado;
}

// Aplica un signo negativo a una matriz
fn matriz_negativa(mut A: [[f64;tam]; tam]) -> [[f64;tam]; tam]{
    for i in 0..tam {
        for j in 0..tam {
            A[i][j]= -A[i][j];
        }
    }
    return A;
}

// Aplica un signo negativo a un vector
fn vector_negativa(mut A: [f64;tam]) -> [f64;tam]{
    for i in 0..tam {
        A[i] = -A[i]
    }
    return A;
}

//Esta funcion calcula la norma de la matriz simple ingresada
fn norma_simple(A:[f64;tam])-> f64{
    let mut numeros:f64=0.0;
    let mut norma:f64=0.0;
    for i in 0..tam {
        numeros = A[i].powf(2 as f64)+numeros as f64;
    }
    norma= numeros.powf(0.5 as f64) as f64;
    return norma;
}
