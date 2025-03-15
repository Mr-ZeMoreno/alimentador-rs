use crate::ciclo::Ciclo;
use crate::logs::Print;

impl Print for Ciclo {
    /// Imprime la información detallada de una ración.
    ///
    /// Muestra el `id`, los `pulsos`, la duración del pulso en milisegundos (`DP`),
    /// y el tiempo de espera entre pulsos (`EP`).
    ///
    /// # Ejemplo:
    ///! ```rust
    ///! let ciclo = Ciclo::new();
    ///! ciclo.print();
    ///! ```
    fn print(&self) -> &Ciclo {
        println!(
            "\n[Ración][{}][{} P][{}ms DP][{}ms EP]\n",
            self.get_id(),
            self.get_pulsos(),
            self.get_pulso_duracion(),
            self.get_pulso_espera()
        );
        self
    }
}
