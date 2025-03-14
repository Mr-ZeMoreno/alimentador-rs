# Tests
## Soplador
- [x] test_creacion_soplador
    - **Estados iniciales**
    - [x] El *estado* debe ser False
    - [x] La *potencia* debe ser 0
    - [x] La *id* debe ser un uuid_v4 **unica**
- [x] test_set_estado
    - [x] Solo debe aceptar booleanos
    - [x] Debe cambiar el estado
- [x] test_set_potencia
    - [x] Solo debe aceptar u32
    - [x] Debe cambiar la potencia
    - [x] Arroja error en valores fuera del rando [0, 100]
- [x] test_get_estado
    - [x] Debe obtener un booleano
    - [x] Debe actualizarse
- [ ] test_get_potencia
    - [x] Debe obtener un valor entre [0, 100]
    - [x] Debe actualizarse
- [ ] test_get_id
    - [x] Debe obtener un uuid v4
    - [x] Debe ser único
- [ ] test_chain_methods
    - [ ] Debe poder hacerse chaining

- [ ] ** Documentar para `cargo doc`**

## Traits
- **Prints**
- [ ] Soplador
    - [ ] Debe ser encendido o apago en función del estado
    - [ ] Debe mostrar la estructura [Soplador][id][potencia]: -- [estado] --
- [ ] Dosificador
    - [ ] Debe ser encendido o apago en función del estado
    - [ ] Debe mostrar la estructura [Dosificador][id][entrega]: -- [estado] --
- **PrintsConArg**
- [ ] Silo
    - [ ] Debe mostrar la estructura [Silo][id][Historico][Alimento]: -- [entregado] --