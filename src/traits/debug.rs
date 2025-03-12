use crate::Dosificador;
use crate::Programa;
use crate::Racion;
use crate::Silo;
use crate::Soplador;

/// El trait `Debuggeable` permite que un tipo implemente el método `print`
/// para mostrar su estado de manera legible.
///
/// # Métodos:
/// - `print`: Imprime la información relevante del tipo en la consola.
pub trait Debuggeable {
    fn print(&self) -> &Self;
}

pub trait DebuggeableConArg {
    fn print(&self, arg: u32) -> &Self;
}

impl Debuggeable for Soplador {
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

impl Debuggeable for Dosificador {
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

impl Debuggeable for Racion {
    /// Imprime la información detallada de una ración.
    ///
    /// Muestra el `id`, los `pulsos`, la duración del pulso en milisegundos (`DP`),
    /// y el tiempo de espera entre pulsos (`EP`).
    ///
    /// # Ejemplo:
    /// ```rust
    /// let racion = Racion::new();
    /// racion.print();
    /// ```
    fn print(&self) -> &Racion {
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

impl<'a> Debuggeable for Programa<'a> {
    /// Imprime los detalles del programa, incluyendo su `id` y los `id`s de las raciones asociadas.
    ///
    /// # Ejemplo:
    /// ```rust
    /// let programa = Programa::new(vec![&racion1, &racion2]);
    /// programa.print();
    /// ```
    fn print(&self) -> &Programa<'a> {
        println!(
            "\n[Programa][{}]: Raciones <[{}]>",
            self.get_id(),
            self.get_raciones()
                .iter()
                .map(|racion| racion.get_id().to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );
        self
    }
}

impl DebuggeableConArg for Silo {
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
