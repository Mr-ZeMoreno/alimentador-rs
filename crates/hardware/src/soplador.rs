use utils::utils::sleep;
use uuid::Uuid;

use types::rango::{Rango, RangoError};

/// Representa un **Soplador**, que puede ser encendido o apagado y tener su potencia ajustada.
///
/// La estructura `Soplador` tiene las siguientes propiedades:
/// - **estado**: El estado actual del soplador. `true` si está encendido, `false` si está apagado.
/// - **potencia**: La potencia del soplador, que puede ir de 0 a 100 (representado por un número de 8 bits sin signo).
/// - **id**: Un identificador único para cada instancia del soplador.
pub struct Soplador {
    /// El estado del soplador (encendido o apagado).
    estado: bool,

    /// La potencia del soplador, con un rango de 0 a 100.
    potencia: Rango,

    /// El identificador único del soplador.
    id: Uuid,
}

impl Soplador {
    /// Crea un nuevo `Soplador` con el estado apagado, potencia en 0 y un ID único generado al azar.
    ///
    /// # Ejemplo:
    ///! ```
    ///! let soplador = Soplador::new();
    ///! assert_eq!(soplador.get_estado(), false);  // El soplador debería estar apagado por defecto.
    ///! assert_eq!(soplador.get_potencia(), 0);   // La potencia debería ser 0 por defecto.
    ///! ```
    pub fn new() -> Self {
        Self {
            estado: false,
            potencia: Rango::new(0, 100, 0).unwrap(),
            id: Uuid::new_v4(),
        }
    }
}

/// Implementación de los métodos getter y setter para `Soplador`.
impl Soplador {
    /// Establece el estado del soplador (encendido o apagado).
    /// Si el estado es `true`, el soplador se enciende y se simula una duración de 5 segundos con `sleep`.
    ///
    /// # Parámetros:
    /// - `n`: Un valor booleano que indica el estado deseado. `true` para encender el soplador, `false` para apagarlo.
    ///
    /// # Retorna:
    /// Devuelve una referencia mutable al objeto `Soplador` para permitir el encadenamiento de llamadas (method chaining).
    ///
    /// # Ejemplo:
    ///! ```
    ///! let mut soplador = Soplador::new();
    ///! soplador.set_estado(true); // Enciende el soplador.
    ///! assert_eq!(soplador.get_estado(), true); // Verifica que el estado ahora es `true` (encendido).
    ///! ```
    pub fn set_estado(&mut self, n: bool) -> &mut Soplador {
        if n {
            println!("[Soplador][{}]: Encendiendo... Duración 5s", self.id);
            sleep(5000); // Simula el encendido por 5 segundos.
        }
        self.estado = n;
        self
    }

    /// Establece la potencia del soplador.
    ///
    /// La potencia es un valor entre 0 y 100.
    ///
    /// # Parámetros:
    /// - `n`: Un valor de potencia que debe estar en el rango de 0 a 100.
    ///
    /// # Retorna:
    /// Devuelve una referencia mutable al objeto `Soplador` para permitir el encadenamiento de llamadas.
    ///
    /// # Ejemplo:
    ///! ```
    ///! let mut soplador = Soplador::new();
    ///! soplador.set_potencia(75); // Establece la potencia del soplador al 75%.
    ///! assert_eq!(soplador.get_potencia(), 75).expect("Se intentó insertar una potencia superior al rango"); // Verifica que la potencia ahora es 75.
    ///! ```
    pub fn set_potencia(&mut self, n: u32) -> Result<(), crate::errors::SopladorError> {
        match self.potencia.set(n, "[Soplador]") {
            Ok(()) => Ok(()),
            Err(RangoError::FueraDeRango) => Err(crate::errors::SopladorError::FueraDeRango),
        }
    }

    /// Obtiene el estado actual del soplador (encendido o apagado).
    ///
    /// # Retorna:
    /// - `true` si el soplador está encendido.
    /// - `false` si el soplador está apagado.
    ///
    /// # Ejemplo:
    ///! ```
    ///! let soplador = Soplador::new();
    ///! assert_eq!(soplador.get_estado(), false); // El soplador está apagado por defecto.
    ///! ```
    pub fn get_estado(&self) -> bool {
        self.estado
    }

    /// Obtiene la potencia actual del soplador.
    ///
    /// # Retorna:
    /// Un valor de tipo `u8` entre 0 y 100 que representa la potencia del soplador.
    ///
    /// # Ejemplo:
    ///! ```
    ///! let soplador = Soplador::new();
    ///! assert_eq!(soplador.get_potencia(), 0); // La potencia es 0 por defecto.
    ///! ```
    pub fn get_potencia(&self) -> u32 {
        self.potencia.get()
    }

    /// Obtiene el identificador único (UUID) del soplador.
    ///
    /// # Retorna:
    /// El identificador único del soplador, que es de tipo `Uuid`.
    ///
    /// # Ejemplo:
    ///! ```
    ///! let soplador = Soplador::new();
    ///! let id = soplador.get_id();
    ///! println!("El ID del soplador es: {}", id);
    ///! ```
    pub fn get_id(&self) -> Uuid {
        self.id
    }
}
