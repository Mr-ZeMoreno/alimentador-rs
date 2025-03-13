use crate::{ciclo::Ciclo, racion::Racion};

/// El trait `Print` permite que un tipo implemente el método `print`
/// para mostrar su estado de manera legible.
///
/// # Métodos:
/// - `print`: Imprime la información relevante del tipo en la consola.
pub trait Print {
    fn print(&self) -> &Self;
}

impl Print for Ciclo {
    /// Imprime la información detallada de una ración.
    ///
    /// Muestra el `id`, los `pulsos`, la duración del pulso en milisegundos (`DP`),
    /// y el tiempo de espera entre pulsos (`EP`).
    ///
    /// # Ejemplo:
    /// ```rust
    /// let ciclo = Ciclo::new();
    /// ciclo.print();
    /// ```
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

impl<'a> Print for Racion<'a> {
    /// Imprime los detalles de la racion, incluyendo su `id` y los `id`s de los ciclos asociados.
    ///
    /// # Ejemplo:
    /// ```rust
    /// let racion = Racion::new(vec![&racion1, &racion2]);
    /// racion.print();
    /// ```
    fn print(&self) -> &Racion<'a> {
        println!(
            "\n[Racion][{}]: Ciclos <[{}]>",
            self.get_id(),
            self.get_ciclos()
                .iter()
                .map(|racion| racion.get_id().to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );
        self
    }
}
