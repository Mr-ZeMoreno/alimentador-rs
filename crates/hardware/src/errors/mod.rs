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

#[derive(Debug)]
pub enum SopladorError {
    FueraDeRango,
    ErrorInesperado,
}
impl PartialEq for SopladorError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SopladorError::FueraDeRango, SopladorError::FueraDeRango) => true,
            (SopladorError::ErrorInesperado, SopladorError::ErrorInesperado) => true,
            _ => false,
        }
    }
}
#[derive(Debug)]
pub enum SelectoraError {
    FueraDeRango,
    ErrorInesperado,
}
impl PartialEq for SelectoraError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SelectoraError::FueraDeRango, SelectoraError::FueraDeRango) => true,
            (SelectoraError::ErrorInesperado, SelectoraError::ErrorInesperado) => true,
            _ => false,
        }
    }
}
