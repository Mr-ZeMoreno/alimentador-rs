use hardware::silo::Silo;

const CAPACIDAD_SILOS: u32 = 24000;

#[cfg(test)]
mod tests {
    use super::*;

    /// Prueba la creación de un nuevo dosificador y verifica sus valores iniciales.
    #[test]
    fn test_creacion_silo() {
        let silo = Silo::new(CAPACIDAD_SILOS);

        // Verificar valores iniciales
        assert_eq!(silo.get_alimento(), 0);
        assert_eq!(silo.get_historico(), 0);

        // Verificar que el UUID no sea el mismo en dos instancias diferentes
        let id1 = silo.get_id();
        let silo2 = Silo::new(24000);
        let id2 = silo2.get_id();
        assert_ne!(id1, id2);
    }

    /// Prueba el setter `set_entrega` para modificar la capacidad de entrega del dosificador.
    #[test]
    fn test_set_alimento() {
        let mut silo = Silo::new(CAPACIDAD_SILOS);

        // Comprobamos que set alimento funciona bien
        silo.set_alimento(10);
        assert_eq!(silo.get_alimento(), 10);

        // Comprobamos que el historico aumenta
        silo.set_alimento(11);
        assert_eq!(silo.get_historico(), 11);

        // Comprobamos que el historico no disminuye
        silo.set_alimento(8);
        assert_ne!(silo.get_historico(), 8);
    }

    /// Verifica que el valor de `entrega` se obtiene correctamente.
    #[test]
    fn test_get_alimento() {
        let silo = Silo::new(CAPACIDAD_SILOS);
        assert_eq!(silo.get_alimento(), 0); // Valor inicial
    }

    /// Verifica que el valor de `estado` se obtiene correctamente.
    #[test]
    fn test_get_historico() {
        let mut silo = Silo::new(CAPACIDAD_SILOS);
        assert_eq!(silo.get_historico(), 0); // Valor inicial

        silo.set_alimento(100); // Aumentamo 100 el historico (se supone)
        assert_eq!(silo.get_historico(), 100);

        silo.set_alimento(0); // Al vaciar el silo el historico deberia mantenerse en 100
        assert_eq!(silo.get_historico(), 100);

        silo.set_alimento(100); // Al volver a cargar 100 deberia sumarse con el historico y dar 200
        assert_eq!(silo.get_historico(), 200);
    }

    /// Verifica que el UUID se obtiene correctamente y es un UUID válido.
    #[test]
    fn test_get_id() {
        let silo = Silo::new(CAPACIDAD_SILOS);
        let id = silo.get_id();

        // Verificar que el ID sea un UUID válido (simple check)
        assert!(id.to_string().len() > 0);
    }

    /// Verificar que los pulsos se entregan
    #[test]
    fn test_entregar_pulso() {
        let mut silo = Silo::new(CAPACIDAD_SILOS);

        silo.set_alimento(CAPACIDAD_SILOS);

        silo.entregar_pulso(5);
        assert_eq!(silo.get_alimento(), CAPACIDAD_SILOS - 5);

        silo.entregar_pulso(CAPACIDAD_SILOS - 5);
        assert_eq!(silo.get_alimento(), 0);
    }

    /// Verifica que los setters `set_alimento` y `entregar_pulso` se pueden encadenar correctamente.
    #[test]
    fn test_chain_setters() {
        let mut silo = Silo::new(CAPACIDAD_SILOS);

        // Probar el encadenamiento de setters
        silo.set_alimento(5).entregar_pulso(1);

        assert_eq!(silo.get_alimento(), 4);
    }
}
