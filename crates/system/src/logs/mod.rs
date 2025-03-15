mod ciclo;
mod racion;

/// El trait `Print` permite que un tipo implemente el método `print`
/// para mostrar su estado de manera legible.
///
/// # Métodos:
/// - `print`: Imprime la información relevante del tipo en la consola.
pub trait Print {
    fn print(&self) -> &Self;
}
