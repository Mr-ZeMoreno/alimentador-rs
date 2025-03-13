use hardware::dosificador::Dosificador;

#[cfg(test)]
mod tests {
    use super::*;

    /// Prueba la creación de un nuevo dosificador y verifica sus valores iniciales.
    #[test]
    fn test_creacion_dosificador() {
        let dosificador = Dosificador::new();

        // Verificar valores iniciales
        assert_eq!(dosificador.get_entrega(), 0);
        assert_eq!(dosificador.get_estado(), false);
        // Verificar que el UUID no sea el mismo en dos instancias diferentes
        let id1 = dosificador.get_id();
        let dosificador2 = Dosificador::new();
        let id2 = dosificador2.get_id();
        assert_ne!(id1, id2);
    }

    /// Prueba el setter `set_entrega` para modificar la capacidad de entrega del dosificador.
    #[test]
    fn test_set_entrega() {
        let mut dosificador = Dosificador::new();
        dosificador.set_entrega(10);

        assert_eq!(dosificador.get_entrega(), 10);
    }

    /// Prueba el setter `set_estado` para modificar el estado del dosificador.
    #[test]
    fn test_set_estado() {
        let mut dosificador = Dosificador::new();
        dosificador.set_estado(true);

        assert_eq!(dosificador.get_estado(), true);
    }

    /// Verifica que el valor de `entrega` se obtiene correctamente.
    #[test]
    fn test_get_entrega() {
        let dosificador = Dosificador::new();
        assert_eq!(dosificador.get_entrega(), 0); // Valor inicial
    }

    /// Verifica que el valor de `estado` se obtiene correctamente.
    #[test]
    fn test_get_estado() {
        let dosificador = Dosificador::new();
        assert_eq!(dosificador.get_estado(), false); // Valor inicial
    }

    /// Verifica que el UUID se obtiene correctamente y es un UUID válido.
    #[test]
    fn test_get_id() {
        let dosificador = Dosificador::new();
        let id = dosificador.get_id();

        // Verificar que el ID sea un UUID válido (simple check)
        assert!(id.to_string().len() > 0);
    }

    /// Verifica que los setters `set_entrega` y `set_estado` se pueden encadenar correctamente.
    #[test]
    fn test_chain_setters() {
        let mut dosificador = Dosificador::new();

        // Probar el encadenamiento de setters
        dosificador.set_entrega(5).set_estado(true);

        assert_eq!(dosificador.get_entrega(), 5);
        assert_eq!(dosificador.get_estado(), true);
    }
}
