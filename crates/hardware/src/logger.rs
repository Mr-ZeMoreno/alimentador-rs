use crate::dosificador::Dosificador;
use crate::silo::Silo;
use crate::soplador::Soplador;

/// El trait `Print` permite que un tipo implemente el método `print`
/// para mostrar su estado de manera legible.
///
/// # Métodos:
/// - `print`: Imprime la información relevante del tipo en la consola.
pub trait Print {
    fn print(&self) -> String;
}

pub trait PrintConArg {
    fn print(&self, arg: u32) -> String;
}

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
