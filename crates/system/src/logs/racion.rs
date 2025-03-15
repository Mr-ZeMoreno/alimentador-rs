use crate::logs::Print;
use crate::racion::Racion;

impl<'a> Print for Racion<'a> {
    /// Imprime los detalles de la racion, incluyendo su `id` y los `id`s de los ciclos asociados.
    ///
    /// # Ejemplo:
    ///! ```rust
    ///! let racion = Racion::new(vec![&racion1, &racion2]);
    ///! racion.print();
    ///! ```
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
