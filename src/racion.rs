use uuid::Uuid;

pub struct Racion {
    // Duración de pulsos en ms, 
    // No deberían durar mas de un minuto en producción
    pulso_duracion: u32,
    // pulsos por ración 
    pulsos:u32,
    // tiempo entre pulsos en ms
    pulso_espera:u32,

    //id
    id: Uuid
}

impl Racion {
    pub fn new()->Self {
        Self {
            pulso_duracion: 0,
            pulsos: 0,
            pulso_espera:0,
            id:Uuid::new_v4()
        }
    }

    pub fn set_pulso_duracion(&mut self, n:u32)->&mut Self{
        self.pulso_duracion = n;
        self
    }

    pub fn set_pulsos(&mut self, n:u32)->&mut Self{
        self.pulsos = n;
        self
    }

    pub fn set_pulso_espera(&mut self, n:u32)->&mut Self{
        self.pulso_espera = n;
        self
    }

    pub fn get_pulsos(&self)->u32{
        self.pulsos
    }

    pub fn get_pulso_espera(&self)->u32 {
        self.pulso_espera
    }

    pub fn get_pulso_duracion(&self)->u32{
        self.pulso_duracion
    }

    pub fn get_all(&self)->[u32;3]{
        [self.pulsos, self.pulso_duracion, self.pulso_espera]
    }

    pub fn get_id(&self)->Uuid {
        self.id
    }
}