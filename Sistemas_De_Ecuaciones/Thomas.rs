const tam: usize = 3;
// La funcion main controla todo lo que se va a ejecutar , crea los parametros de entrada y
//los envia a la funcion Thomas
fn main() {
    let mut a : [f32; tam]= [0.0000,-1.0000,-1.0000];
    let mut b : [f32; tam]= [2.0000,2.0000,2.0000];
    let mut c : [f32; tam]= [-1.0000,-1.0000,0.0000];
    let mut d : [f32; tam]= [124.0000,4.0000,14.0000];
    let mut x = Thomas(a,b,c,d);
    for i in 0..tam {
        print!("{}  ", x[i]);
    }
}
//La funcion Thomas se encarga de recibir los parametros de entrada y aplocando las diferentes formulas Retorna
//como resultado la solucion a ese sistema de matrices
fn Thomas(a:[f32;tam],b:[f32;tam],c:[f32;tam],d:[f32;tam])-> [f32; tam]{
    let mut p = [0f32; tam];
    let mut q = [0f32; tam];
    let mut x = [0f32; tam];
    //Caso inicial
    p[0]=c[0]/b[0];
    q[0]=d[0]/b[0];
    for i in 1..tam {
        //Caso Pi n-1
        if i < tam-1 {
            p[i]=c[i]/(b[i]-(p[i-1]*a[i]));
        }
        q[i]=(d[i]-q[i-1]*a[i])/(b[i]-p[i-1]*a[i]);
    }
    x[tam-1]=q[tam-1];
    let mut j=tam-2;
    for j in 0..(tam-1) {
        x[tam-2-j]=q[tam-2-j]-p[tam-2-j]*x[tam-2-j+1];
    }
    return x;
}
