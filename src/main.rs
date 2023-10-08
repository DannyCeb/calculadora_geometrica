

// ==========================================================================
//                        Traits
//===========================================================================
trait FuncionesGeometricas {
    fn calcular_area(&self);
    fn calcular_perimetro(&self);
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

impl Triangulo {
    fn new(base: f64, lado1: f64, lado2: f64, altura: f64) -> Triangulo{
        Triangulo { base, lado1, lado2, altura}
    }
}

impl Circulo {
    fn new(radio: f64) -> Circulo{ Circulo { radio } }
}


// ==========================================================================
//                       Main
//===========================================================================

fn main() {
    println!("Hello, world!");
}
