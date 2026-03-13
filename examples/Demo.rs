use documentacion_rust::{es_par, suma};

fn main() {
    let a = 10;
    let b = 5;
    let total = suma(a, b);
    println!("suma de {} y {} es {}", a, b, total);

    let n = 7;
    println!("{} es par {}", n, es_par(n));
}
