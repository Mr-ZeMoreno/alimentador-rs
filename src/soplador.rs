use uuid::Uuid;
use crate::sleep;

pub struct Soplador {
    estado: bool,
    
    // 0 - 100 
    potencia: u8,
    
    id: Uuid
}

impl Soplador {
    pub fn new() -> Self {
        Self {
            estado:false,
            potencia: 0,
            id: Uuid::new_v4()
        }
    }
}


// Getter y Setter
impl Soplador {
    pub fn set_estado(&mut self, n:bool)->&mut Soplador {
        if n {
            println!("[Silo][{}]: Encendiendo... Duración 5s", self.id);
            sleep(5000);
        }
        self.estado = n;
        self
    }

    pub fn set_potencia(&mut self, n:u8)->&mut Soplador {
        self.potencia = n;
        self
    }

    pub fn get_estado(&self)->bool{
        self.estado
    }

    pub fn get_potencia(&self)->u8{
        self.potencia
    }

    pub fn get_id(&self)->Uuid{
        self.id
    }
}