use crate::figuras::traits::FuncionesGeometricas;
use core::fmt::Display;
use std::fmt;
pub struct Cuadrado {
    lado: f64,
}

impl Cuadrado {
    pub fn new(lado: f64) -> Cuadrado {
        Cuadrado { lado }
    }
}

impl FuncionesGeometricas for Cuadrado {
    fn calcular_area(&self) -> f64 {
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
