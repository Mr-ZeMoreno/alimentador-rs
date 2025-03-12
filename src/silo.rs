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
/// ```
/// let mut silo = Silo::new();
/// silo.set_alimento(1000); // Establece la cantidad de alimento a 1000 kg.
/// println!("Alimento actual: {}", silo.get_alimento()); // Imprime 1000.
/// silo.entregar_pulso(200); // Reduce el alimento por 200 kg.
/// println!("Alimento restante: {}", silo.get_alimento()); // Imprime 800.
/// ```
pub struct Silo {
    /// La cantidad de alimento en kilogramos actualmente almacenado en el silo.
    ///
    /// * Máximo: 4,294,967,295 kg.
    alimento: u32,

    /// El total de alimento en kilogramos que ha pasado por el silo históricamente.
    ///
    /// * Máximo: 4,294,967,295 kg.
    historico: u32,

    /// La capacidad máxima que puede contener el silo (en kilogramos).
    /// Por lo general se trabaja con silos de 25000 kg.
    ///
    /// * Máximo: 65,535 kg.
    capacidad: u32,

    /// El identificador único del silo.
    id: Uuid,
}

impl Silo {
    /// Crea un nuevo silo con los valores iniciales establecidos en 0.
    ///
    /// Asigna un `UUID` único para cada instancia de silo.
    ///
    /// # Ejemplo:
    /// ```
    /// let silo = Silo::new();
    /// ```
    ///
    /// Esto creará un silo nuevo con atributos `alimento: 0`, `historico: 0`, `capacidad: 0` y `id: Uuid::new_v4()`.
    pub fn new() -> Self {
        Self {
            alimento: 0,
            historico: 0,
            capacidad: 0,
            id: Uuid::new_v4(),
        }
    }

    /// Actualiza el valor histórico del silo sumando los kilogramos de alimento pasados.
    ///
    /// # Parámetros:
    /// - `n`: Los kilogramos de alimento que se van a agregar al histórico.
    ///
    /// # Ejemplo:
    /// ```
    /// let mut silo = Silo::new();
    /// silo.set_historico(1000);
    /// println!("Historico: {}", silo.get_historico()); // Imprime 1000.
    /// ```
    fn set_historico(&mut self, n: u32) {
        self.historico += n;
    }
    /// Verifica si se puede agregar más alimento al silo sin exceder su capacidad máxima.
    ///
    /// # Parámetros:
    /// - `n`: La cantidad de alimento (en kilogramos) que se quiere agregar al silo.
    ///
    /// # Retorna:
    /// `true` si es posible agregar `n` kilogramos sin superar la capacidad del silo,
    /// `false` en caso contrario.
    ///
    /// # Ejemplo:
    /// ```
    /// let mut silo = Silo::new();
    /// silo.capacidad = 5000; // Establecemos la capacidad máxima del silo a 5000 kg.
    /// assert!(silo.verificar_llenado(4000)); // Devuelve `true` porque podemos agregar 4000 kg sin exceder la capacidad.
    /// assert!(!silo.verificar_llenado(6000)); // Devuelve `false` porque no podemos agregar 6000 kg.
    /// ```
    fn verificar_llenado(&self, n: u32) -> bool {
        n + self.alimento <= self.capacidad
    }

    /// Obtiene la cantidad de alimento máximo que se puede agregar al silo en dicho momento, es decir devuelve el espacio restante.
    ///
    /// # Retorna:
    /// `true` si es posible agregar `n` kilogramos sin superar la capacidad del silo,
    /// `false` en caso contrario.
    ///
    /// # Ejemplo:
    /// ```
    /// let mut silo = Silo::new();
    /// println!("El espacio restante es: {} y la capacidad máxima es: {}", self.get_espacio_restante(), self.get_capacidad());
    /// ```
    fn get_espacio_restante(&self) -> u32 {
        self.capacidad - self.alimento
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
    /// ```
    /// let mut silo = Silo::new();
    /// silo.set_alimento(1000); // Establece la cantidad de alimento a 1000 kg.
    /// println!("Alimento actual: {}", silo.get_alimento()); // Imprime 1000.
    /// ```
    pub fn set_alimento(&mut self, n: u32) -> &mut Silo {
        if self.alimento < n {
            self.set_historico(n - self.alimento);
        }
        self.alimento = n;
        self
    }

    /// Obtiene la cantidad actual de alimento almacenado en el silo.
    ///
    /// # Retorna:
    /// La cantidad de alimento en kilogramos.
    ///
    /// # Ejemplo:
    /// ```
    /// let mut silo = Silo::new();
    /// silo.set_alimento(1000);
    /// assert_eq!(silo.get_alimento(), 1000);
    /// ```
    pub fn get_alimento(&self) -> u32 {
        self.alimento
    }

    /// Obtiene el total histórico de alimento que ha pasado por el silo.
    ///
    /// # Retorna:
    /// El total histórico de alimento en kilogramos.
    ///
    /// # Ejemplo:
    /// ```
    /// let mut silo = Silo::new();
    /// silo.set_historico(1000);
    /// assert_eq!(silo.get_historico(), 1000);
    /// ```
    pub fn get_historico(&self) -> u32 {
        self.historico
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
    /// ```
    /// let mut silo = Silo::new();
    /// silo.set_alimento(1000);
    /// silo.entregar_pulso(200);
    /// assert_eq!(silo.get_alimento(), 800); // La cantidad de alimento disminuye a 800 kg.
    /// ```
    pub fn entregar_pulso(&mut self, pulso: u32) -> &mut Silo {
        self.set_alimento(self.get_alimento() - pulso)
    }

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
    /// silo.print_silo(200);
    /// ```
    pub fn print_silo(&self, pulso: u32) -> &Silo {
        println!("\nSilo: {} - Historico: {}", self.alimento, self.historico);
        println!("Entregando {} kg\n", pulso);
        self
    }
}
