use types::rango::{Rango, RangoData};
use uuid::Uuid;

pub enum TAGS {
    NAME,
}

impl TAGS {
    pub fn as_str(&self) -> &'static str {
        match self {
            TAGS::NAME => "Selectora",
        }
    }
}

pub struct Selectora {
    posicion: Rango,
    id: Uuid,
    espera: u32,
}

impl Selectora {
    /// El constructor recibe el argumento la posición máxima que puede tener una selectora, las posiciones comienzan del 0
    /// Ejemplo una selectora con 6 salidas su posición máxima seria 5 -> [0,1,2,3,4,5]
    ///! ```rust
    ///! let selectora = Selectora::new(5);
    ///! ```
    pub fn new(posicion_maxima: u32) -> Self {
        Self {
            posicion: Rango::new(0, posicion_maxima, 0).unwrap(),
            id: Uuid::new_v4(),
            espera: 20000,
        }
    }

    pub fn set_posicion(&mut self, n: u32) -> Result<(), crate::errors::SelectoraError> {
        match self.posicion.set(n, &TAGS::NAME.as_str()) {
            Ok(()) => Ok(()),
            Err(types::rango::RangoError::FueraDeRango) => {
                Err(crate::errors::SelectoraError::FueraDeRango)
            }
        }
    }

    pub fn set_espera(&mut self, n: u32) {
        self.espera = n;
    }

    pub fn get_posicion(&self) -> u32 {
        self.posicion.get()
    }

    pub fn get_posiciones(&self) -> RangoData {
        self.posicion.get_rango()
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }
}
