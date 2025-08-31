use super::padrao::RegraNegocio;
use crate::models::empreendimento::Empreendimento;

#[derive(Debug)]
pub struct RegraAreaLazerAlpha;

impl RegraNegocio for RegraAreaLazerAlpha {
    fn validar(&self, e: &Empreendimento) -> Option<String> {
        if e.construtora != "Alpha" || e.numero_de_torres <= 1 {
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

    mod regra_area_lazer_alpha {
        use super::*;

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

            assert_eq!(regra.validar(&empreendimento), None, "Construtora não Alpha não deve aplicar regra");
        }

        #[test]
        fn test_construtora_alpha_1_torre_nao_aplica() {
            let regra = RegraAreaLazerAlpha;
            let empreendimento = Empreendimento {
                construtora: "Alpha".to_string(),
                numero_de_torres: 1,
                area_do_terreno: 1000.0,
                area_da_torre: 100.0,
                altura_da_torre: 10.0,
                area_de_lazer: Some(50.0),
                cidade: "São Paulo".to_string(),
            };

            assert_eq!(regra.validar(&empreendimento), None, "Alpha com 1 torre não deve aplicar regra");
        }

        #[test]
        fn test_construtora_alpha_2_torres_area_suficiente() {
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

            assert_eq!(regra.validar(&empreendimento), None, "Alpha com área de lazer suficiente deve ser válido");
        }

        #[test]
        fn test_construtora_alpha_2_torres_area_exatamente_no_limite() {
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

            assert_eq!(regra.validar(&empreendimento), None, "Alpha com área de lazer no limite deve ser válido");
        }

        #[test]
        fn test_construtora_alpha_2_torres_area_insuficiente() {
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
            assert!(resultado.is_some(), "Alpha com área de lazer insuficiente deve violar");
            assert!(resultado.unwrap().contains("Área de lazer insuficiente para Alpha: 80.00 m²"));
        }

        #[test]
        fn test_construtora_alpha_2_torres_area_muito_insuficiente() {
            let regra = RegraAreaLazerAlpha;
            let empreendimento = Empreendimento {
                construtora: "Alpha".to_string(),
                numero_de_torres: 2,
                area_do_terreno: 1000.0,
                area_da_torre: 100.0,
                altura_da_torre: 10.0,
                area_de_lazer: Some(10.0),
                cidade: "São Paulo".to_string(),
            };

            let resultado = regra.validar(&empreendimento);
            assert!(resultado.is_some(), "Alpha com área de lazer muito insuficiente deve violar");
            assert!(resultado.unwrap().contains("Área de lazer insuficiente para Alpha: 10.00 m²"));
        }

        #[test]
        fn test_construtora_alpha_2_torres_sem_area_de_lazer() {
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

            let resultado = regra.validar(&empreendimento);
            assert!(resultado.is_some(), "Alpha sem área de lazer deve violar");
            assert_eq!(resultado.unwrap(), "Faltando área de lazer para Alpha.");
        }

        #[test]
        fn test_construtora_alpha_muitas_torres_area_suficiente() {
            let regra = RegraAreaLazerAlpha;
            let empreendimento = Empreendimento {
                construtora: "Alpha".to_string(),
                numero_de_torres: 10,
                area_do_terreno: 1000.0,
                area_da_torre: 50.0,
                altura_da_torre: 10.0,
                area_de_lazer: Some(150.0),
                cidade: "São Paulo".to_string(),
            };

            assert_eq!(regra.validar(&empreendimento), None, "Alpha com muitas torres e área suficiente deve ser válido");
        }

        #[test]
        fn test_construtora_alpha_muitas_torres_area_insuficiente() {
            let regra = RegraAreaLazerAlpha;
            let empreendimento = Empreendimento {
                construtora: "Alpha".to_string(),
                numero_de_torres: 10,
                area_do_terreno: 1000.0,
                area_da_torre: 50.0,
                altura_da_torre: 10.0,
                area_de_lazer: Some(80.0),
                cidade: "São Paulo".to_string(),
            };

            let resultado = regra.validar(&empreendimento);
            assert!(resultado.is_some(), "Alpha com muitas torres e área insuficiente deve violar");
            assert!(resultado.unwrap().contains("Área de lazer insuficiente para Alpha: 80.00 m²"));
        }

        #[test]
        fn test_construtora_alpha_terreno_diferente() {
            let regra = RegraAreaLazerAlpha;
            let empreendimento = Empreendimento {
                construtora: "Alpha".to_string(),
                numero_de_torres: 2,
                area_do_terreno: 500.0,
                area_da_torre: 100.0,
                altura_da_torre: 10.0,
                area_de_lazer: Some(40.0),
                cidade: "São Paulo".to_string(),
            };

            let resultado = regra.validar(&empreendimento);
            assert!(resultado.is_some(), "Alpha com terreno menor deve calcular percentual correto");
            assert!(resultado.unwrap().contains("Área de lazer insuficiente para Alpha: 40.00 m²"));
        }

        #[test]
        fn test_construtora_alpha_terreno_grande() {
            let regra = RegraAreaLazerAlpha;
            let empreendimento = Empreendimento {
                construtora: "Alpha".to_string(),
                numero_de_torres: 2,
                area_do_terreno: 2000.0,
                area_da_torre: 100.0,
                altura_da_torre: 10.0,
                area_de_lazer: Some(180.0),
                cidade: "São Paulo".to_string(),
            };

            let resultado = regra.validar(&empreendimento);
            assert!(resultado.is_some(), "Alpha com terreno maior deve calcular percentual correto");
            assert!(resultado.unwrap().contains("Área de lazer insuficiente para Alpha: 180.00 m²"));
        }

        #[test]
        fn test_nome_da_regra() {
            let regra = RegraAreaLazerAlpha;
            assert_eq!(regra.nome(), "RegraAreaLazerAlpha");
        }
    }

    mod cenarios_especiais {
        use super::*;

        #[test]
        fn test_construtora_alpha_case_sensitive() {
            let regra = RegraAreaLazerAlpha;
            let empreendimento = Empreendimento {
                construtora: "alpha".to_string(),
                numero_de_torres: 2,
                area_do_terreno: 1000.0,
                area_da_torre: 100.0,
                altura_da_torre: 10.0,
                area_de_lazer: Some(50.0),
                cidade: "São Paulo".to_string(),
            };

            assert_eq!(regra.validar(&empreendimento), None, "Construtora 'alpha' não deve ser igual a 'Alpha'");
        }

        #[test]
        fn test_construtora_alpha_com_espacos() {
            let regra = RegraAreaLazerAlpha;
            let empreendimento = Empreendimento {
                construtora: " Alpha ".to_string(),
                numero_de_torres: 2,
                area_do_terreno: 1000.0,
                area_da_torre: 100.0,
                altura_da_torre: 10.0,
                area_de_lazer: Some(50.0),
                cidade: "São Paulo".to_string(),
            };

            assert_eq!(regra.validar(&empreendimento), None, "Construtora com espaços não deve ser igual a 'Alpha'");
        }

        #[test]
        fn test_construtora_alpha_zero_torres() {
            let regra = RegraAreaLazerAlpha;
            let empreendimento = Empreendimento {
                construtora: "Alpha".to_string(),
                numero_de_torres: 0,
                area_do_terreno: 1000.0,
                area_da_torre: 100.0,
                altura_da_torre: 10.0,
                area_de_lazer: Some(50.0),
                cidade: "São Paulo".to_string(),
            };

            assert_eq!(regra.validar(&empreendimento), None, "Alpha com 0 torres não deve aplicar regra");
        }

        #[test]
        fn test_construtora_alpha_area_de_lazer_zero() {
            let regra = RegraAreaLazerAlpha;
            let empreendimento = Empreendimento {
                construtora: "Alpha".to_string(),
                numero_de_torres: 2,
                area_do_terreno: 1000.0,
                area_da_torre: 100.0,
                altura_da_torre: 10.0,
                area_de_lazer: Some(0.0),
                cidade: "São Paulo".to_string(),
            };

            let resultado = regra.validar(&empreendimento);
            assert!(resultado.is_some(), "Alpha com área de lazer zero deve violar");
            assert!(resultado.unwrap().contains("Área de lazer insuficiente para Alpha: 0.00 m²"));
        }
    }

    mod integracao_com_outras_regras {
        use super::*;

        #[test]
        fn test_alpha_viola_regra_especifica_mas_nao_padrao() {
            let empreendimento = Empreendimento {
                construtora: "Alpha".to_string(),
                cidade: "São Paulo".to_string(),
                area_do_terreno: 1000.0,
                numero_de_torres: 2,
                altura_da_torre: 25.0,
                area_da_torre: 300.0,
                area_de_lazer: Some(80.0),
            };

            let regra_alpha = RegraAreaLazerAlpha;
            let resultado = regra_alpha.validar(&empreendimento);

            assert!(resultado.is_some(), "Deve violar regra Alpha específica");
            assert!(resultado.unwrap().contains("Área de lazer insuficiente para Alpha: 80.00 m²"));
        }

        #[test]
        fn test_alpha_viola_regra_especifica_e_padrao() {
            let empreendimento = Empreendimento {
                construtora: "Alpha".to_string(),
                cidade: "São Paulo".to_string(),
                area_do_terreno: 1000.0,
                numero_de_torres: 2,
                altura_da_torre: 35.0,
                area_da_torre: 500.0,
                area_de_lazer: Some(80.0),
            };

            let regra_alpha = RegraAreaLazerAlpha;
            let resultado = regra_alpha.validar(&empreendimento);

            assert!(resultado.is_some(), "Deve violar regra Alpha específica independente de outras violações");
            assert!(resultado.unwrap().contains("Área de lazer insuficiente para Alpha: 80.00 m²"));
        }
    }
}
