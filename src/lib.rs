// Biblioteca base para ejemplos de Rust

pub fn suma(a: i32, b: i32) -> i32 {
    a + b
}

pub fn es_par(n: i32) -> bool {
    n % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prueba_suma() {
        assert_eq!(suma(2, 3), 5);
    }

    #[test]
    fn prueba_par() {
        assert!(es_par(4));
        assert!(!es_par(5));
    }
}
