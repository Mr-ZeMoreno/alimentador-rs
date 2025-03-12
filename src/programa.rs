use crate::racion::Racion;
use uuid::Uuid;

/// Representa un **Programa** que contiene una lista de raciones y un tiempo de espera para la siguiente ración.
///
/// La estructura `Programa` tiene los siguientes campos:
/// - **raciones**: Un vector de referencias a raciones, que representan las raciones asociadas con el programa.
/// - **racion_espera**: El tiempo en segundos de espera antes de la siguiente ración.
/// - **id**: Un identificador único para cada instancia del programa.
pub struct Programa<'a> {
    /// Lista de referencias a las raciones asociadas con el programa.
    raciones: Vec<&'a Racion>,

    /// Tiempo de espera antes de la siguiente ración en segundos.
    racion_espera: u32,

    /// El identificador único del programa.
    id: Uuid,
}

/// Implementación de los métodos getter y setter para `Programa`.
impl<'a> Programa<'a> {
    /// Crea un nuevo `Programa` con una lista de raciones y un tiempo de espera inicial de 0.
    ///
    /// # Parámetros:
    /// - `raciones`: Un vector de referencias a las raciones asociadas con el programa.
    ///
    /// # Retorna:
    /// Un nuevo objeto `Programa` con las raciones proporcionadas, tiempo de espera en 0 y un ID único generado.
    ///
    /// # Ejemplo:
    /// ```
    /// let raciones = vec![&racion1, &racion2]; // Suponiendo que `racion1` y `racion2` son instancias de `Racion`.
    /// let programa = Programa::new(raciones);
    /// ```
    pub fn new(raciones: Vec<&'a Racion>) -> Self {
        Self {
            raciones,
            racion_espera: 0,
            id: Uuid::new_v4(),
        }
    }
}

impl<'a> Programa<'a> {
    /// Obtiene la lista de raciones asociadas con el programa.
    ///
    /// # Retorna:
    /// Un valor de tipo `&Vec<&'a Racion>`, que es una referencia a un vector de referencias a raciones.
    ///
    /// # Ejemplo:
    /// ```
    /// let raciones = programa.get_raciones();
    /// assert_eq!(raciones.len(), 2); // Suponiendo que el programa tiene 2 raciones asociadas.
    /// ```
    pub fn get_raciones(&self) -> &Vec<&'a Racion> {
        &self.raciones
    }

    /// Establece el tiempo de espera para la siguiente ración.
    ///
    /// # Parámetros:
    /// - `n`: El tiempo de espera en segundos.
    ///
    /// # Retorna:
    /// Una referencia mutable al objeto `Programa` para permitir el encadenamiento de llamadas (method chaining).
    ///
    /// # Ejemplo:
    /// ```
    /// programa.set_racion_espera(30); // Establece un tiempo de espera de 30 segundos.
    /// assert_eq!(programa.get_racion_espera(), 30); // Verifica que el tiempo de espera sea ahora 30 segundos.
    /// ```
    pub fn set_racion_espera(&mut self, n: u32) -> &mut Self {
        self.racion_espera = n;
        self
    }

    /// Obtiene el tiempo de espera actual para la siguiente ración.
    ///
    /// # Retorna:
    /// El tiempo de espera en segundos (de tipo `u32`).
    ///
    /// # Ejemplo:
    /// ```
    /// let tiempo_espera = programa.get_racion_espera();
    /// assert_eq!(tiempo_espera, 30); // El tiempo de espera debería ser 30 segundos.
    /// ```
    pub fn get_racion_espera(&self) -> u32 {
        self.racion_espera
    }

    /// Obtiene el identificador único (UUID) del programa.
    ///
    /// # Retorna:
    /// El identificador único del programa, que es de tipo `Uuid`.
    ///
    /// # Ejemplo:
    /// ```
    /// let id = programa.get_id();
    /// println!("El ID del programa es: {}", id);
    /// ```
    pub fn get_id(&self) -> Uuid {
        self.id
    }
}
