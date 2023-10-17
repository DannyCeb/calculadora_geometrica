use crate::figuras::traits::FuncionesGeometricas;
use core::fmt::Display;
use std::fmt;

pub struct Circulo {
    radio: f64,
}

impl Circulo {
    pub fn new(radio: f64) -> Circulo {
        Circulo { radio }
    }
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
