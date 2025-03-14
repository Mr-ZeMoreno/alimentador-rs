use hardware::errors::SiloError;
use hardware::silo::Silo;

const CAPACIDAD_SILOS: u32 = 24000;
const GRAMOS_PULSO: u32 = 25;

#[cfg(test)]
mod silo {
    use super::*;

    mod new {
        use super::{Silo, CAPACIDAD_SILOS};
        /// Prueba la creación de un nuevo dosificador y verifica sus valores iniciales.
        #[test]
        fn test_alimento_inicial() {
            let silo = Silo::new(CAPACIDAD_SILOS);
            assert_eq!(silo.get_alimento(), 0);
        }

        #[test]
        fn test_historico_inicial() {
            let silo = Silo::new(CAPACIDAD_SILOS);

            assert_eq!(silo.get_historico(), 0);
        }

        mod id {
            use super::{Silo, CAPACIDAD_SILOS};
            use uuid::Uuid;

            /// Prueba que la id sea unica
            #[test]
            fn test_unicidad() {
                let silo = Silo::new(CAPACIDAD_SILOS);
                // Verifica unicidad de id
                let id1 = silo.get_id();
                let id2 = Silo::new(CAPACIDAD_SILOS).get_id();
                assert_ne!(id1, id2, "Las ID no son únicas");
            }

            /// Prueba que la id sea un UUID válido
            #[test]
            fn test_validez() {
                let silo = Silo::new(CAPACIDAD_SILOS);
                let id = silo.get_id();
                let parsed_id = Uuid::parse_str(&id.to_string());
                assert!(parsed_id.is_ok(), "El ID no es un UUID válido");
            }

            #[test]
            fn tets_is_v4() {
                let silo = Silo::new(CAPACIDAD_SILOS);
                let id = silo.get_id();
                let parsed_id = Uuid::parse_str(&id.to_string());
                assert_eq!(
                    parsed_id.unwrap().get_version(),
                    Some(uuid::Version::Random),
                    "La ID no es un UUID v4"
                );
            }
        }
    }

    /// Prueba el setter `set_entrega` para modificar la capacidad de entrega del dosificador.
    #[test]
    fn test_aumenta_alimento() {
        let mut silo = Silo::new(CAPACIDAD_SILOS);

        // Comprobamos que set alimento funciona bien
        silo.set_alimento(10)
            .expect("[test_aumenta_alimento]: No se ha podido actualizar [Linea 70]");
        assert_eq!(silo.get_alimento(), 10);
    }

    mod set_historico {
        use super::{Silo, CAPACIDAD_SILOS};
        #[test]
        fn test_aumenta() {
            let mut silo = Silo::new(CAPACIDAD_SILOS);
            // Comprobamos que el historico aumenta
            silo.set_alimento(11)
                .expect("[test_aumenta]: No se ha actualizado el alimento");
            assert_eq!(silo.get_historico(), 11);
        }
        #[test]
        fn test_no_disminuye() {
            let mut silo = Silo::new(CAPACIDAD_SILOS);

            assert_eq!(silo.get_historico(), 0); // Valor inicial

            silo.set_alimento(11)
                .expect("[test_no_disminuye]: No se ha actualizado el alimento [11]");
            assert_eq!(silo.get_historico(), 11);
            // Comprobamos que el historico no disminuye
            silo.set_alimento(8)
                .expect("[test_no_disminuye]: No se ha actualizado el alimento [8]");

            assert_ne!(silo.get_historico(), 8);
        }

        #[test]
        fn test_aumenta_varias_veces() {
            let mut silo = Silo::new(CAPACIDAD_SILOS);

            for i in 1..=100 {
                silo.set_alimento(i).expect(
                    "[test_aumenta_varias_veces]: No se ha podido actualizar alimento [Linea 105]",
                );
                assert_eq!(silo.get_historico(), i);
            }

            silo.set_alimento(0)
                .expect("[test_aumenta_varias_veces]: No se ha podido actualiza [Linea 111]");
            assert_eq!(silo.get_historico(), 100);

            for i in 1..100 {
                silo.set_alimento(i).expect(
                    "[test_aumenta_varias_veces]: No se ha podido actualizar alimento [Linea 116]",
                );
                assert_eq!(silo.get_historico(), i + 100);
            }
        }
    }

    /// Verifica que el valor de `entrega` se obtiene correctamente.
    #[test]
    fn test_get_alimento() {
        let silo = Silo::new(CAPACIDAD_SILOS);
        assert_eq!(silo.get_alimento(), 0); // Valor inicial
    }

    // Revisar por que no pasó los tests
    mod entregar_pulso {
        use super::{Silo, SiloError, CAPACIDAD_SILOS, GRAMOS_PULSO};

        /// Verificar que los pulsos se entregan
        #[test]
        fn test_actualiza_alimento() {
            let mut silo = Silo::new(CAPACIDAD_SILOS);

            silo.set_alimento(CAPACIDAD_SILOS).expect(
                "[test_actualiza_alimento]: No se ha podido actualizar el alimento [Linea 141]",
            );

            silo.entregar_pulso(GRAMOS_PULSO)
                .expect("[test_actualiza_alimento]: No se ha podido entregar pulso [Linea 145]");
            assert_eq!(silo.get_alimento(), CAPACIDAD_SILOS - GRAMOS_PULSO);

            silo.entregar_pulso(CAPACIDAD_SILOS - GRAMOS_PULSO)
                .expect("[test_actualiza_alimento]:  No se ha podido entregar pulso [Linea 149]");
            assert_eq!(silo.get_alimento(), 0);
        }

        #[test]
        fn test_no_actualiza_historico() {
            let mut silo = Silo::new(CAPACIDAD_SILOS);

            silo.set_alimento(CAPACIDAD_SILOS)
                .expect("Ha sobrepasado la capacidad");
            let historico = silo.get_historico();

            silo.entregar_pulso(GRAMOS_PULSO).expect(&format!(
                "El silo es incapaz de entregar {} gramos",
                GRAMOS_PULSO
            ));
            assert_eq!(
                silo.get_historico(),
                historico,
                "El valor de historico ha cambiado"
            );
        }

        #[test]
        fn test_no_entrega_vacio() {
            let mut silo = Silo::new(CAPACIDAD_SILOS);

            let resultado = silo.entregar_pulso(GRAMOS_PULSO);

            // Compara el resultado con el error esperado dentro de SiloError
            assert_eq!(resultado, Err(SiloError::SinAlimento));
        }
    }
}
