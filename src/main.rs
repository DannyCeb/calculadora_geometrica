
use core::fmt::Display;
use std::fmt;
// ==========================================================================
//                        Traits
//===========================================================================
trait FuncionesGeometricas: Display{
    fn calcular_area(&self) -> f64;
    fn calcular_perimetro(&self) -> f64;
}
// ==========================================================================


// ==========================================================================
//                        Structs
//===========================================================================

struct Cuadrado {
    lado: f64,
}


struct Triangulo {
    base: f64,
    lado1: f64,
    lado2: f64,
    altura: f64
}


struct Circulo {
    radio: f64
}
struct Calculadora {
    figuras: Vec<Box<dyn FuncionesGeometricas>>
}

// ==========================================================================
//                        Implementaciones
//===========================================================================


impl Cuadrado {
    fn new(lado: f64) -> Cuadrado { Cuadrado { lado } }
}

impl FuncionesGeometricas for Cuadrado{
    fn calcular_area(&self) -> f64{
        self.lado.powi(2)
    }

    fn calcular_perimetro(&self) -> f64 {
        self.lado * 4.0
    }
}

impl Display for Cuadrado {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cuadrado: (lado: {})", self.lado)
    }
}


impl Triangulo {
    fn new(base: f64, lado1: f64, lado2: f64, altura: f64) -> Triangulo{
        Triangulo { base, lado1, lado2, altura}
    }
}

impl FuncionesGeometricas for Triangulo {
    fn calcular_area(&self) -> f64 {
        (self.base * self.altura) / 2.0
    }

    fn calcular_perimetro(&self) -> f64 {
        self.lado1 + self.lado2 + self.base
    }

}

impl Display for Triangulo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Triangulo: (Base: {}, Altura: {}, Lado1: {}, Lado2: {})", self.base, self.altura, self.lado1, self.lado2)
    }
}

impl Circulo {
    fn new(radio: f64) -> Circulo{ Circulo { radio } }
}

impl FuncionesGeometricas for Circulo {
    
    fn calcular_area(&self) -> f64 {
        self.radio.powi(2) * std::f64::consts::PI
    }

    fn calcular_perimetro(&self) -> f64 {
        self.radio * 2.0 * std::f64::consts::PI
    }

}

impl Display for Circulo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circulo: (Radio: {})", self.radio)
    }
}

impl Calculadora {
    fn new() -> Calculadora{
        Calculadora { figuras: vec![] }
    }
}

impl Calculadora {
    fn mostrar_figuras(&self) {
        for (i, v) in self.figuras.iter().enumerate() {
            println!("Figura {}: {}",i,v );
        }
    }

    fn obtener_area(&self, indice: usize) -> Option<f64>{
        match self.figuras.get(indice) {
            Some(figura) => Some(figura.calcular_area()) ,
            None => None
        }
    }
}


// ==========================================================================
//                       Main
//===========================================================================

fn main() {
    println!("Hello 🌍!");

    let mut prueba: Calculadora = Calculadora::new();

    prueba.figuras.push( Box::new( Circulo::new( 23.1 ) ) );

    prueba.figuras.push( Box::new( Cuadrado::new(24.5) ) );

    prueba.mostrar_figuras();

    println!("{:?}", prueba.obtener_area(1));
    println!("{:?}", prueba.obtener_area(2));


}
