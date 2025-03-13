# Tests
## Soplador
- [ ] test_creacion_soplador
    - **Estados iniciales**
    - [ ] El *estado* debe ser False
    - [ ] La *potencia* debe ser 0
    - [ ] La *id* debe ser un uuid_v4 **unica**
- [ ] test_set_estado
    - [ ] Solo debe aceptar booleanos
    - [ ] Debe cambiar el estado
- [ ] test_set_potencia
    - [ ] Solo debe aceptar u32
    - [ ] Debe cambiar la potencia
    - [ ] Arroja error en valores fuera del rando [0, 100]
- [ ] test_get_estado
    - [ ] Debe obtener un booleano
    - [ ] Debe actualizarse
- [ ] test_get_potencia
    - [ ] Debe obtener un valor entre [0, 100]
    - [ ] Debe actualizarse
- [ ] test_get_id
    - [ ] Debe obtener un uuid v4
    - [ ] Debe ser único
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