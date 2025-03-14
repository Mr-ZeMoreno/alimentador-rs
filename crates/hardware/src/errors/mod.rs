#[derive(Debug)]
pub enum SiloError {
    SinAlimento,
    FueraDeRango,
    ErrorInesperado,
}
impl PartialEq for SiloError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SiloError::FueraDeRango, SiloError::FueraDeRango) => true,
            (SiloError::SinAlimento, SiloError::SinAlimento) => true,
            (SiloError::ErrorInesperado, SiloError::ErrorInesperado) => true,
            _ => false,
        }
    }
}
