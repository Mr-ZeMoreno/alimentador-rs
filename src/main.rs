use crate::traits::debug::Debuggeable;

mod traits;

mod utils;

use utils::{sleep, Rango};

mod racion;
use racion::Racion;

mod silo;
use silo::Silo;

mod programa;
use programa::Programa;

mod dosificador;
use dosificador::Dosificador;

mod soplador;
use soplador::Soplador;

fn main() {
    let mut racion1 = Racion::new();
    racion1
        .set_pulsos(50)
        .set_pulso_duracion(5000)
        .set_pulso_espera(8000);

    let mut racion2 = Racion::new();
    racion2
        .set_pulsos(20)
        .set_pulso_duracion(3000)
        .set_pulso_espera(4000);

    let mut programa = Programa::new(vec![&racion1, &racion2, &racion1, &racion1]);

    let mut doser: Dosificador = Dosificador::new();
    let mut soplador: Soplador = Soplador::new(); /////////////////////////////////////

    let mut silo: Silo = Silo::new();

    silo.set_alimento(24000);

    doser.set_entrega(2).set_estado(false);

    programa.set_racion_espera(60000);

    for (i, racion) in programa.get_raciones().iter().enumerate() {
        soplador.set_estado(true);

        let id = racion.get_id();
        println!("[Ración: {}][Tipo: {}]: En Ejecución", i + 1, id);

        let [pulsos, pulso_duracion, pulso_espera]: [u32; 3] = racion.get_all();

        if pulso_duracion == 0 || pulsos == 0 || pulso_espera == 0 {
            break;
        }

        println!("Iniciando...");
        racion.print();

        for _pulso in 1..pulsos + 1 {
            let entregado = doser.get_entrega() * (pulso_duracion / 1000);

            doser.set_estado(true).print();
            silo.entregar_pulso(entregado).print_silo(entregado);

            sleep(u64::from(pulso_duracion));

            doser.set_estado(false).print();
            sleep(u64::from(pulso_espera));
        }
        println!(
            "Ración {}: En Espera... Duración {}s",
            id,
            programa.get_racion_espera()
        );
        sleep(u64::from(programa.get_racion_espera()));
    }
}
