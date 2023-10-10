

// ==========================================================================
//                        Traits
//===========================================================================
trait FuncionesGeometricas {
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
struct Calculadora {}

// ==========================================================================
//                          Enums
//===========================================================================

enum FiguraGeometrica {
    Cuadrado( Cuadrado ),
    Triangulo ( Triangulo ),
    Circulo ( Circulo )
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


// ==========================================================================
//                       Main
//===========================================================================

fn main() {
    println!("Hello 🌍!");
}
