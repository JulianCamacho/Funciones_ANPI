const tam: usize = 3;
use std::num;
// La funcion main controla todo lo que se va a ejecutar , crea los parametros de entrada y
//los envia a la funcion Pseudoinversa
fn main() {
    let mut A=[[2.0,-1.0,0.0],[-1.0,2.0,-1.0],[0.0,-1.0,2.0]];
    let mut b=[124.0,4.0,14.0];
    Pseudoinversa(A,b,0.00000001,10);

}

//La funcion Pseudoinversa recibe los parametros de entrada y lso procesa para resolver el sistema
fn Pseudoinversa(A: [[f64;tam]; tam] ,b:[f64;tam],tol:f64,interMax:i32)-> [[f64;tam]; tam]{
    let mut x0 = multiplicarA_numero(&Transpuesta(&A),1.0/norma(A));
    let mut xk = [[0f64; tam]; tam];
    let mut xk1 = [[0f64; tam]; tam];
    let mut ver_xk = [0f64; tam];
    let mut ver_xk1 = [0f64; tam];

    let mut error_p:f64=100.0;
    let mut I = identidad();
    let mut s2=[[0f64; tam]; tam];
    let mut xk=[[0f64; tam]; tam];
    for i in 0..interMax {
        s2= resta_de_matriz(multiplicarA_numero(&I,2.0),multiplicacion_matriz_Iguales(A,x0));
        xk= multiplicacion_matriz_Iguales(x0,s2);
        ver_xk=multiplicacion_matriz(xk,b);
        let mut s3= resta_de_matriz(multiplicarA_numero(&I,2.0),multiplicacion_matriz_Iguales(A,xk));
        let mut xk1= multiplicacion_matriz_Iguales(xk,s3);
        ver_xk1=multiplicacion_matriz(xk1,b);

        error_p= (norma_simple(resta_de_matriz_simple(ver_xk,ver_xk1)))/(norma_simple(ver_xk1));
        if error_p < tol {
            break;
       }
        x0=xk;
    }
    println!("------------- Solucion  Xk --------- ");
    imprimir_matriz(&xk);
    println!("Error ---------------------- {}",error_p );
    return xk;
}

//Con esta funcion se obtiene la norma 2 de una matriz , y lo retorna elevado a la 2
fn norma(A:[[f64;tam]; tam])-> f64{
    let mut numeros:f64=0.0;
    let mut norma:f64=0.0;
    for i in 0..tam {
        for j in 0..tam {
            numeros=(A[i][j].powf((2 as f64))+numeros as f64);
        }
    }
    norma= (numeros.powf((0.5 as f64)) as f64);
    return norma*norma;
}

////Con esta funcion se obtiene la norma 2 de una lista
fn norma_simple(A:[f64;tam])-> f64{
    let mut numeros:f64=0.0;
    let mut norma:f64=0.0;
    for i in 0..tam {
        numeros=(A[i].powf((2 as f64))+numeros as f64);
    }
    norma= (numeros.powf((0.5 as f64)) as f64);
    return norma;
}
//Crea una matriz identidad del tamaño tam
fn identidad()->[[f64;tam]; tam]{
    let mut resultado = [[0f64; tam]; tam];
    for i in 0..tam {
        resultado[i][i]=1.0;
    }
    //imprimir_matriz(&resultado);
    return resultado;
}
//Estas funciones se encargan de imprimir la matriz ingresada segun sus parametros
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
//Esta funcion multriplica una matriz con un numero escalar
fn multiplicarA_numero(slice: &[[f64;tam]; tam], b:f64)->[[f64;tam]; tam]{
    let mut resultado = [[0f64; tam]; tam];
    for i in 0..tam {
        for j in 0..tam {
            resultado[i][j]=slice[i][j]*b;
        }
    }
    //imprimir_matriz(&resultado);
    return resultado;
}
//Retorna la matriz transpuesta de la matriz ingresada
fn Transpuesta(slice: &[[f64;tam]; tam])->[[f64;tam]; tam]{
    let mut resultado = [[0f64; tam]; tam];
    for i in 0..tam {
        for j in 0..tam {
            resultado[j][i]=slice[i][j];
        }
    }
    //imprimir_matriz(&resultado);
    return resultado;
}
//Las siguientes funciones multiplican dos matrices , en la primera son dos matrices iguales
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
//Y en esta se multiplica una matriz con una lista
fn multiplicacion_matriz_Iguales(A: [[f64;tam]; tam] ,xk:[[f64;tam]; tam])-> [[f64;tam]; tam]{
    let mut resultado = [[0f64; tam]; tam];
    let mut aux=0.0;
    let mut aux1=0.0;
    for a in 0..tam {
        for i in 0..tam {
            for j in 0..tam {
                aux=A[i][j]*xk[j][a]+aux1;
                //println!("{} * {} + {}  ", A[i][j],xk[j][a],aux1);
                aux1=aux;
            }
            //println!("--------------------------------------");
            resultado[i][a]=aux1;
            aux1=0.0;
        }

    }
    //imprimir_matriz(&resultado);
    return resultado;
}

//Las siguientes funciones restan matrices deacuerdo a sus parametros 
fn resta_de_matriz(A: [[f64;tam]; tam],B: [[f64;tam]; tam])-> [[f64;tam]; tam]{
        let mut resultado = [[0f64; tam];tam];
        for i in 0..tam {
            for j in 0..tam {
                resultado[i][j]=A[i][j]-B[i][j];
            }
        }
        return resultado;
}
fn resta_de_matriz_simple(A: [f64;tam],B: [f64;tam])-> [f64;tam]{
        let mut resultado = [0f64; tam];
        for i in 0..tam {
                resultado[i]=A[i]-B[i];
        }
        return resultado;
}
