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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_deserialize_kebab_case() {
        let json = r#"{
            "construtora": "Alpha",
            "cidade": "São Paulo",
            "area-do-terreno": 1000.0,
            "numero-de-torres": 2,
            "altura-da-torre": 25.0,
            "area-da-torre": 300.0,
            "area-de-lazer": 150.0
        }"#;

        let empreendimento: Empreendimento = serde_json::from_str(json).unwrap();
        
        assert_eq!(empreendimento.construtora, "Alpha");
        assert_eq!(empreendimento.cidade, "São Paulo");
        assert_eq!(empreendimento.area_do_terreno, 1000.0);
        assert_eq!(empreendimento.numero_de_torres, 2);
        assert_eq!(empreendimento.altura_da_torre, 25.0);
        assert_eq!(empreendimento.area_da_torre, 300.0);
        assert_eq!(empreendimento.area_de_lazer, Some(150.0));
    }

    #[test]
    fn test_deserialize_snake_case() {
        let json = r#"{
            "construtora": "Beta",
            "cidade": "Rio de Janeiro",
            "area_do_terreno": 800.0,
            "numero_de_torres": 3,
            "altura_da_torre": 30.0,
            "area_da_torre": 250.0,
            "area_de_lazer": 100.0
        }"#;

        let empreendimento: Empreendimento = serde_json::from_str(json).unwrap();
        
        assert_eq!(empreendimento.construtora, "Beta");
        assert_eq!(empreendimento.cidade, "Rio de Janeiro");
        assert_eq!(empreendimento.area_do_terreno, 800.0);
        assert_eq!(empreendimento.numero_de_torres, 3);
        assert_eq!(empreendimento.altura_da_torre, 30.0);
        assert_eq!(empreendimento.area_da_torre, 250.0);
        assert_eq!(empreendimento.area_de_lazer, Some(100.0));
    }

    #[test]
    fn test_deserialize_mixed_case() {
        let json = r#"{
            "construtora": "Gamma",
            "cidade": "Brasília",
            "area-do-terreno": 1200.0,
            "numero_de_torres": 4,
            "altura-da-torre": 35.0,
            "area_da_torre": 400.0,
            "area-de-lazer": 200.0
        }"#;

        let empreendimento: Empreendimento = serde_json::from_str(json).unwrap();
        
        assert_eq!(empreendimento.construtora, "Gamma");
        assert_eq!(empreendimento.cidade, "Brasília");
        assert_eq!(empreendimento.area_do_terreno, 1200.0);
        assert_eq!(empreendimento.numero_de_torres, 4);
        assert_eq!(empreendimento.altura_da_torre, 35.0);
        assert_eq!(empreendimento.area_da_torre, 400.0);
        assert_eq!(empreendimento.area_de_lazer, Some(200.0));
    }

    #[test]
    fn test_deserialize_without_area_de_lazer() {
        let json = r#"{
            "construtora": "Delta",
            "cidade": "Salvador",
            "area-do-terreno": 600.0,
            "numero-de-torres": 1,
            "altura-da-torre": 20.0,
            "area-da-torre": 500.0
        }"#;

        let empreendimento: Empreendimento = serde_json::from_str(json).unwrap();
        
        assert_eq!(empreendimento.construtora, "Delta");
        assert_eq!(empreendimento.cidade, "Salvador");
        assert_eq!(empreendimento.area_do_terreno, 600.0);
        assert_eq!(empreendimento.numero_de_torres, 1);
        assert_eq!(empreendimento.altura_da_torre, 20.0);
        assert_eq!(empreendimento.area_da_torre, 500.0);
        assert_eq!(empreendimento.area_de_lazer, None);
    }
}
