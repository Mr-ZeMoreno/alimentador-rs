use uuid::Uuid;

pub struct Dosificador {
    // Capacidad de entrega en kg/s
    // Aproximar al entero mas cercano
    entrega:u32,

    estado: bool,

    id:Uuid
}

impl Dosificador {
    pub fn new() -> Self {
        Self {
            entrega: 0,
            estado: false,
            id: Uuid::new_v4()
        }
    }

    pub fn set_entrega(&mut self, n:u32)->&mut Dosificador {
        self.entrega = n;
        self
    }
    pub fn set_estado(&mut self, n:bool)->&mut Dosificador {
        self.estado = n;
        self
    }

    pub fn get_entrega(&self)->u32 {
        self.entrega
    }

    pub fn get_estado(&self)->bool {
        self.estado
    }

    pub fn get_id(&self)->Uuid {
        self.id
    }
}