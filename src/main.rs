// ==========================================================================
//                       Main
//===========================================================================

use calculadora_geometrica::calculadora::Calculadora;
use calculadora_geometrica::figuras::circulo::Circulo;
use calculadora_geometrica::figuras::cuadrado::Cuadrado;
use calculadora_geometrica::figuras::triangulo::Triangulo;

fn main() {
    println!("Hello 🌍!");

    let mut prueba: Calculadora = Calculadora::new();

    prueba.nuevo_elemento(Circulo::new(23.1));

    prueba.nuevo_elemento(Cuadrado::new(24.5));

    prueba.nuevo_elemento(Triangulo::new(23.0, 32.0, 56.0, 78.0));

    prueba.nuevo_elemento(Triangulo::from((23.0, 32.0, 56.0, 78.0)));

    prueba.mostrar_figuras();

    println!("{:?}", prueba.obtener_area(0));
    println!("{:?}", prueba.obtener_area(1));
    println!("{:?}", prueba.obtener_area(2));
    println!("{:?}", prueba.obtener_area(3));
}
