use types::rango::{Key, Rango};
use uuid::Uuid;

/// La estructura `Silo` representa un silo de almacenamiento de alimento.
///
/// Un silo tiene tres propiedades clave:
/// 1. **alimento**: La cantidad actual de alimento (en kilogramos) almacenada en el silo.
/// 2. **historico**: El total acumulado de alimento que ha pasado por el silo en el pasado.
/// 3. **capacidad**: La capacidad máxima del silo (en kilogramos).
///
/// La estructura incluye métodos para manejar y monitorear el llenado y la entrega de alimento.
///
/// # Ejemplo:
///! ```
///! let mut silo = Silo::new(24000);
///! silo.set_alimento(1000); // Establece la cantidad de alimento a 1000 kg.
///! println!("Alimento actual: {}", silo.get_alimento()); // Imprime 1000.
///! silo.entregar_pulso(200); // Reduce el alimento por 200 kg.
///! println!("Alimento restante: {}", silo.get_alimento()); // Imprime 800.
///! ```
pub struct Silo {
    /// La cantidad de alimento en kilogramos actualmente almacenado en el silo.
    ///
    alimento: Rango,

    /// El total de alimento en kilogramos que ha pasado por el silo históricamente.
    ///
    historico: Rango,

    /// El identificador único del silo.
    id: Uuid,
}

impl Silo {
    /// Crea un nuevo silo con los valores iniciales establecidos en 0.
    ///
    /// Asigna un `UUID` único para cada instancia de silo.
    ///
    /// # Ejemplo:
    ///! ```
    ///! let silo = Silo::new(24000);
    ///! ```
    ///
    /// Esto creará un silo nuevo con atributos `alimento: 0`, `historico: 0`, `capacidad: 24000` y `id: Uuid::new_v4()`.
    pub fn new(capacidad: u32) -> Self {
        Self {
            alimento: Rango::new(0, capacidad, 0).unwrap(),
            historico: Rango::new(0, 4294967295, 0).unwrap(),
            id: Uuid::new_v4(),
        }
    }

    /// Realiza una entrega de alimento desde el silo.
    ///
    /// Disminuye la cantidad de alimento en el silo en el valor de `pulso`.
    ///
    /// # Parámetros:
    /// - `pulso`: La cantidad de alimento a entregar.
    ///
    /// # Retorna:
    /// Una referencia mutable al silo para permitir encadenamiento de métodos.
    ///
    /// # Ejemplo:
    ///! ```
    ///! let mut silo = Silo::new(24000);
    ///! silo.set_alimento(1000);
    ///! silo.entregar_pulso(200);
    ///! assert_eq!(silo.get_alimento(), 800); // La cantidad de alimento disminuye a 800 kg.
    ///! ```
    pub fn entregar_pulso(&mut self, pulso: u32) -> Result<(), crate::errors::SiloError> {
        // Usa el tipo correcto de error
        let alimento_actual = self.get_alimento();

        if pulso > alimento_actual {
            return Err(crate::errors::SiloError::SinAlimento);
        }

        match self.set_alimento(alimento_actual - pulso) {
            Ok(()) => Ok(()),
            Err(crate::errors::SiloError::FueraDeRango) => {
                // Asegúrate de que el error coincida
                println!("Error: Intento de set fuera del rango.");
                Err(crate::errors::SiloError::SinAlimento)
            }
            Err(_) => {
                // Error inesperado
                println!("Error inesperado al intentar actualizar el alimento.");
                Err(crate::errors::SiloError::ErrorInesperado)
            }
        }
    }
}

/// Implementación de getter y setter
impl Silo {
    /// Actualiza el valor histórico del silo sumando los kilogramos de alimento pasados.
    ///
    /// # Parámetros:
    /// - `n`: Los kilogramos de alimento que se van a agregar al histórico.
    ///
    /// # Ejemplo:
    ///! ```
    ///! let mut silo = Silo::new(24000);
    ///! silo.set_historico(1000);
    ///! println!("Historico: {}", silo.get_historico()); // Imprime 1000.
    ///! ```
    fn set_historico(&mut self, n: u32) -> Result<(), crate::errors::SiloError> {
        let mut x = self.historico.get();
        x += n;
        match self.historico.set(x, "[Silo]") {
            Ok(()) => Ok(()),
            Err(_) => Err(crate::errors::SiloError::FueraDeRango),
        }
    }

    /// Establece la cantidad de alimento actual en el silo.
    ///
    /// Si la nueva cantidad es mayor, se actualiza el histórico con la diferencia.
    ///
    /// # Parámetros:
    /// - `n`: La cantidad de alimento a establecer.
    ///
    /// # Retorna:
    /// Una referencia mutable a la instancia actual de `Silo` para permitir el encadenamiento de métodos.
    ///
    /// # Ejemplo:
    ///! ```
    ///! let mut silo = Silo::new(24000);
    ///! silo.set_alimento(1000); // Establece la cantidad de alimento a 1000 kg.
    ///! println!("Alimento actual: {}", silo.get_alimento()); // Imprime 1000.
    ///! ```
    pub fn set_alimento(&mut self, n: u32) -> Result<(), crate::errors::SiloError> {
        if self.alimento.get() < n {
            if let Err(_) = self.set_historico(n - self.alimento.get()) {
                return Err(crate::errors::SiloError::FueraDeRango);
            }
        }

        match self.alimento.set(n, "[Silo]") {
            Ok(()) => Ok(()),
            Err(_) => Err(crate::errors::SiloError::FueraDeRango),
        }
    }

    /// Obtiene la cantidad actual de alimento almacenado en el silo.
    ///
    /// # Retorna:
    /// La cantidad de alimento en kilogramos.
    ///
    /// # Ejemplo:
    ///! ```
    ///! let mut silo = Silo::new(24000);
    ///! silo.set_alimento(1000);
    ///! assert_eq!(silo.get_alimento(), 1000);
    ///! ```
    pub fn get_alimento(&self) -> u32 {
        self.alimento.get()
    }

    /// Obtiene el total histórico de alimento que ha pasado por el silo.
    ///
    /// # Retorna:
    /// El total histórico de alimento en kilogramos.
    ///
    /// # Ejemplo:
    ///! ```
    ///! let mut silo = Silo::new(24000);
    ///! silo.set_historico(1000);
    ///! assert_eq!(silo.get_historico(), 1000);
    ///! ```
    pub fn get_historico(&self) -> u32 {
        self.historico.get()
    }

    /// Obtiene la cantidad de alimento máximo que se puede agregar al silo en dicho momento, es decir devuelve el espacio restante.
    ///
    /// # Retorna:
    /// `true` si es posible agregar `n` kilogramos sin superar la capacidad del silo,
    /// `false` en caso contrario.
    ///
    /// # Ejemplo:
    ///! ```
    ///! let mut silo = Silo::new(24000);
    ///! println!("El espacio restante es: {} y la capacidad máxima es: {}", self.get_espacio_restante(), self.get_capacidad());
    ///! ```
    fn _get_espacio_restante(&self) -> u32 {
        // En este caso puedo asegurar que siempre habrá un key::max
        self.alimento.get_rango().get(&Key::Max).unwrap() - self.alimento.get()
    }

    /// Obtiene el identificador único de la ración.
    ///
    /// # Retorna:
    /// El identificador único de la ración (de tipo `Uuid`).
    ///
    /// # Ejemplo:
    ///! ```
    ///! let silo = Silo::new(24000);
    ///! let id = silo.get_id();
    ///! println!("El ID del silo es: {}", id);
    ///! ```
    pub fn get_id(&self) -> Uuid {
        self.id
    }
}
