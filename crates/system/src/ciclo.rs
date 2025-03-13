use types::rango::Rango;
use uuid::Uuid;

/// Representa una **Ración**, que contiene parámetros para la duración y el comportamiento de los pulsos.
///
/// La estructura `Ciclo` tiene los siguientes campos:
/// - **pulso_duracion**: La duración de cada pulso en milisegundos. No debería durar más de un minuto en producción.
/// - **pulsos**: El número total de pulsos por ración.
/// - **pulso_espera**: El tiempo entre cada pulso en milisegundos.
/// - **id**: Un identificador único para cada instancia de la ración.
pub struct Ciclo {
    /// Duración de cada pulso en milisegundos. No debe ser mayor a un minuto en producción.
    pulso_duracion: Rango,

    /// Número total de pulsos por ración.
    pulsos: Rango,

    /// Tiempo de espera entre pulsos en milisegundos.
    pulso_espera: Rango,

    /// Identificador único de la ración.
    id: Uuid,
}

impl Ciclo {
    /// Crea una nueva instancia de `Ciclo` con valores predeterminados.
    /// Todos los campos se inicializan en `0`, y se genera un ID único para la ración.
    ///
    /// # Retorna:
    /// Un nuevo objeto `Ciclo` con los valores predeterminados y un ID único generado.
    ///
    /// # Ejemplo:
    /// ```
    /// let Ciclo = Ciclo::new();
    /// assert_eq!(Ciclo.get_pulsos(), 0); // La cantidad de pulsos debería ser 0 por defecto.
    /// ```
    pub fn new() -> Self {
        Self {
            pulso_duracion: Rango::new(1000, 10000, 1000).unwrap(),
            pulsos: Rango::new(0, 10000, 0).unwrap(),
            pulso_espera: Rango::new(1000, 20000, 1000).unwrap(),
            id: Uuid::new_v4(),
        }
    }

    fn get_tag(&self, nombre: &str) -> String {
        format!("[ración][{}][{}]: ", self.id, nombre)
    }
}

/// Implementación de los métodos getter y setter para la estructura `Ciclo`.
impl Ciclo {
    /// Establece la duración de cada pulso en milisegundos.
    ///
    /// # Parámetros:
    /// - `n`: La duración del pulso en milisegundos.
    ///
    /// # Retorna:
    /// Una referencia mutable al objeto `Ciclo` para permitir el encadenamiento de llamadas (method chaining).
    ///
    /// # Ejemplo:
    /// ```
    /// let mut Ciclo = Ciclo::new();
    /// Ciclo.set_pulso_duracion(500); // Establece la duración del pulso a 500 ms.
    /// assert_eq!(Ciclo.get_pulso_duracion(), 500); // Verifica que la duración del pulso sea 500 ms.
    /// ```
    pub fn set_pulso_duracion(&mut self, n: u32) -> &mut Self {
        self.pulso_duracion.set(n, &self.get_tag("pulsos_duracion"));
        self
    }

    /// Establece el número total de pulsos por ración.
    ///
    /// # Parámetros:
    /// - `n`: El número total de pulsos por ración.
    ///
    /// # Retorna:
    /// Una referencia mutable al objeto `Ciclo` para permitir el encadenamiento de llamadas.
    ///
    /// # Ejemplo:
    /// ```
    /// let mut Ciclo = Ciclo::new();
    /// Ciclo.set_pulsos(10); // Establece 10 pulsos por ración.
    /// assert_eq!(Ciclo.get_pulsos(), 10); // Verifica que el número de pulsos sea 10.
    /// ```
    pub fn set_pulsos(&mut self, n: u32) -> &mut Self {
        self.pulsos.set(n, &self.get_tag("pulsos"));
        self
    }

    /// Establece el tiempo de espera entre pulsos en milisegundos.
    ///
    /// # Parámetros:
    /// - `n`: El tiempo de espera entre pulsos en milisegundos.
    ///
    /// # Retorna:
    /// Una referencia mutable al objeto `Ciclo` para permitir el encadenamiento de llamadas.
    ///
    /// # Ejemplo:
    /// ```
    /// let mut Ciclo = Ciclo::new();
    /// Ciclo.set_pulso_espera(100); // Establece el tiempo de espera entre pulsos a 100 ms.
    /// assert_eq!(Ciclo.get_pulso_espera(), 100); // Verifica que el tiempo de espera sea 100 ms.
    /// ```
    pub fn set_pulso_espera(&mut self, n: u32) -> &mut Self {
        self.pulso_espera.set(n, &self.get_tag("pulsos_espera"));
        self
    }

    /// Obtiene el número total de pulsos por ración.
    ///
    /// # Retorna:
    /// El número total de pulsos por ración (de tipo `u32`).
    ///
    /// # Ejemplo:
    /// ```
    /// let Ciclo = Ciclo::new();
    /// assert_eq!(Ciclo.get_pulsos(), 0); // Debería ser 0 por defecto.
    /// ```
    pub fn get_pulsos(&self) -> u32 {
        self.pulsos.get()
    }

    /// Obtiene el tiempo de espera entre pulsos en milisegundos.
    ///
    /// # Retorna:
    /// El tiempo de espera entre pulsos (de tipo `u32`).
    ///
    /// # Ejemplo:
    /// ```
    /// let Ciclo = Ciclo::new();
    /// assert_eq!(Ciclo.get_pulso_espera(), 0); // Debería ser 0 por defecto.
    /// ```
    pub fn get_pulso_espera(&self) -> u32 {
        self.pulso_espera.get()
    }

    /// Obtiene la duración de cada pulso en milisegundos.
    ///
    /// # Retorna:
    /// La duración del pulso en milisegundos (de tipo `u32`).
    ///
    /// # Ejemplo:
    /// ```
    /// let Ciclo = Ciclo::new();
    /// assert_eq!(Ciclo.get_pulso_duracion(), 0); // Debería ser 0 por defecto.
    /// ```
    pub fn get_pulso_duracion(&self) -> u32 {
        self.pulso_duracion.get()
    }

    /// Obtiene todos los parámetros de la ración como un array de 3 elementos:
    /// - Pulsos
    /// - Duración del pulso
    /// - Tiempo de espera entre pulsos
    ///
    /// # Retorna:
    /// Un array de tres elementos (`[u32; 3]`), representando los pulsos, la duración del pulso y el tiempo de espera entre pulsos.
    ///
    /// # Ejemplo:
    /// ```
    /// let Ciclo = Ciclo::new();
    /// let parametros = Ciclo.get_all();
    /// assert_eq!(parametros, [0, 0, 0]); // Todos los valores deberían ser 0 por defecto.
    /// ```
    pub fn get_all(&self) -> [u32; 3] {
        [
            self.pulsos.get(),
            self.pulso_duracion.get(),
            self.pulso_espera.get(),
        ]
    }

    /// Obtiene el identificador único de la ración.
    ///
    /// # Retorna:
    /// El identificador único de la ración (de tipo `Uuid`).
    ///
    /// # Ejemplo:
    /// ```
    /// let Ciclo = Ciclo::new();
    /// let id = Ciclo.get_id();
    /// println!("El ID de la ración es: {}", id);
    /// ```
    pub fn get_id(&self) -> Uuid {
        self.id
    }
}
