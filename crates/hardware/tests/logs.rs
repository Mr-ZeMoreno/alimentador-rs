use hardware::logs::Print;

#[cfg(test)]
mod avisos {
    // Si no hay comentarios en ignore comentar este modulo
    #[test]
    #[ignore = "Por Favor Leer en TODO.md los test de traits"]
    fn archivo_test() {}

    // #[test]
    // #[ignore = "Por Favor Leer en TODO.md la implementación de traits"]
    // fn archivo_implementacion() {}

    // #[test]
    // #[ignore = "Por Favor Leer en TODO.md los errores de traits"]
    // fn archivo_errores() {}
}

#[cfg(test)]
mod logs {
    use super::*;

    mod soplador {
        use hardware::soplador::Soplador;

        use super::*;

        #[test]
        fn test_estructura() {
            let soplador = Soplador::new();

            let log = soplador.print();
            assert!(log.contains("[Soplador]"));
            assert!(log.contains(&soplador.get_id().to_string()));
            assert!(log.contains(&format!("{}%", &soplador.get_potencia().to_string())));
            assert!(log.contains("Apagado")); // Estado base
        }

        #[test]
        fn test_cambio_de_estado() {
            let mut soplador = Soplador::new();

            let log = soplador.print().to_lowercase();

            assert!(log.contains("apagado"), "No contiene el estado apagado");

            soplador.set_estado(true);

            let log = soplador.print().to_lowercase();

            assert!(log.contains("encendido"), "No contiene el estado encendido");
        }

        #[test]
        fn test_cambio_de_potencia() {
            let mut soplador = Soplador::new();

            let log = soplador.print().to_lowercase();

            assert!(log.contains("0%"), "No contiene la potencia en porcentaje");

            soplador
                .set_potencia(100)
                .expect("Ha puesto una potencia sobre el rango permitido");

            let log = soplador.print().to_lowercase();

            assert!(log.contains("100%"), "No ha cambiado la potencia");
        }
    }

    mod dosificador {
        use hardware::dosificador::Dosificador;

        const ENTREGA: u32 = 10;

        use super::*;

        #[test]
        fn test_estructura() {
            let dosificador = Dosificador::new(ENTREGA);

            let log = dosificador.print();

            assert!(log.contains("[Dosificador]"));
            assert!(log.contains(&dosificador.get_id().to_string()));
            assert!(log.contains(&format!("{}", &dosificador.get_entrega().to_string())));
            assert!(log.contains("Apagado")); // Estado base
        }

        #[test]
        fn test_cambio_de_estado() {
            let mut dosificador = Dosificador::new(ENTREGA);

            let log = dosificador.print().to_lowercase();

            assert!(log.contains("apagado"), "No contiene el estado apagado");

            dosificador.set_estado(true);

            let log = dosificador.print().to_lowercase();

            assert!(log.contains("encendido"), "No contiene el estado encendido");
        }
    }

    mod silo {
        use super::*;
        use hardware::silo::Silo;

        const CAPACIDAD_SILOS: u32 = 24000;

        mod estructura {
            use super::*;

            #[test]
            fn test_contiene_silo() {
                let silo = Silo::new(CAPACIDAD_SILOS);

                let log = silo.print();

                assert!(log.contains("[Silo]"), "No contiene la palabra silo");
            }

            #[test]
            fn test_contiene_id() {
                let silo = Silo::new(CAPACIDAD_SILOS);

                let log = silo.print();

                assert!(
                    log.contains(&silo.get_id().to_string()),
                    "No contiene la id"
                );
            }

            #[test]
            fn test_contiene_actual() {
                let silo = Silo::new(CAPACIDAD_SILOS);

                let log = silo.print();

                assert!(
                    log.contains(&format!("Actual: {}kg", silo.get_alimento().to_string())),
                    "No contiene el alimento del silo"
                );
            }

            #[test]
            fn test_contiene_historico() {
                let silo = Silo::new(CAPACIDAD_SILOS);

                let log = silo.print();

                assert!(
                    log.contains(&format!(
                        "Historico: {}kg",
                        silo.get_historico().to_string()
                    )),
                    "No contien el alimento historico"
                )
            }
        }

        mod actualiza_con_setter {
            use super::*;

            fn assert_actualizable(silo: &mut Silo, tag: &str, set: u32, assert: u32) {
                silo.set_alimento(set)
                    .expect("Ha intentado ingresar una cantidad superior a la capacidad del silo");
                assert!(silo.print().contains(&format!("{}: {}kg", tag, assert)));
            }

            #[test]
            fn test_alimento() {
                const TAG: &str = "Actual";

                let mut silo = Silo::new(CAPACIDAD_SILOS);
                assert!(silo.print().contains(&format!("{}: {}kg", TAG, 0)));

                assert_actualizable(&mut silo, &TAG, CAPACIDAD_SILOS, CAPACIDAD_SILOS);
            }

            #[test]
            fn test_historico() {
                const TAG: &str = "Historico";

                let mut silo = Silo::new(CAPACIDAD_SILOS);
                assert!(silo.print().contains(&format!("{}: 0kg", &TAG)));

                assert_actualizable(&mut silo, &TAG, CAPACIDAD_SILOS, CAPACIDAD_SILOS);
                assert_actualizable(&mut silo, &TAG, 0, CAPACIDAD_SILOS);
                assert_actualizable(&mut silo, &TAG, CAPACIDAD_SILOS, 2 * CAPACIDAD_SILOS);
            }
        }
    }
}
