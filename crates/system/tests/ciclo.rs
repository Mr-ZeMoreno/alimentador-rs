use system::ciclo::Ciclo;

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
mod ciclo {
    use std::any::type_name_of_val;

    use super::*;
    use system::errors::CicloError;

    fn assert_return_type<T>(_val: &T) {}

    mod new {
        use system::ciclo::{DURACION_MIN, ESPERA_MIN, PULSOS_MIN};

        use super::*;

        #[test]
        fn test_valores_iniciales() {
            let ciclo = Ciclo::new();

            assert_eq!(
                ciclo.get_pulsos(),
                PULSOS_MIN,
                "Los pulsos iniciales no es: {}",
                PULSOS_MIN
            );
            assert_eq!(
                ciclo.get_pulso_espera(),
                ESPERA_MIN,
                "La espera inicial no es: {}",
                ESPERA_MIN
            );
            assert_eq!(
                ciclo.get_pulso_duracion(),
                DURACION_MIN,
                "La duración inicial no es: {}",
                DURACION_MIN
            );
        }
        mod id {
            use super::Ciclo;
            use uuid::Uuid;

            /// Prueba que la id sea unica
            #[test]
            fn test_unicidad() {
                // Verifica unicidad de id
                let id1 = Ciclo::new().get_id();
                let id2 = Ciclo::new().get_id();
                assert_ne!(id1, id2, "Las ID no son únicas");
            }

            /// Prueba que la id sea un UUID válido
            #[test]
            fn test_validez() {
                let id = Ciclo::new().get_id();
                let parsed_id = Uuid::parse_str(&id.to_string());
                assert!(parsed_id.is_ok(), "El ID no es un UUID válido");
            }

            #[test]
            fn tets_is_v4() {
                let id = Ciclo::new().get_id();
                let parsed_id = Uuid::parse_str(&id.to_string());
                assert_eq!(
                    parsed_id.unwrap().get_version(),
                    Some(uuid::Version::Random),
                    "La ID no es un UUID v4"
                );
            }
        }
    }

    mod set_pulso {

        use system::ciclo::{PULSOS_MAX, PULSOS_MIN};

        use super::*;

        #[test]
        fn test_establece_pulsos() {
            let mut ciclo = Ciclo::new();

            assert_eq!(
                ciclo.get_pulsos(),
                PULSOS_MIN,
                "Los pulsos no son: {}",
                PULSOS_MIN
            );

            ciclo
                .set_pulsos(PULSOS_MAX)
                .expect("Ha intentado establecer una cantidad de pulsos fuera del rango");
            assert_eq!(
                ciclo.get_pulsos(),
                PULSOS_MAX,
                "Los pulsos no son: {}",
                PULSOS_MAX
            );
        }

        #[test]
        fn test_sobre_el_rango() {
            let mut ciclo = Ciclo::new();

            assert_eq!(
                ciclo.get_pulsos(),
                PULSOS_MIN,
                "Los pulsos no son: {}",
                PULSOS_MIN
            );

            let response = ciclo.set_pulsos(PULSOS_MAX + 1);

            assert_eq!(
                response,
                Err(CicloError::CantidadFueraDeRango),
                "Los errores no coinciden"
            );
        }
    }

    mod set_pulso_duracion {
        use system::ciclo::{DURACION_MAX, DURACION_MIN};

        use super::*;

        #[test]
        fn test_establece_duracion() {
            let mut ciclo = Ciclo::new();

            assert_eq!(
                ciclo.get_pulso_duracion(),
                DURACION_MIN,
                "La duracion no es: {}",
                DURACION_MIN
            );

            ciclo
                .set_pulso_duracion(DURACION_MAX)
                .expect("Ha intentado establecer una duracion fuera del rango");
            assert_eq!(
                ciclo.get_pulso_duracion(),
                DURACION_MAX,
                "La duracion no es: {}",
                DURACION_MAX
            );
        }

        #[test]
        fn test_bajo_el_rango() {
            let mut ciclo = Ciclo::new();

            assert_eq!(
                ciclo.get_pulso_duracion(),
                DURACION_MIN,
                "La duracion no es: {}",
                DURACION_MIN
            );

            let response = ciclo.set_pulso_duracion(DURACION_MIN - 1);

            assert_eq!(
                response,
                Err(CicloError::DuracionFueraDeRango),
                "Los errores no coinciden"
            );
        }

        #[test]
        fn test_sobre_el_rango() {
            let mut ciclo = Ciclo::new();

            assert_eq!(
                ciclo.get_pulso_duracion(),
                DURACION_MIN,
                "La duración no es: {}",
                DURACION_MIN
            );

            let response = ciclo.set_pulso_duracion(DURACION_MAX + 1);

            assert_eq!(
                response,
                Err(CicloError::DuracionFueraDeRango),
                "Los errores no coinciden"
            );
        }
    }

    mod set_pulso_espera {
        use system::ciclo::{ESPERA_MAX, ESPERA_MIN};

        use super::*;

