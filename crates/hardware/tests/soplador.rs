use hardware::soplador::Soplador;

#[cfg(test)]
mod avisos {
    // Si no hay comentarios en ignore comentar este modulo
    // #[test]
    // #[ignore = "Por Favor Leer en TODO.md los test de soplador"]
    // fn archivo_test() {}

    // #[test]
    // #[ignore = "Por Favor Leer en TODO.md la implementación de soplador"]
    // fn archivo_implementacion() {}

    // #[test]
    // #[ignore = "Por Favor Leer en TODO.md los errores de soplador"]
    // fn archivo_errores() {}
}

#[cfg(test)]
mod soplador {
    use super::*;

    mod new {
        use super::Soplador;

        /// Prueba que el estado inicial sea false
        #[test]
        fn test_estado_inicial() {
            let soplador = Soplador::new();
            assert_eq!(
                soplador.get_estado(),
                false,
                "El estado inicial no es false"
            );
        }

        /// Prueba que la potencia inicial sea 0
        #[test]
        fn test_potencia_inicial() {
            let soplador = Soplador::new();
            assert_eq!(soplador.get_potencia(), 0, "La potencia inicial no es 0");
        }

        mod id {
            use super::Soplador;
            use uuid::Uuid;

            /// Prueba que la id sea unica
            #[test]
            fn test_unicidad() {
                let soplador = Soplador::new();
                // Verifica unicidad de id
                let id1 = soplador.get_id();
                let id2 = Soplador::new().get_id();
                assert_ne!(id1, id2, "Las ID no son únicas");
            }

            /// Prueba que la id sea un UUID válido
            #[test]
            fn test_validez() {
                let soplador = Soplador::new();
                let id = soplador.get_id();
                let parsed_id = Uuid::parse_str(&id.to_string());
                assert!(parsed_id.is_ok(), "El ID no es un UUID válido");
            }

            #[test]
            fn tets_is_v4() {
                let soplador = Soplador::new();
                let id = soplador.get_id();
                let parsed_id = Uuid::parse_str(&id.to_string());
                assert_eq!(
                    parsed_id.unwrap().get_version(),
                    Some(uuid::Version::Random),
                    "La ID no es un UUID v4"
                );
            }
        }
    }

    /// Prueba que cambia de estado
    #[test]
    fn test_set_estado() {
        let mut soplador = Soplador::new();

        soplador.set_estado(true);
        assert_eq!(soplador.get_estado(), true, "El estado no ha cambiado");
    }

    mod set_potencia {
        use super::Soplador;

        /// Prueba que cambia
        #[test]
        fn test_set_un_valor() {
            let mut soplador = Soplador::new();

            // Probamos que cambia el valor
            soplador
                .set_potencia(100)
                .expect("[test_set_un_valor] Linea 99");
            assert_eq!(soplador.get_potencia(), 100, "La potencia no ha cambiado");
        }

        /// Probamos todos los numeros del rango
        #[test]
        fn test_set_todo_el_rango() {
            let mut soplador = Soplador::new();
            // Probamos que admite todos los valores entre 0 y 100
            for i in 0..=100 {
                // Probamos que cambia el valor
                soplador
                    .set_potencia(i)
                    .expect("[test_set_todo_el_rango] Linea 111");
                assert_eq!(
                    soplador.get_potencia(),
                    i,
                    "La potencia se pudo cambiar por {}",
                    i
                );
            }
        }

        /// Probamos que no pase el 100
        #[test]
        fn set_sobre_el_rango() {
            let mut soplador = Soplador::new();
            // Intentamos establecer una potencia fuera del límite superior
            let result = soplador.set_potencia(102);

            // La potencia debería estar en un rango menor o igual a 100
            // comentario: Al recibir como argumento un u32 el compilador no compila
            //             si ingresamos negativos por lo cual no cree una comprobación
            //             para dicho caso, esto se verá en casi todos los setter del
            //             proyecto donde se trabaje con usigned int
            assert_eq!(
                result,
                Err(hardware::errors::SopladorError::FueraDeRango),
                "Se ha obtenido un error distinto a error fuera de rango"
            );
        }
    }

    mod get_estado {
        use super::Soplador;

        /// Probamos que se obtiene el estado
        #[test]
        fn test_is_booleano() {
            let soplador = Soplador::new();

            assert!(
                soplador.get_estado() == true || soplador.get_estado() == false,
                "El valor devuelto no es un booleano válido"
            );
        }

        /// Probamos que efectivamente obtenemos valores actualizados
        #[test]
        fn test_actualiza_el_valor() {
            let mut soplador = Soplador::new();

            soplador.set_estado(true);
            assert_eq!(soplador.get_estado(), true, "El valor no se ha actualizado");

            soplador.set_estado(false);
            assert_eq!(
                soplador.get_estado(),
                false,
                "El valor no se ha actualizado"
            );
        }
    }

    /// Probamos que obtenemos la potencia
    #[test]
    fn test_get_potencia() {
        let soplador = Soplador::new();

        assert!(
            soplador.get_potencia() <= 100,
            "El valor devuelto no esta entre el rango 0 y 100"
        );
    }

    /// Verifica que los setters `set_alimento` y `entregar_pulso` se pueden encadenar correctamente.
    #[test]
    fn test_chain_setters() {
        let mut soplador = Soplador::new();

        soplador
            .set_estado(true)
            .set_potencia(100)
            .expect("[test_chain_setters] Linea 191");

        assert_eq!(soplador.get_estado(), true);
        assert_eq!(soplador.get_potencia(), 100);
    }
}
