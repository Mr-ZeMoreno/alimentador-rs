use crate::log::Print;
use crate::soplador::Soplador;
impl Print for Soplador {
    /// Imprime el estado del soplador.
    ///
    /// Muestra el `id`, la `potencia`, y si está `encendido` o `apagado`.
    ///
    /// # Ejemplo:
    ///! ```rust
    ///! let soplador = Soplador::new();
    ///! soplador.print();
    ///! ```
    fn print(&self) -> String {
        let estado = if self.get_estado() {
            "Encendido"
        } else {
            "Apagado"
        };

        let texto = format!(
            "[Soplador][{}][{}%]: -- [{}] --",
            self.get_id(),
            self.get_potencia(),
            estado
        );

        println!("{}", &texto);

        texto
    }
}
