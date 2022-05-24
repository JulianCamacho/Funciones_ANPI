const TAM: usize = 25;

// La funcion main controla por defecto las acciones a ejecutar, donde primero se crean los parametros de entrada y
//juego se llama a función gradiente_conjugado con dichos parametros.

fn main() {
    //Parametros de entrada para la funcion

    let a= matriz_tridiagonal();
    let b= llenar_vector();
    let x0=[0f64;TAM];
    
    //llamada a la funcion
    gradiente_conjugado(a,b,x0,10e-10,100);


}


/*
Función que resuelve un sistema de ecuaciones mediante el método del gradiente conjugado
  Input: 
   a: matriz que se va a resolver.
   b: Vector de resultados del sistema de ecuaciones.
   x0: vector inicial de soluciones.
   iterMax: numero de iteraciones máximas.
   tol: valor mínimo aceptable para el error.
  output: 
   xk: vector final de soluciones.
*/
fn gradiente_conjugado(a: [[f64;TAM]; TAM] ,b:[f64;TAM],x0:[f64;TAM],tol: f64,iter_max: i32)-> [f64; TAM]{

    let mut xk = x0;

    let mut rk;

    let mut ak;

    let mut error=0.0;
    
    for k in 0..iter_max {

        rk=resta_vectores(b,multi_matriz_vector(a,xk));
        
        ak=multi_vectort_vector(rk, rk)/multi_vectort_vector(rk, multi_matriz_vector(a,rk));
        
        xk=suma_vectores(xk, multi_escalar_vector(rk, ak));
        
        error=norma(resta_vectores(b,multi_matriz_vector(a,xk)));
        
        if error < tol {
            break
        }
    }


    println!("########## Solucion xk ##########");
    imprimir_vector(&xk);

    println!("#################################");

    println!("Error = {}", error );

    return xk;
}

//Esta funcion crea una matriz tridiagonal de 25x25
//Input: NA
//output: Matriz tridiagonal de tamaño 25x25 donde A(i, i) = 2i, A(i, i + 1) = i, A(i, i − 1) = i − 1
        //para todo i = 1, ..., 25.
fn matriz_tridiagonal()-> [[f64; TAM];TAM]{
    let mut m = [[0f64; TAM];TAM];
    m[0][0]=2.0;
    m[0][1]=1.0;
    m[TAM-1][TAM-1]=50.0;
    m[TAM-1][TAM-2]=24.0;

    for i in 1..TAM-1 {
        m[i][i]=2.0*(i as f64 +1f64);
        m[i][i-1]=(i as f64 + 1f64)-1.0;
        m[i][i+1]=i as f64 + 1f64;
    }
    return m;
}

//Función que imprime el contenido de una matriz
//Input: Matriz a imprimir
//output: Matriz mejor ordenada
fn imprimir_matriz(slice: &[[f64;TAM]; TAM]){
    for i in 0..TAM {
        for j in 0..TAM {
            print!("{}  ", slice[i][j]);
        }
        println!("");
    }
}

//Función que se encarga de crear el vector de soluciones b
//Input: NA
//output: Vector b e R^25 donde b(i) = i
fn llenar_vector()->[f64;TAM]{
    let mut b = [0f64; TAM];
    for i in 0..TAM {
        b[i]=i as f64 +1f64;
    }
    return b;
}


//Función que imprime el contenido de un vector
//Input: vector a imprimir
//output: vector mejor ordenado
fn imprimir_vector(slice: &[f64;TAM]){
    for i in 0..TAM {
            print!("{}  ", slice[i]);

    }
    println!("");
}


//Función que calcula la norma del vector ingresado
//Input: Vector
//output: Norma del vector
fn norma(a:[f64;TAM])-> f64{
    let mut numeros:f64=0.0;
    let mut norma:f64=0.0;
    for i in 0..TAM {
        numeros = a[i].powf(2 as f64)+numeros as f64;
    }
    norma= numeros.powf(0.5 as f64) as f64;
    return norma;
}

//Funcion que resta dos vectores del mismo tamaño
//Input: a = primer vector, b = segundo vector
//output: resultado de la resta de a-b
fn resta_vectores(a: [f64;TAM],b: [f64;TAM])-> [f64; TAM]{
    let mut resultado = [0f64; TAM];
    for i in 0..TAM {
            resultado[i]=a[i]-b[i];
    }
    return resultado;
}

//Funcion que multiplica una matriz normal por una matriz/vector columna
//Input: a = matriz normal, xk = vector columna
//output: vector resultante
fn multi_matriz_vector(a: [[f64;TAM]; TAM] ,xk:[f64;TAM])-> [f64; TAM]{
    let mut resultado = [0f64; TAM];
    let mut aux=0.0;
    for i in 0..TAM {
        aux=0.0;
        for j in 0..TAM {
            aux=aux+xk[j]*a[i][j];
        }
        resultado[i]=aux;

    }
    return resultado;
}

///Funcion que multiplica un vector por su transpuesto
//Input: v1 = primer vector, v2 = segundo vector
//output: valor resultante 
fn multi_vectort_vector(v1: [f64;TAM] ,v2:[f64;TAM])-> f64{
    let mut resultado=0.0;
    for i in 0..TAM {
        resultado=resultado+v1[i]*v2[i];
    }
    return resultado;
}


//Funcion que suma dos vectores  
//Input: a = primera matriz, b = segunda matriz
//output: vector resultante de a + b
fn suma_vectores(a: [f64;TAM],b: [f64;TAM])-> [f64; TAM]{
    let mut resultado = [0f64; TAM];
    for i in 0..TAM {
            resultado[i]=a[i]+b[i];
    }
    return resultado;
}

//Funcion que multiplica vector por un valor escalar
//Input: a = vector, b = valor escalar
//output: vector resultante de b*a[]
fn multi_escalar_vector(a: [f64;TAM],b: f64)-> [f64; TAM]{
    let mut resultado = [0f64; TAM];
    for i in 0..TAM {
            resultado[i]=a[i]*b;
    }
    return resultado;
}

