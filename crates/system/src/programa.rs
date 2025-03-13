use hardware::{dosificador::Dosificador, silo::Silo, soplador::Soplador};

use crate::racion::Racion;
use crate::traits::Print as SystemPrint;

use hardware::traits::Print as HardwarePrint;
use hardware::traits::PrintConArg;

use utils::utils::sleep;

pub struct Programa<'a> {
    racion: &'a Racion<'a>, // Ahora `Programa` solo tiene una referencia a `Racion`
}

impl<'a> Programa<'a> {
    pub fn new(racion: &'a Racion<'a>) -> Self {
        Self { racion }
    }

    pub fn iniciar(&mut self, soplador: &mut Soplador, doser: &mut Dosificador, silo: &mut Silo) {
        for (i, ciclo) in self.racion.get_ciclos().iter().enumerate() {
            soplador.set_estado(true);

            let id = ciclo.get_id();
            println!("[Ración: {}][Tipo: {}]: En Ejecución", i + 1, id);

            let [pulsos, pulso_duracion, pulso_espera]: [u32; 3] = ciclo.get_all();

            if pulso_duracion == 0 || pulsos == 0 || pulso_espera == 0 {
                break;
            }

            println!("Iniciando...");
            ciclo.print();

            for _pulso in 1..=pulsos {
                let entregado = doser.get_entrega() * (pulso_duracion / 1000);

                doser.set_estado(true).print();
                silo.entregar_pulso(entregado).print(entregado);

                sleep(u64::from(pulso_duracion));

                doser.set_estado(false).print();
                sleep(u64::from(pulso_espera));
            }
            println!(
                "Ración {}: En Espera... Duración {}s",
                id,
                self.racion.get_ciclo_espera()
            );
            sleep(u64::from(self.racion.get_ciclo_espera()));
        }
    }
}
