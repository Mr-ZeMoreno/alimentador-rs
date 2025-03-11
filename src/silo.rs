use uuid::Uuid;

pub struct Silo {
    // Maneja el alimento en kilogramos
    // Máximo de kilogramos -> 65,535
    alimento: u32,

    // Alimento historico en kilogramos
    // Máximo de kilogramos -> 4,294,967,295 
    // En una temporada trabajamos alrededor de 864,000 kilos
    // Esto porque se llenan 3 veces por mes en un año 3 * 12 * 24,000
    historico: u32,

    // Guardamos la capacidad máxima que puede contener un silo
    // Máximo de kilogramos -> 65,535
    // Nuestros silo son de 24 toneladas
    capacidad:u32,

    id:Uuid
}

impl Silo {
    pub fn new()-> Self {
        Self {
            alimento:0,
            historico:0,
            capacidad:0,
            id: Uuid::new_v4()
        }
    }
    fn set_historico(&mut self,n:u32){
        self.historico += u32::from(n);
    }

    fn verificar_llenado(&mut self, n:u32)->bool{
        if n + self.alimento > self.capacidad {
            return false;
        }
        return true;
    }

    pub fn set_alimento(&mut self,n: u32)->&mut Silo{
        if self.alimento<n {
            self.set_historico(n - self.alimento);
        }
        self.alimento = n;
        self
    }

    pub fn get_alimento(&self)->u32{
        self.alimento
    }
    pub fn get_historico(&self)->u32{
        self.historico
    }

    pub fn entregar_pulso(&mut self, pulso:u32)->&mut Silo {
        self.set_alimento(self.get_alimento() - pulso)
    }

    pub fn print_silo(&self, pulso:u32)->&Silo{
        println!("\nSilo: {} - Historico: {}", self.alimento, self.historico);
        println!("Entregando {} kg\n", pulso);
        self
    }
}