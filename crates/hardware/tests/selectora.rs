use hardware::selectora::Selectora;
const POSICION_MAXIMA: u32 = 5;

// #[cfg(test)]
// mod avisos {
//     // Si no hay comentarios en ignore comentar este modulo
//     #[test]
//     #[ignore = "Por Favor Leer en TODO.md los test de los logs"]
//     fn archivo_test() {}

//     #[test]
//     #[ignore = "Por Favor Leer en TODO.md la implementación de traits"]
//     fn archivo_implementacion() {}

//     #[test]
//     #[ignore = "Por Favor Leer en TODO.md los errores de traits"]
//     fn archivo_errores() {}
// }

#[cfg(test)]
mod selectora {
    use super::*;

    mod new {
        use super::*;

        #[test]
        fn test_posicion_inicial() {
            let selectora = Selectora::new(POSICION_MAXIMA);

            assert_eq!(selectora.get_posicion(), 0, "La posicion inicial no es 0");
        }

        mod id {
            use super::*;
            use uuid::Uuid;

            /// Prueba que la id sea unica
            #[test]
            fn test_unicidad() {
                // Verifica unicidad de id
                let id1 = Selectora::new(POSICION_MAXIMA).get_id();
                let id2 = Selectora::new(POSICION_MAXIMA).get_id();
                assert_ne!(id1, id2, "Las ID no son únicas");
            }

            /// Prueba que la id sea un UUID válido
            #[test]
            fn test_validez() {
                let id = Selectora::new(POSICION_MAXIMA).get_id();
                let parsed_id = Uuid::parse_str(&id.to_string());
                assert!(parsed_id.is_ok(), "El ID no es un UUID válido");
            }

            #[test]
            fn tets_is_v4() {
                let id = Selectora::new(POSICION_MAXIMA).get_id();
                let parsed_id = Uuid::parse_str(&id.to_string());
                assert_eq!(
                    parsed_id.unwrap().get_version(),
                    Some(uuid::Version::Random),
                    "La ID no es un UUID v4"
                );
            }
        }
    }

    mod set_posicion {
        use super::*;
        use hardware::errors::SelectoraError;

        #[test]
        fn test_cambio_posicion() {
            let mut selectora = Selectora::new(POSICION_MAXIMA);

            assert_eq!(selectora.get_posicion(), 0, "La posición inicial no es 0");

            selectora
                .set_posicion(POSICION_MAXIMA)
                .expect("Ha intentado establecer una posición fuera del rango permitido");

            assert_eq!(
                selectora.get_posicion(),
                POSICION_MAXIMA,
                "La posición no es {}",
                POSICION_MAXIMA
            )
        }

        #[test]
        fn test_no_pasa_el_maximo() {
            let mut selectora = Selectora::new(POSICION_MAXIMA);

            let resultado = selectora.set_posicion(POSICION_MAXIMA + 1);

            assert_eq!(
                resultado,
                Err(SelectoraError::FueraDeRango),
                "El error no es fuera de rango"
            )
        }
    }
}
