use crate::ciclo::Ciclo;
use types::rango::{Rango, RangoError};
use uuid::Uuid;

/// Espera mínima de 1 minutos
pub const ESPERA_MIN: u32 = 60 * 1000 * 1;

/// Espera máxima de 5 horas
pub const ESPERA_MAX: u32 = 60 * 1000 * 60 * 5;

/// Representa un **Racion** que contiene una lista de ciclos y un tiempo de espera para la siguiente ración.
///
/// La estructura `Racion` tiene los siguientes campos:
/// - **ciclos**: Un vector de referencias a ciclos, que representan las ciclos asociadas con el racion.
/// - **ciclo_espera**: El tiempo en segundos de espera antes de la siguiente ración.
/// - **id**: Un identificador único para cada instancia del racion.
pub struct Racion<'a> {
    /// Lista de referencias a las ciclos asociadas con el racion.
    ciclos: Vec<&'a Ciclo>,

    /// Tiempo de espera antes de la siguiente ración en segundos.
    ciclo_espera: Rango,

    /// El identificador único del racion.
    id: Uuid,
}

/// Implementación de los métodos getter y setter para `Racion`.
impl<'a> Racion<'a> {
    /// Crea un nuevo `Racion` con una lista de ciclos y un tiempo de espera inicial de 0.
    ///
    /// # Parámetros:
    /// - `ciclos`: Un vector de referencias a las ciclos asociadas con el racion.
    ///
    /// # Retorna:
    /// Un nuevo objeto `Racion` con las ciclos proporcionadas, tiempo de espera en 0 y un ID único generado.
    ///
    /// # Ejemplo:
    ///! ```
    ///! let ciclos = vec![&racion1, &racion2]; // Suponiendo que `racion1` y `racion2` son instancias de `Ciclo`.
    ///! let racion = Racion::new(ciclos);
    ///! ```
    pub fn new(ciclos: Vec<&'a Ciclo>) -> Self {
        Self {
            ciclos,
            ciclo_espera: Rango::new(ESPERA_MIN, ESPERA_MAX, ESPERA_MIN).unwrap(),
            id: Uuid::new_v4(),
        }
    }
}

impl<'a> Racion<'a> {
    /// Obtiene la lista de ciclos asociadas con el racion.
    ///
    /// # Retorna:
    /// Un valor de tipo `&Vec<&'a Ciclo>`, que es una referencia a un vector de referencias a ciclos.
    ///
    /// # Ejemplo:
    ///! ```
    ///! let ciclos = racion.get_ciclos();
    ///! assert_eq!(ciclos.len(), 2); // Suponiendo que el racion tiene 2 ciclos asociadas.
    ///! ```
    pub fn get_ciclos(&self) -> &Vec<&'a Ciclo> {
        &self.ciclos
    }

    /// Establece el tiempo de espera para la siguiente ración.
    ///
    /// # Parámetros:
    /// - `n`: El tiempo de espera en segundos.
    ///
    /// # Retorna:
    /// Una referencia mutable al objeto `Racion` para permitir el encadenamiento de llamadas (method chaining).
    ///
    /// # Ejemplo:
    ///! ```
    ///! racion.set_ciclo_espera(30); // Establece un tiempo de espera de 30 segundos.
    ///! assert_eq!(racion.get_ciclo_espera(), 30); // Verifica que el tiempo de espera sea ahora 30 segundos.
    ///! ```
    pub fn set_ciclo_espera(&mut self, n: u32) -> Result<(), crate::errors::RacionError> {
        match self.ciclo_espera.set(n, "[Racion]") {
            Ok(()) => Ok(()),
            Err(RangoError::FueraDeRango) => Err(crate::errors::RacionError::EsperaFueraDeRango),
        }
    }

    /// Obtiene el tiempo de espera actual para el siguiente ciclo.
    ///
    /// # Retorna:
    /// El tiempo de espera en segundos (de tipo `u32`).
    ///
    /// # Ejemplo:
    ///! ```
    ///! let tiempo_espera = racion.get_ciclo_espera();
    ///! assert_eq!(tiempo_espera, 30); // El tiempo de espera debería ser 30 segundos.
    ///! ```
    pub fn get_ciclo_espera(&self) -> u32 {
        self.ciclo_espera.get()
    }

    /// Obtiene el identificador único (UUID) del racion.
    ///
    /// # Retorna:
    /// El identificador único del racion, que es de tipo `Uuid`.
    ///
    /// # Ejemplo:
    ///! ```
    ///! let id = racion.get_id();
    ///! println!("El ID del racion es: {}", id);
    ///! ```
    pub fn get_id(&self) -> Uuid {
        self.id
    }
}
