use super::padrao::RegraNegocio;
use crate::models::empreendimento::Empreendimento;

#[derive(Debug)]
pub struct RegraAreaLazerAlpha;

impl RegraNegocio for RegraAreaLazerAlpha {
    fn validar(&self, e: &Empreendimento) -> Option<String> {
        if e.construtora != "Alpha" {
            return None;
        }
        match e.area_de_lazer {
            Some(area) if area >= 0.1 * e.area_do_terreno => None,
            Some(area) => Some(format!(
                "Área de lazer insuficiente para Alpha: {:.2} m².",
                area
            )),
            None => Some("Faltando área de lazer para Alpha.".to_string()),
        }
    }
    fn nome(&self) -> &'static str {
        "RegraAreaLazerAlpha"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::empreendimento::Empreendimento;

    #[test]
    fn test_construtora_nao_alpha_nao_aplica() {
        let regra = RegraAreaLazerAlpha;
        let empreendimento = Empreendimento {
            construtora: "Beta".to_string(),
            numero_de_torres: 2,
            area_do_terreno: 1000.0,
            area_da_torre: 100.0,
            altura_da_torre: 10.0,
            area_de_lazer: Some(50.0),
            cidade: "São Paulo".to_string(),
        };

        assert_eq!(regra.validar(&empreendimento), None);
    }

    #[test]
    fn test_alpha_area_de_lazer_none() {
        let regra = RegraAreaLazerAlpha;
        let empreendimento = Empreendimento {
            construtora: "Alpha".to_string(),
            numero_de_torres: 2,
            area_do_terreno: 1000.0,
            area_da_torre: 100.0,
            altura_da_torre: 10.0,
            area_de_lazer: None,
            cidade: "São Paulo".to_string(),
        };

        assert_eq!(
            regra.validar(&empreendimento),
            Some("Faltando área de lazer para Alpha.".to_string())
        );
    }

    #[test]
    fn test_alpha_area_no_limite_valida() {
        let regra = RegraAreaLazerAlpha;
        let empreendimento = Empreendimento {
            construtora: "Alpha".to_string(),
            numero_de_torres: 2,
            area_do_terreno: 1000.0,
            area_da_torre: 100.0,
            altura_da_torre: 10.0,
            area_de_lazer: Some(100.0),
            cidade: "São Paulo".to_string(),
        };

        assert_eq!(regra.validar(&empreendimento), None);
    }

    #[test]
    fn test_alpha_area_insuficiente() {
        let regra = RegraAreaLazerAlpha;
        let empreendimento = Empreendimento {
            construtora: "Alpha".to_string(),
            numero_de_torres: 2,
            area_do_terreno: 1000.0,
            area_da_torre: 100.0,
            altura_da_torre: 10.0,
            area_de_lazer: Some(80.0),
            cidade: "São Paulo".to_string(),
        };

        let resultado = regra.validar(&empreendimento);
        assert!(resultado.is_some());
        assert!(
            resultado
                .unwrap()
                .contains("Área de lazer insuficiente para Alpha: 80.00 m²")
        );
    }

    #[test]
    fn test_alpha_area_suficiente() {
        let regra = RegraAreaLazerAlpha;
        let empreendimento = Empreendimento {
            construtora: "Alpha".to_string(),
            numero_de_torres: 2,
            area_do_terreno: 1000.0,
            area_da_torre: 100.0,
            altura_da_torre: 10.0,
            area_de_lazer: Some(150.0),
            cidade: "São Paulo".to_string(),
        };

        assert_eq!(regra.validar(&empreendimento), None);
    }
}
