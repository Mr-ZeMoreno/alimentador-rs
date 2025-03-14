use hardware::dosificador::Dosificador;

#[cfg(test)]
mod dosificador {
    use super::*;

    use uuid::Uuid;

    #[test]
    fn test_creacion_dosificador() {
        let dosificador = Dosificador::new();

        // Verificar valores iniciales
        assert_eq!(dosificador.get_entrega(), 0);
        assert_eq!(dosificador.get_estado(), false);

        // Verificar unicidad del UUID (que es única la weá)
        let id1 = dosificador.get_id();
        let id2 = Dosificador::new().get_id();
        assert_ne!(id1, id2, "Las ID no son únicas");
    }

    #[test]
    fn test_set_entrega() {
        let mut dosificador = Dosificador::new();
        dosificador.set_entrega(10);
        assert_eq!(dosificador.get_entrega(), 10);
    }

    #[test]
    fn test_set_estado() {
        let mut dosificador = Dosificador::new();
        dosificador.set_estado(true);
        assert_eq!(dosificador.get_estado(), true);
    }

    #[test]
    fn test_get_id() {
        let dosificador = Dosificador::new();
        let id = dosificador.get_id();

        // Verificar que el ID sea un UUID v4 válido
        let parsed_id = Uuid::parse_str(&id.to_string());
        assert!(parsed_id.is_ok(), "El ID no es un UUID válido");
        assert_eq!(
            parsed_id.unwrap().get_version(),
            Some(uuid::Version::Random),
            "La ID no es un UUID v4"
        );
    }

    #[test]
    fn test_chain_setters() {
        let mut dosificador = Dosificador::new();
        dosificador.set_entrega(5).set_estado(true);
        assert_eq!(dosificador.get_entrega(), 5);
        assert_eq!(dosificador.get_estado(), true);
    }
}
