use hardware::dosificador::Dosificador;
use hardware::silo::Silo;
use hardware::soplador::Soplador;
use system::ciclo::Ciclo;
use system::programa::Programa;
use system::racion::Racion;

fn main() {
    let mut c1 = Ciclo::new();
    c1.set_pulsos(50)
        .expect("Ha intentado establer una cantidad de pulsos fuera del rango permitido");
    c1.set_pulso_duracion(5000)
        .expect("Ha intentado establecer una duracion de pulso fuera del rango permitido");
    c1.set_pulso_espera(8000)
        .expect("Ha intentado establecer una espera entre pulsos fuera del rango permitido");

    let mut c2 = Ciclo::new();
    c2.set_pulsos(20)
        .expect("Ha intentado establer una cantidad de pulsos fuera del rango permitido");
    c2.set_pulso_duracion(3000)
        .expect("Ha intentado establecer una duracion de pulso fuera del rango permitido");
    c2.set_pulso_espera(4000)
        .expect("Ha intentado establecer una espera entre pulsos fuera del rango permitido");

    let mut racion = Racion::new(vec![&c1, &c2, &c1, &c1]);

    let mut doser: Dosificador = Dosificador::new(2);
    let mut soplador: Soplador = Soplador::new();

    let mut silo: Silo = Silo::new(24000);

    silo.set_alimento(24000)
        .expect("Ha intentado ingresar mas alimento que la capacidad del silo");

    racion
        .set_ciclo_espera(60000)
        .expect("Ha intentado establecer una espera entre ciclos fuera del rango permitido");

    let mut programa: Programa = Programa::new(&mut racion);

    programa.iniciar(&mut soplador, &mut doser, &mut silo);
}
