use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Empreendimento {
    #[serde(alias = "construtora")]
    pub construtora: String,

    #[serde(alias = "cidade")]
    pub cidade: String,

    #[serde(alias = "area_do_terreno")]
    pub area_do_terreno: f64,

    #[serde(alias = "numero_de_torres")]
    pub numero_de_torres: u32,

    #[serde(alias = "altura_da_torre")]
    pub altura_da_torre: f64,

    #[serde(alias = "area_da_torre")]
    pub area_da_torre: f64,

    #[serde(alias = "area_de_lazer")]
    pub area_de_lazer: Option<f64>,
}
