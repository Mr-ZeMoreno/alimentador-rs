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
mod racion {
    use system::racion::Racion;
    use system::{ciclo::Ciclo, racion::ESPERA_MAX, racion::ESPERA_MIN};

    // Función auxiliar para crear un ciclo con valores predeterminados
    fn crear_ciclo(pulsos: u32, duracion: u32, espera: u32) -> Ciclo {
        let mut ciclo = Ciclo::new();
        ciclo
            .set_pulsos(pulsos)
            .expect("Ha intentado insertar pulsos fuera del rango permitido");
        ciclo
            .set_pulso_duracion(duracion)
            .expect("Ha intentado establecer una duracion fuera del rango permitido");
        ciclo
            .set_pulso_espera(espera)
            .expect("Ha intentado establecer una espera fuera del rango permitido");
        ciclo
    }

    mod new {
        use super::*;

        #[test]
        fn test_valores_iniciales() {
            let c1 = crear_ciclo(50, 5000, 8000);
            let c2 = crear_ciclo(20, 3000, 4000);

            let vector = vec![&c1, &c2, &c1, &c1, &c2];
            let vector2 = vector.clone();

            let racion = Racion::new(vector);

            assert_eq!(
                racion.get_ciclo_espera(),
                ESPERA_MIN,
                "La espera inicial no es {}",
                ESPERA_MIN
            );
            assert_eq!(
                *racion.get_ciclos(),
                *vector2,
                "El ciclo no es {:#?}",
                vector2
            );
        }

        mod id {
            use super::{crear_ciclo, Racion};
            use uuid::Uuid;

            /// Prueba que la id sea unica
            #[test]
            fn test_unicidad() {
                let c1 = crear_ciclo(50, 5000, 8000);
                let c2 = crear_ciclo(20, 3000, 4000);

                let v1 = vec![&c1, &c2, &c1, &c1, &c2];
                let v2 = v1.clone();

                // Verifica unicidad de id
                let id1 = Racion::new(v1).get_id();
                let id2 = Racion::new(v2).get_id();
                assert_ne!(id1, id2, "Las ID no son únicas");
            }

            /// Prueba que la id sea un UUID válido
            #[test]
            fn test_validez() {
                let c1 = crear_ciclo(50, 5000, 8000);
                let c2 = crear_ciclo(20, 3000, 4000);

                let v1 = vec![&c1, &c2, &c1, &c1, &c2];

                // Verifica unicidad de id
                let id = Racion::new(v1).get_id();
                let parsed_id = Uuid::parse_str(&id.to_string());
                assert!(parsed_id.is_ok(), "El ID no es un UUID válido");
            }

            #[test]
            fn tets_is_v4() {
                let c1 = crear_ciclo(50, 5000, 8000);
                let c2 = crear_ciclo(20, 3000, 4000);

                let v1 = vec![&c1, &c2, &c1, &c1, &c2];

                // Verifica unicidad de id
                let id = Racion::new(v1).get_id();
                let parsed_id = Uuid::parse_str(&id.to_string());
                assert_eq!(
                    parsed_id.unwrap().get_version(),
                    Some(uuid::Version::Random),
                    "La ID no es un UUID v4"
                );
            }
        }
    }

    mod set_ciclo_espera {
        use super::{crear_ciclo, Racion, ESPERA_MAX, ESPERA_MIN};

        #[test]
        fn test_establece_valor() {
            let c1 = crear_ciclo(50, 5000, 8000);
            let c2 = crear_ciclo(20, 3000, 4000);

            let vector = vec![&c1, &c2, &c1, &c1, &c2];

            // Crear la ración con los ciclos predefinidos
            let mut racion = Racion::new(vector);

            assert_eq!(
                racion.get_ciclo_espera(),
                ESPERA_MIN,
                "La espera no es {}",
                ESPERA_MIN
            );

            racion
                .set_ciclo_espera(ESPERA_MIN + 1)
                .expect("Ha intentado establecer una espera fuera de rango");

            assert_eq!(
                racion.get_ciclo_espera(),
                ESPERA_MIN + 1,
                "La espera no es {}",
                ESPERA_MIN + 1
            );
        }

        #[test]
        fn test_bajo_el_rango() {
            let c1 = crear_ciclo(50, 5000, 8000);
            let c2 = crear_ciclo(20, 3000, 4000);

            let vector = vec![&c1, &c2, &c1, &c1, &c2];

            // Crear la ración con los ciclos predefinidos
            let mut racion = Racion::new(vector);

            let r = racion.set_ciclo_espera(ESPERA_MIN - 1);

            assert_eq!(
                r,
                Err(system::errors::RacionError::EsperaFueraDeRango),
                "Los errores no coinciden"
            )
        }

        #[test]
        fn test_sobre_el_rango() {
            let c1 = crear_ciclo(50, 5000, 8000);
            let c2 = crear_ciclo(20, 3000, 4000);

            let vector = vec![&c1, &c2, &c1, &c1, &c2];

            // Crear la ración con los ciclos predefinidos
            let mut racion = Racion::new(vector);
            let r = racion.set_ciclo_espera(ESPERA_MAX + 1);

            assert_eq!(
                r,
                Err(system::errors::RacionError::EsperaFueraDeRango),
                "Los errores no coinciden"
            )
        }
    }
}
