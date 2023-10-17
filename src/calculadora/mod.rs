use crate::figuras::traits::FuncionesGeometricas;
pub struct Calculadora {
    figuras: Vec<Box<dyn FuncionesGeometricas>>,
}

impl Calculadora {
    pub fn new() -> Calculadora {
        Calculadora { figuras: vec![] }
    }
}

impl Calculadora {
    pub fn mostrar_figuras(&self) {
        for (i, v) in self.figuras.iter().enumerate() {
            println!("Figura {}: {}", i, v);
        }
    }

    pub fn obtener_area(&self, indice: usize) -> Option<f64> {
        match self.figuras.get(indice) {
            Some(figura) => Some(figura.calcular_area()),
            None => None,
        }
    }
    pub fn nuevo_elemento<T: FuncionesGeometricas + 'static>(&mut self, e: T) {
        self.figuras.push(Box::new(e));
    }
}
