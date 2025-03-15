use types::rango::{Rango, RangoData};
use uuid::Uuid;

pub const ESPERA_MIN: u32 = 1000;
pub const ESPERA_MAX: u32 = 20000;

pub const PULSOS_MIN: u32 = 0;
pub const PULSOS_MAX: u32 = 10000;

pub const DURACION_MIN: u32 = 1000;
pub const DURACION_MAX: u32 = 10000;

pub struct CicloData {
    pub pulsos: RangoData,
    pub duracion: RangoData,
    pub espera: RangoData,
}

const TAG: &str = "[Ciclo]";

/// Representa una **Ración**, que contiene parámetros para la duración y el comportamiento de los pulsos.
///
/// La estructura `Ciclo` tiene los siguientes campos:
/// - **pulso_duracion**: La duración de cada pulso en milisegundos. No debería durar más de un minuto en producción.
/// - **pulsos**: El número total de pulsos por ración.
/// - **pulso_espera**: El tiempo entre cada pulso en milisegundos.
/// - **id**: Un identificador único para cada instancia de la ración.
#[derive(PartialEq, Debug)]
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
    ///! ```
    ///! let Ciclo = Ciclo::new();
    ///! assert_eq!(Ciclo.get_pulsos(), 0); // La cantidad de pulsos debería ser 0 por defecto.
    ///! ```
    pub fn new() -> Self {
        Self {
            pulso_duracion: Rango::new(DURACION_MIN, DURACION_MAX, DURACION_MIN).unwrap(),
            pulsos: Rango::new(PULSOS_MIN, PULSOS_MAX, PULSOS_MIN).unwrap(),
            pulso_espera: Rango::new(ESPERA_MIN, ESPERA_MAX, ESPERA_MIN).unwrap(),
            id: Uuid::new_v4(),
        }
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
    ///! ```
    ///! let mut Ciclo = Ciclo::new();
    ///! Ciclo.set_pulso_duracion(500); // Establece la duración del pulso a 500 ms.
    ///! assert_eq!(Ciclo.get_pulso_duracion(), 500); // Verifica que la duración del pulso sea 500 ms.
    ///! ```
    pub fn set_pulso_duracion(&mut self, n: u32) -> Result<(), crate::errors::CicloError> {
        match self.pulso_duracion.set(n, TAG) {
            Ok(()) => Ok(()),
            Err(types::rango::RangoError::FueraDeRango) => {
                Err(crate::errors::CicloError::DuracionFueraDeRango)
            }
        }
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
    ///! ```
    ///! let mut Ciclo = Ciclo::new();
    ///! Ciclo.set_pulsos(10); // Establece 10 pulsos por ración.
    ///! assert_eq!(Ciclo.get_pulsos(), 10); // Verifica que el número de pulsos sea 10.
    ///! ```
    pub fn set_pulsos(&mut self, n: u32) -> Result<(), crate::errors::CicloError> {
        match self.pulsos.set(n, TAG) {
            Ok(()) => Ok(()),
            Err(types::rango::RangoError::FueraDeRango) => {
                Err(crate::errors::CicloError::CantidadFueraDeRango)
            }
        }
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
    ///! ```
    ///! let mut Ciclo = Ciclo::new();
    ///! Ciclo.set_pulso_espera(100); // Establece el tiempo de espera entre pulsos a 100 ms.
    ///! assert_eq!(Ciclo.get_pulso_espera(), 100); // Verifica que el tiempo de espera sea 100 ms.
    ///! ```
    pub fn set_pulso_espera(&mut self, n: u32) -> Result<(), crate::errors::CicloError> {
        match self.pulso_espera.set(n, TAG) {
            Ok(()) => Ok(()),
            Err(types::rango::RangoError::FueraDeRango) => {
                Err(crate::errors::CicloError::EsperaFueraDeRango)
            }
        }
    }

    /// Obtiene el número total de pulsos por ración.
    ///
    /// # Retorna:
    /// El número total de pulsos por ración (de tipo `u32`).
    ///
    /// # Ejemplo:
    ///! ```
    ///! let Ciclo = Ciclo::new();
    ///! assert_eq!(Ciclo.get_pulsos(), 0); // Debería ser 0 por defecto.
    ///! ```
    pub fn get_pulsos(&self) -> u32 {
        self.pulsos.get()
    }

    /// Obtiene el tiempo de espera entre pulsos en milisegundos.
    ///
    /// # Retorna:
    /// El tiempo de espera entre pulsos (de tipo `u32`).
    ///
    /// # Ejemplo:
    ///! ```
    ///! let Ciclo = Ciclo::new();
    ///! assert_eq!(Ciclo.get_pulso_espera(), 0); // Debería ser 0 por defecto.
    ///! ```
    pub fn get_pulso_espera(&self) -> u32 {
        self.pulso_espera.get()
    }

    /// Obtiene la duración de cada pulso en milisegundos.
    ///
    /// # Retorna:
    /// La duración del pulso en milisegundos (de tipo `u32`).
    ///
    /// # Ejemplo:
    ///! ```
    ///! let Ciclo = Ciclo::new();
    ///! assert_eq!(Ciclo.get_pulso_duracion(), 0); // Debería ser 0 por defecto.
    ///! ```
    pub fn get_pulso_duracion(&self) -> u32 {
        self.pulso_duracion.get()
    }

    /// Obtiene todos los parámetros de la ración como un array de 3 elementos:
    /// - Pulsos
    /// - Duración del pulso
    /// - Tiempo de espera entre pulsos
    ///
    /// # Retorna:
    /// Estructura ciclo data
    ///
    /// # Ejemplo:
    ///! ```
    ///! let Ciclo = Ciclo::new();
    ///! let parametros = Ciclo.get_all();
    ///! ```
    pub fn get_all(&self) -> CicloData {
        CicloData {
            pulsos: self.pulsos.get_rango(),
            duracion: self.pulso_duracion.get_rango(),
            espera: self.pulso_espera.get_rango(),
        }
    }

    /// Obtiene el identificador único de la ración.
    ///
    /// # Retorna:
    /// El identificador único de la ración (de tipo `Uuid`).
    ///
    /// # Ejemplo:
    ///! ```
    ///! let Ciclo = Ciclo::new();
    ///! let id = Ciclo.get_id();
    ///! println!("El ID de la ración es: {}", id);
    ///! ```
    pub fn get_id(&self) -> Uuid {
        self.id
    }
}
