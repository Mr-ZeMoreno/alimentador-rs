use uuid::Uuid;

pub struct Dosificador {
    // Capacidad de entrega en kg/s
    // Aproximar al entero mas cercano
    // No es una caracteristica que deba modificarse
    entrega: u32,

    estado: bool,

    id: Uuid,
}

impl Dosificador {
    pub fn new(entrega: u32) -> Self {
        Self {
            entrega,
            estado: false,
            id: Uuid::new_v4(),
        }
    }
    pub fn set_estado(&mut self, n: bool) -> &mut Dosificador {
        self.estado = n;
        self
    }

    pub fn get_entrega(&self) -> u32 {
        self.entrega
    }

    pub fn get_estado(&self) -> bool {
        self.estado
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }
}
