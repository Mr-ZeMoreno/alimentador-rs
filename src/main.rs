use hardware::dosificador::Dosificador;
use hardware::silo::Silo;
use hardware::soplador::Soplador;
use system::ciclo::Ciclo;
use system::programa::Programa;
use system::racion::Racion;

fn main() {
    let mut ciclo1 = Ciclo::new();
    ciclo1
        .set_pulsos(50)
        .set_pulso_duracion(5000)
        .set_pulso_espera(8000);

    let mut ciclo2 = Ciclo::new();
    ciclo2
        .set_pulsos(20)
        .set_pulso_duracion(3000)
        .set_pulso_espera(4000);

    let mut racion = Racion::new(vec![&ciclo1, &ciclo2, &ciclo1, &ciclo1]);

    let mut doser: Dosificador = Dosificador::new();
    let mut soplador: Soplador = Soplador::new();

    let mut silo: Silo = Silo::new(24000);

    silo.set_alimento(24000);

    doser.set_entrega(2).set_estado(false);

    racion.set_ciclo_espera(60000);

    let mut programa: Programa = Programa::new(&mut racion);

    programa.iniciar(&mut soplador, &mut doser, &mut silo);
}
