use uuid::Uuid;
use crate::racion::Racion;

pub struct Programa<'a> {
    raciones: Vec<&'a Racion>, 
    racion_espera: u32,
    id: Uuid,
}

impl<'a> Programa<'a> {
    pub fn new(raciones: Vec<&'a Racion>) -> Self { 
        Self {
            raciones,
            racion_espera: 0,
            id: Uuid::new_v4(),
        }
    }

    pub fn get_raciones(&self) -> &Vec<&'a Racion>{
        &self.raciones
    } 

    pub fn set_racion_espera(&mut self, n: u32) -> &mut Self {
        self.racion_espera = n;
        self
    }

    pub fn get_racion_espera(&self) -> u32 {
        self.racion_espera
    }

    pub fn get_racion_by_id(&mut self, id: Uuid) -> Option<&Racion> { 
        self.raciones
            .iter_mut()
            .find(|r| r.get_id() == id)
            .map(|r| &**r)
    }        
}
