#[derive(PartialEq, Debug)]
pub enum CicloError {
    CantidadFueraDeRango,
    DuracionFueraDeRango,
    EsperaFueraDeRango,
}

#[derive(PartialEq, Debug)]
pub enum RacionError {
    EsperaFueraDeRango,
}
