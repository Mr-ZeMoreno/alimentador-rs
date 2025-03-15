use std::thread;
use std::time::Duration;

/// Wrapper para `std::thread::sleep` que acepta milisegundos.
///
/// Esta función permite hacer una pausa en la ejecución del programa
/// especificando el tiempo en milisegundos.
///
/// # Parámetros:
/// - `ms`: El número de milisegundos a dormir.
///
/// # Ejemplo:
///! ```rust
///! lib::sleep(2000);  // Pausa de 2 segundos
///! ```
pub fn sleep(ms: u32) {
    thread::sleep(Duration::from_millis(ms.into()));
}
