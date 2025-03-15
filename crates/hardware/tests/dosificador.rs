use hardware::dosificador::Dosificador;

const ENTREGA: u32 = 10;

// #[cfg(test)]
// mod avisos {
//     // Si no hay comentarios en ignore comentar este modulo
//     #[test]
//     #[ignore = "Por Favor Leer en TODO.md los test de dosificador"]
//     fn archivo_test() {}

//     #[test]
//     #[ignore = "Por Favor Leer en TODO.md la implementación de dosificador"]
//     fn archivo_implementacion() {}

//     #[test]
//     #[ignore = "Por Favor Leer en TODO.md los errores de dosificador"]
//     fn archivo_errores() {}
// }

#[cfg(test)]
mod dosificador {
    use super::*;

    use uuid::Uuid;

    /// Este test comprueba que el objeto inicial tiene los atributos básicos en sus valores
    #[test]
    fn test_creacion_dosificador() {
        let dosificador = Dosificador::new(ENTREGA);

        // Verificar valores iniciales
        assert_eq!(dosificador.get_entrega(), ENTREGA);
        assert_eq!(dosificador.get_estado(), false);

        // Verificar unicidad del UUID (que es única la weá)
        let id1 = dosificador.get_id();
        let id2 = Dosificador::new(ENTREGA).get_id();
        assert_ne!(id1, id2, "Las ID no son únicas");
    }

    /// Prueba que el estado cambia mediante el método set_estado
    #[test]
    fn test_set_estado() {
        let mut dosificador = Dosificador::new(ENTREGA);
        dosificador.set_estado(true);
        assert_eq!(dosificador.get_estado(), true);
    }

    #[test]
    fn test_get_id() {
        let dosificador = Dosificador::new(ENTREGA);
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
}
