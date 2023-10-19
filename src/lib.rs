pub mod calculadora;
pub mod figuras;

#[cfg(test)]
mod tests {
    use crate::calculadora::Calculadora;
    use crate::figuras::circulo::Circulo;
    use crate::figuras::cuadrado::Cuadrado;
    use crate::figuras::triangulo::Triangulo;
    #[test]
    fn calculo_de_areas() {
        let mut prueba: Calculadora = Calculadora::new();

        prueba.nuevo_elemento(Circulo::new(1.0));

        prueba.nuevo_elemento(Cuadrado::new(5.0));

        prueba.nuevo_elemento(Triangulo::new(23.0, 32.0, 56.0, 78.0));

        assert_eq!(prueba.obtener_area(0).unwrap(), std::f64::consts::PI);
        assert_eq!(prueba.obtener_area(1).unwrap(), 25.0);
        assert_eq!(prueba.obtener_area(2).unwrap(), 897.0);
    }
}
