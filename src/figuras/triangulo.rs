use crate::figuras::traits::FuncionesGeometricas;
use core::fmt::Display;
use std::fmt;

pub struct Triangulo {
    base: f64,
    lado1: f64,
    lado2: f64,
    altura: f64,
}

impl Triangulo {
    pub fn new(base: f64, lado1: f64, lado2: f64, altura: f64) -> Triangulo {
        Triangulo {
            base,
            lado1,
            lado2,
            altura,
        }
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
        write!(
            f,
            "Triangulo: (Base: {}, Altura: {}, Lado1: {}, Lado2: {})",
            self.base, self.altura, self.lado1, self.lado2
        )
    }
}

impl From<(f64, f64, f64, f64)> for Triangulo {
    fn from(v: (f64, f64, f64, f64)) -> Triangulo {
        Triangulo {
            base: v.0,
            lado1: v.1,
            lado2: v.2,
            altura: v.3,
        }
    }
}
