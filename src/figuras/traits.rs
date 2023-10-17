use core::fmt::Display;
pub trait FuncionesGeometricas: Display {
    fn calcular_area(&self) -> f64;
    fn calcular_perimetro(&self) -> f64;
}
