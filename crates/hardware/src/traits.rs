use crate::dosificador::Dosificador;
use crate::silo::Silo;
use crate::soplador::Soplador;

/// El trait `Print` permite que un tipo implemente el método `print`
/// para mostrar su estado de manera legible.
///
/// # Métodos:
/// - `print`: Imprime la información relevante del tipo en la consola.
pub trait Print {
    fn print(&self) -> &Self;
}

pub trait PrintConArg {
    fn print(&self, arg: u32) -> &Self;
}

impl Print for Soplador {
    /// Imprime el estado del soplador.
    ///
    /// Muestra el `id`, la `potencia`, y si está `encendido` o `apagado`.
    ///
    /// # Ejemplo:
    /// ```rust
    /// let soplador = Soplador::new();
    /// soplador.print();
    /// ```
    fn print(&self) -> &Soplador {
        let estado: &'static str = {
            if self.get_estado() {
                "Encendido"
            } else {
                "Apagado"
            }
        };
        println!(
            "[Soplador][{}],[{}]: -- [{}] --",
            self.get_id(),
            self.get_potencia(),
            estado
        );
        self
    }
}

impl Print for Dosificador {
    /// Imprime el estado del dosificador.
    ///
    /// Muestra el `id` y si el dosificador está `encendido` o `apagado`.
    ///
    /// # Ejemplo:
    /// ```rust
    /// let dosificador = Dosificador::new();
    /// dosificador.print();
    /// ```
    fn print(&self) -> &Dosificador {
        let estado: &'static str = {
            if self.get_estado() {
                "Encendido"
            } else {
                "Apagado"
            }
        };
        println!("\nDosificador [{}]: {}\n", self.get_id(), estado);
        self
    }
}

impl PrintConArg for Silo {
    /// Imprime la información del silo y la cantidad de alimento que se va a entregar.
    ///
    /// # Parámetros:
    /// - `pulso`: La cantidad de alimento que se entregará.
    ///
    /// # Retorna:
    /// Una referencia al silo para permitir el encadenamiento de métodos.
    ///
    /// # Ejemplo:
    /// ```
    /// let mut silo = Silo::new();
    /// silo.set_alimento(1000);
    /// silo.print(200);
    /// ```
    fn print(&self, pulso: u32) -> &Silo {
        println!(
            "\nSilo: {} - Historico: {}",
            self.get_alimento(),
            self.get_historico()
        );
        println!("Entregando {} kg\n", pulso);
        self
    }
}
