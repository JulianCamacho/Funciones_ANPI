use std::num;
const tam: usize = 3;
// La funcion main controla todo lo que se va a ejecutar , crea los parametros de entrada y
//los envia a la funcion Gauss_Seidel
fn main() {
    let mut A=[[2.0,-1.0,0.0],[-1.0,2.0,-1.0],[0.0,-1.0,2.0]];
    let mut b=[124.0,4.0,14.0];
    let mut x0=[1.0,4.0,2.0];
    //descomposicion_U(state);
    //suma_de_matriz(state,state2);
    //multiplicacion_matriz(A,b);
    //descomposicion_L(state);
    //descomposicion_L(&state);
    //imprimir_matriz(&state);
    Gauss_Seidel(A,b,x0,0.00000001,10);
    //sustitucion_adelante(A,b);
}

fn Gauss_Seidel(A: [[f64;tam]; tam] ,b:[f64;tam],x0:[f64;tam],tol: f64,iterMax: i32)-> [f64; tam]{
    let mut xk = x0;
    let mut L = descomposicion_L(A);
    let mut D = descomposicion_D(A);
    let mut U = descomposicion_U(A);
    let mut y = sustitucion_adelante(suma_de_matriz(L,D),b);

    let mut zk;
    let mut error_nor=0.0;
    for j in 1..iterMax {
        zk=sustitucion_adelante(suma_de_matriz(L,D),multiplicacion_matriz(U,xk));
        xk=suma_de_matriz2(zk,y);
        error_nor=norma_simple(resta_de_matriz(multiplicacion_matriz(A,xk),b));
        //println!("Error {} = {}",j, error_nor );
        if (error_nor < tol) {
            break
        }
    }
    imprimir_matriz_simple(&xk);
    println!("Error = {}", error_nor );
    return xk;
}
//-------------------------------------------------------------------------------


//Esta funcion de encarga de realizar el algoritmo de sustitucion hacia delante , retornando una
//solucion para ese sistema de matrices
fn sustitucion_adelante(A: [[f64;tam]; tam] ,b:[f64;tam])-> [f64; tam]{
    let mut x = [0f64; tam];
    let mut aux = 0.0;
    for i in 0..tam {
        aux=0.0;
        for j in 0..i {
            aux=aux+A[i][j]*x[j];
            //println!("Aux -->{}  A->{}  x->{}",aux,A[i][j], x[j]);
        }
        x[i]=(1.0/A[i][i])*(b[i]-aux);
    }
    //imprimir_matriz_simple(&x);
    return x;
}
//Descompone la matriz principal en el triangulo inferior del triangulo
fn descomposicion_U(slice: [[f64;tam]; tam])-> [[f64; tam]; tam]{
    let mut u = [[0f64; tam]; tam];
    for j in 0..tam {
        for i in j+1..tam {
            u[i][j]=slice[i][j];
        }
    }
    //println!("---------Descomposicion U------------------" );
    //imprimir_matriz(&u);
    return u;
}
//Descompone la matriz principal en el triangulo superior del triangulo
fn descomposicion_L(slice: [[f64;tam]; tam])-> [[f64; tam]; tam]{
    let mut l = [[0f64; tam]; tam];
    for i in 0..tam {
        for j in i+1..tam {
            l[i][j]=slice[i][j];
        }
    }
    //println!("---------Descomposicion L------------------" );
    //imprimir_matriz(&l);
    return l;
}
//Descompone la matriz principal en la diagonal de la matriz
fn descomposicion_D(slice: [[f64;tam]; tam])-> [[f64; tam]; tam]{
    let mut d = [[0f64; tam]; tam];
    for i in 0..tam {
        d[i][i]=slice[i][i];
        //println!("{}", d[i][i]); // x: i32
    }
    //println!("---------Descomposicion D------------------" );
    //imprimir_matriz(&d);
    return d;
}
//Esta funcion suma dos matrices del mismo tamaño
fn suma_de_matriz(A: [[f64;tam]; tam],B: [[f64;tam]; tam])-> [[f64; tam]; tam]{
    let mut resultado = [[0f64; tam]; tam];
    for i in 0..tam {
        for j in 0..tam {
            resultado[i][j]=A[i][j]+B[i][j];
        }
    }
    //imprimir_matriz(&resultado);
    return resultado;
}
//Esta funcion suma dos matrices del mismo tamaño
fn suma_de_matriz2(A: [f64;tam],B: [f64;tam])-> [f64; tam]{
    let mut resultado = [0f64; tam];
    for i in 0..tam {
        resultado[i]=-A[i]+B[i];
    }

    return resultado;
}

//Esta funcion resta dos matrices del mismo tamaño
fn resta_de_matriz(A: [f64;tam],B: [f64;tam])-> [f64; tam]{
    let mut resultado = [0f64; tam];
    for i in 0..tam {
            resultado[i]=A[i]-B[i];
    }
    return resultado;
}
//Esta funcion multiplica dos matrices una matriz normal y otra con una columna
fn multiplicacion_matriz(A: [[f64;tam]; tam] ,xk:[f64;tam])-> [f64; tam]{
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
//Imprime el contenido de una matriz
fn imprimir_matriz(slice: &[[f64;tam]; tam]){
    for i in 0..tam {
        for j in 0..tam {
            print!("{}  ", slice[i][j]);
        }
        println!("");
    }
}

fn imprimir_matriz_simple(slice: &[f64;tam]){
    for i in 0..tam {
            print!("{}  ", slice[i]);

    }
    println!("");
}
//Esta funcion calcula la norma de la matriz simple ingresada
fn norma_simple(A:[f64;tam])-> f64{
    let mut numeros:f64=0.0;
    let mut norma:f64=0.0;
    for i in 0..tam {s,
        numeros=(A[i].powf((2 as f64))+numeros as f64);
    }
    norma= (numeros.powf((0.5 as f64)) as f64);
    return norma;
}
