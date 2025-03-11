// use crate::Silo;
use crate::Racion;
use crate::Programa;
use crate::Soplador;
use crate::Dosificador;


pub trait Debuggeable {
    fn print(&self)->&Self;
}

impl Debuggeable for Soplador {
    fn print(&self)->&Soplador {
        let estado:&'static str = {
            if self.get_estado() {"Encendido"} else {"Apagado"}
        };
        println!("[Soplador][{}],[{}]: -- [{}] --", self.get_id(),self.get_potencia(), estado);
        self
    }
}

impl Debuggeable for Dosificador {
    fn print(&self)->&Dosificador {
        let estado:&'static str = {
            if self.get_estado() {"Encendido"} else {"Apagado"}
        };
        println!("\nDosificador [{}]: {}\n",self.get_id(), estado);
        self
    }
}

impl Debuggeable for Racion {
    fn print(&self)->&Racion {
        println!("\n[Ración][{}][{} P][{}ms DP][{}ms EP]\n",
        self.get_id(), 
        self.get_pulsos(),
        self.get_pulso_duracion(), 
        self.get_pulso_espera());
        self
    }
}