        #[test]
        fn test_establece_duracion() {
            let mut ciclo = Ciclo::new();

            assert_eq!(ciclo.get_pulso_espera(), 1000, "Los pulsos no son mil");

            ciclo
                .set_pulso_espera(10000)
                .expect("Ha intentado establecer una espera fuera del rango");
            assert_eq!(ciclo.get_pulso_espera(), 10000, "Los pulsos no son 10000");
        }

        #[test]
        fn test_bajo_el_rango() {
            let mut ciclo = Ciclo::new();

            assert_eq!(ciclo.get_pulso_espera(), 1000, "La espera no es mil");

            let response = ciclo.set_pulso_espera(999);

            assert_eq!(
                response,
                Err(CicloError::EsperaFueraDeRango),
                "Los errores no coinciden"
            );
        }

        #[test]
        fn test_sobre_el_rango() {
            let mut ciclo = Ciclo::new();

            assert_eq!(ciclo.get_pulso_espera(), ESPERA_MIN, "La espera no es mil");

            let response = ciclo.set_pulso_espera(ESPERA_MAX + 1);

            assert_eq!(
                response,
                Err(CicloError::EsperaFueraDeRango),
                "Los errores no coinciden"
            );
        }
    }

    #[test]
    fn test_get_pulso() {
        let b = Ciclo::new().get_pulsos();

        assert!(
            type_name_of_val(&b).contains("u32"),
            "El pulso no es de tipo u32"
        );
    }

    #[test]
    fn test_get_pulso_espera() {
        let b = Ciclo::new().get_pulso_espera();

        assert!(
            type_name_of_val(&b).contains("u32"),
            "La espera no es de tipo u32"
        );
    }

    #[test]
    fn test_get_pulso_duracion() {
        let b = Ciclo::new().get_pulso_duracion();

        assert!(
            type_name_of_val(&b).contains("u32"),
            "La duración no es de tipo u32"
        );
    }

    mod get_all {
        use super::{assert_return_type, Ciclo};
        use system::ciclo::CicloData;
        use types::rango::RangoData;

        mod tipos {
            use super::*;

            #[test]
            fn test_pulso() {
                let ciclo = Ciclo::new();

                let b = ciclo.get_all();

                assert_return_type::<RangoData>(&b.pulsos);
            }

            #[test]
            fn test_duracion() {
                let ciclo = Ciclo::new();

                let b = ciclo.get_all();

                assert_return_type::<RangoData>(&b.duracion);
            }

            #[test]
            fn test_espera() {
                let ciclo = Ciclo::new();

                let b = ciclo.get_all();

                assert_return_type::<RangoData>(&b.espera);
            }

            #[test]
            fn test_all() {
                let b = Ciclo::new().get_all();

                assert_return_type::<CicloData>(&b);
            }
        }

        mod tipos_limites {
            use super::*;
            #[test]
            fn test_pulso() {
                let ciclo = Ciclo::new();

                let b = ciclo.get_all();

                assert_return_type::<u32>(&b.pulsos.min);
                assert_return_type::<u32>(&b.pulsos.max);
            }

            #[test]
            fn test_duracion() {
                let ciclo = Ciclo::new();

                let b = ciclo.get_all();

                assert_return_type::<u32>(&b.duracion.min);
                assert_return_type::<u32>(&b.duracion.max);
            }

            #[test]
            fn test_espera_tipo() {
                let ciclo = Ciclo::new();

                let b = ciclo.get_all();

                assert_return_type::<u32>(&b.espera.min);
                assert_return_type::<u32>(&b.espera.max);
            }
        }

        mod valores {
            use system::ciclo::{
                DURACION_MAX, DURACION_MIN, ESPERA_MAX, ESPERA_MIN, PULSOS_MAX, PULSOS_MIN,
            };

            use super::*;
            #[test]
            fn test_pulso() {
                let ciclo = Ciclo::new();

                let b = ciclo.get_all();

                assert_eq!(
                    b.pulsos.min, PULSOS_MIN,
                    "El limite inferior de los pulsos no coincide"
                );
                assert_eq!(
                    b.pulsos.max, PULSOS_MAX,
                    "El limite superior de los pulsos no coincide"
                );
            }

            #[test]
            fn test_duracion() {
                let ciclo = Ciclo::new();

                let b = ciclo.get_all();

                assert_eq!(
                    b.duracion.min, DURACION_MIN,
                    "El limite inferior de la duración no coincide"
                );
                assert_eq!(
                    b.duracion.max, DURACION_MAX,
                    "El limite superior de la duración no coincide"
                );
            }

            #[test]
            fn test_espera_tipo() {
                let ciclo = Ciclo::new();

                let b = ciclo.get_all();

                assert_eq!(
                    b.espera.min, ESPERA_MIN,
                    "El limite inferior de la espera no coincide"
                );
                assert_eq!(
                    b.espera.max, ESPERA_MAX,
                    "El limite superior de la espera no coincide"
                );
            }
        }
    }
}
