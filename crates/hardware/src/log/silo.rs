use crate::log::Print;
use crate::silo::Silo;

impl Print for Silo {
    /// Imprime la información del silo y la cantidad de alimento que se va a entregar.
    ///
    /// # Parámetros:
    /// - `pulso`: La cantidad de alimento que se entregará.
    ///
    /// # Retorna:
    /// Una referencia al silo para permitir el encadenamiento de métodos.
    ///
    /// # Ejemplo:
    ///! ```
    ///! let mut silo = Silo::new();
    ///! silo.set_alimento(1000);
    ///! silo.print(200);
    ///! ```
    fn print(&self) -> String {
        let texto = format!(
            "\n[Silo][{}][Actual: {}kg][Historico: {}kg]",
            self.get_id(),
            self.get_alimento(),
            self.get_historico()
        );

        println!("{}", texto);
        texto
    }
}
