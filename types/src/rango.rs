use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq)]
pub enum Key {
    Min,
    Max,
}

/// Estructura que representa un rango de valores, con un valor inicial
/// que debe estar dentro de los límites definidos por el rango.
///
/// # Campos:
/// - `valor`: El valor actual dentro del rango.
/// - `min`: El valor mínimo permitido.
/// - `max`: El valor máximo permitido.
///
/// # Ejemplo:
///! ```rust
///! let rango = Rango::new(0, 100, 50).unwrap();
///! assert_eq!(rango.get(), 50);
///! ```
pub struct Rango {
    valor: u32,
    min: u32,
    max: u32,
}

impl Rango {
    /// Crea un nuevo `Rango` con un valor inicial dentro de los límites.
    ///
    /// # Parámetros:
    /// - `min`: El valor mínimo permitido en el rango.
    /// - `max`: El valor máximo permitido en el rango.
    /// - `valor_inicial`: El valor que se asigna al crear el `Rango`.
    ///
    /// # Retorna:
    /// `Result<Self, &'static str>`: Un `Ok(Self)` si el valor inicial está dentro del rango,
    /// de lo contrario, un `Err` indicando que el valor inicial está fuera de los límites.
    ///
    /// # Ejemplo:
    ///! ```rust
    ///! let rango = Rango::new(0, 100, 50).unwrap();  // Valor válido
    ///! let rango_invalido = Rango::new(0, 100, 150); // Error
    ///! ```
    pub fn new(min: u32, max: u32, valor_inicial: u32) -> Result<Self, &'static str> {
        if valor_inicial < min || valor_inicial > max {
            return Err("El valor inicial está fuera del rango permitido.");
        }
        Ok(Self {
            valor: valor_inicial,
            min,
            max,
        })
    }

    /// Establece un nuevo valor para el rango.
    ///
    /// # Parámetros:
    /// - `valor`: El nuevo valor que se desea establecer.
    ///
    /// # Retorna:
    /// `Result<(), &'static str>`: Un `Ok(())` si el valor está dentro del rango,
    /// o un `Err` si el valor está fuera de los límites.
    ///
    /// # Ejemplo:
    ///! ```rust
    ///! let mut rango = Rango::new(0, 100, 50).unwrap();
    ///! rango.set(80).unwrap();  // Establece un nuevo valor válido
    ///! rango.set(150);  // Error: El valor está fuera del rango permitido
    ///! ```
    fn set_rango_value(&mut self, valor: u32) -> Result<(), &'static str> {
        if valor < self.min || valor > self.max {
            return Err("El valor está fuera del rango permitido.");
        }
        self.valor = valor;
        Ok(())
    }

    /// Obtiene el valor actual dentro del rango.
    ///
    /// # Retorna:
    /// El valor actual del rango.
    ///
    /// # Ejemplo:
    ///! ```rust
    ///! let rango = Rango::new(0, 100, 50).unwrap();
    ///! assert_eq!(rango.get(), 50);
    ///! ```
    pub fn get(&self) -> u32 {
        self.valor
    }

    pub fn get_rango(&self) -> HashMap<Key, u32> {
        // No cambiar ya que he utilizado unwrap en varios lugares asegurando que esta funcion siempre retornara un hashmap con key::min y key::max
        HashMap::from([(Key::Min, self.min), (Key::Max, self.max)])
    }

    pub fn set(&mut self, valor: u32, tag: &str) {
        let valor_actual = self.get();
        let rango = self.get_rango();

        print!("{}", tag);
        match self.set_rango_value(valor) {
            Ok(()) => {
                println!("El valor ha cambiado de {} a {}", valor_actual, valor);
            }
            Err(_) => {
                println!(
                    "Error: Fuera del rango ({};{}), el valor no ha cambiado. Valor actual: {}",
                    rango.get(&Key::Min).unwrap(),
                    rango.get(&Key::Max).unwrap(),
                    valor_actual
                );
            }
        }
    }
}
