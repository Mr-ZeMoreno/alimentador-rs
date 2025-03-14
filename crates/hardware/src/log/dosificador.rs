use crate::dosificador::Dosificador;
use crate::log::Print;

impl Print for Dosificador {
    /// Imprime el estado del dosificador.
    ///
    /// Muestra el `id` y si el dosificador está `encendido` o `apagado`.
    ///
    /// # Ejemplo:
    ///! ```rust
    ///! let dosificador = Dosificador::new();
    ///! dosificador.print();
    ///! ```
    fn print(&self) -> String {
        let estado: &'static str = {
            if self.get_estado() {
                "Encendido"
            } else {
                "Apagado"
            }
        };

        let texto = format!(
            "[Dosificador][{}][{}]: -- [{}] --",
            self.get_id(),
            self.get_entrega(),
            estado
        );
        println!("{}", texto);
        texto
    }
}
