use hardware::{dosificador::Dosificador, silo::Silo, soplador::Soplador};

use crate::logs::Print as SystemPrint;
use crate::racion::Racion;

use hardware::logs::Print as HardwarePrint;

use utils::utils::sleep;

pub struct Programa<'a> {
    racion: &'a Racion<'a>,
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

            println!("Iniciando...");
            ciclo.print();

            let pulsos = ciclo.get_pulsos();
            let pulso_duracion = ciclo.get_pulso_duracion();
            let pulso_espera = ciclo.get_pulso_espera();

            for _pulso in 0..=pulsos {
                let entregado = doser.get_entrega() * (pulso_duracion / 1000);

                doser.set_estado(true).print();
                silo.entregar_pulso(entregado)
                    .expect("Has intentado entregar un pulso mayor a la cantidad de alimento");
                silo.print();

                sleep(pulso_duracion);

                doser.set_estado(false).print();
                sleep(pulso_espera);
            }
            println!(
                "Ración {}: En Espera... Duración {}s",
                id,
                self.racion.get_ciclo_espera()
            );
            sleep(self.racion.get_ciclo_espera());
        }
    }
}
