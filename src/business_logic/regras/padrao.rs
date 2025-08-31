use crate::models::empreendimento::Empreendimento;
use std::fmt::Debug;

pub trait RegraNegocio: Debug {
    fn validar(&self, e: &Empreendimento) -> Option<String>;
    fn nome(&self) -> &'static str;
}

#[derive(Debug)]
pub struct RegraAlturaMax(pub f64);
#[derive(Debug)]
pub struct RegraAreaTorresMax(pub f64);
#[derive(Debug)]
pub struct RegraAreaLazerMin(pub f64);

impl RegraNegocio for RegraAlturaMax {
    fn validar(&self, e: &Empreendimento) -> Option<String> {
        if e.altura_da_torre >= self.0 {
            Some(format!("Altura da torre deve ser inferior a {}m.", self.0))
        } else {
            None
        }
    }
    fn nome(&self) -> &'static str {
        "RegraAlturaMax"
    }
}

impl RegraNegocio for RegraAreaTorresMax {
    fn validar(&self, e: &Empreendimento) -> Option<String> {
        let total = e.numero_de_torres as f64 * e.area_da_torre;
        if total >= e.area_do_terreno * self.0 {
            Some(format!(
                "Área total das torres não pode exceder {:.0}% do terreno.",
                self.0 * 100.0
            ))
        } else {
            None
        }
    }
    fn nome(&self) -> &'static str {
        "RegraAreaTorresMax"
    }
}

impl RegraNegocio for RegraAreaLazerMin {
    fn validar(&self, e: &Empreendimento) -> Option<String> {
        if e.numero_de_torres <= 1 {
            return None;
        }
        match e.area_de_lazer {
            Some(area) if area >= e.area_do_terreno * self.0 => None,
            Some(area) => Some(format!("Área de lazer insuficiente: {:.2} m².", area)),
            None => Some("Faltando campo 'area-de-lazer'.".to_string()),
        }
    }
    fn nome(&self) -> &'static str {
        "RegraAreaLazerMin"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::empreendimento::Empreendimento;

    mod regra_altura_max {
        use super::*;

        #[test]
        fn test_altura_abaixo_do_limite() {
            let regra = RegraAlturaMax(30.0);
            let empreendimento = Empreendimento {
                construtora: "Teste".to_string(),
                cidade: "São Paulo".to_string(),
                area_do_terreno: 1000.0,
                numero_de_torres: 2,
                altura_da_torre: 25.0,
                area_da_torre: 300.0,
                area_de_lazer: Some(150.0),
            };

            assert_eq!(regra.validar(&empreendimento), None, "Altura abaixo do limite deve ser válida");
        }

        #[test]
        fn test_altura_exatamente_no_limite() {
            let regra = RegraAlturaMax(30.0);
            let empreendimento = Empreendimento {
                construtora: "Teste".to_string(),
                cidade: "São Paulo".to_string(),
                area_do_terreno: 1000.0,
                numero_de_torres: 2,
                altura_da_torre: 30.0,
                area_da_torre: 300.0,
                area_de_lazer: Some(150.0),
            };

            let resultado = regra.validar(&empreendimento);
            assert!(resultado.is_some(), "Altura exatamente no limite deve violar");
            assert!(resultado.unwrap().contains("Altura da torre deve ser inferior a 30m"));
        }

        #[test]
        fn test_altura_acima_do_limite() {
            let regra = RegraAlturaMax(30.0);
            let empreendimento = Empreendimento {
                construtora: "Teste".to_string(),
                cidade: "São Paulo".to_string(),
                area_do_terreno: 1000.0,
                numero_de_torres: 2,
                altura_da_torre: 35.0,
                area_da_torre: 300.0,
                area_de_lazer: Some(150.0),
            };

            let resultado = regra.validar(&empreendimento);
            assert!(resultado.is_some(), "Altura acima do limite deve violar");
            assert!(resultado.unwrap().contains("Altura da torre deve ser inferior a 30m"));
        }

        #[test]
        fn test_nome_da_regra() {
            let regra = RegraAlturaMax(25.0);
            assert_eq!(regra.nome(), "RegraAlturaMax");
        }
    }

    mod regra_area_torres_max {
        use super::*;

        #[test]
        fn test_area_torres_abaixo_do_limite() {
            let regra = RegraAreaTorresMax(0.8);
            let empreendimento = Empreendimento {
                construtora: "Teste".to_string(),
                cidade: "São Paulo".to_string(),
                area_do_terreno: 1000.0,
                numero_de_torres: 2,
                altura_da_torre: 25.0,
                area_da_torre: 300.0,
                area_de_lazer: Some(150.0),
            };

            assert_eq!(regra.validar(&empreendimento), None, "Área das torres abaixo do limite deve ser válida");
        }

        #[test]
        fn test_area_torres_exatamente_no_limite() {
            let regra = RegraAreaTorresMax(0.8);
            let empreendimento = Empreendimento {
                construtora: "Teste".to_string(),
                cidade: "São Paulo".to_string(),
                area_do_terreno: 1000.0,
                numero_de_torres: 4,
                altura_da_torre: 25.0,
                area_da_torre: 200.0,
                area_de_lazer: Some(150.0),
            };

            let resultado = regra.validar(&empreendimento);
            assert!(resultado.is_some(), "Área das torres exatamente no limite deve violar");
            assert!(resultado.unwrap().contains("Área total das torres não pode exceder 80% do terreno"));
        }

        #[test]
        fn test_area_torres_acima_do_limite() {
            let regra = RegraAreaTorresMax(0.8);
            let empreendimento = Empreendimento {
                construtora: "Teste".to_string(),
                cidade: "São Paulo".to_string(),
                area_do_terreno: 1000.0,
                numero_de_torres: 5,
                altura_da_torre: 25.0,
                area_da_torre: 200.0,
                area_de_lazer: Some(150.0),
            };

            let resultado = regra.validar(&empreendimento);
            assert!(resultado.is_some(), "Área das torres acima do limite deve violar");
            assert!(resultado.unwrap().contains("Área total das torres não pode exceder 80% do terreno"));
        }

        #[test]
        fn test_calculo_correto_da_area_total() {
            let regra = RegraAreaTorresMax(0.5);
            let empreendimento = Empreendimento {
                construtora: "Teste".to_string(),
                cidade: "São Paulo".to_string(),
                area_do_terreno: 1000.0,
                numero_de_torres: 3,
                altura_da_torre: 25.0,
                area_da_torre: 200.0,
                area_de_lazer: Some(150.0),
            };

            let resultado = regra.validar(&empreendimento);
            assert!(resultado.is_some(), "Deve calcular corretamente a área total das torres");
            assert!(resultado.unwrap().contains("Área total das torres não pode exceder 50% do terreno"));
        }

        #[test]
        fn test_nome_da_regra() {
            let regra = RegraAreaTorresMax(0.7);
            assert_eq!(regra.nome(), "RegraAreaTorresMax");
        }
    }

    mod regra_area_lazer_min {
        use super::*;

        #[test]
        fn test_empreendimento_com_1_torre_nao_aplica() {
            let regra = RegraAreaLazerMin(0.1);
            let empreendimento = Empreendimento {
                construtora: "Teste".to_string(),
                cidade: "São Paulo".to_string(),
                area_do_terreno: 1000.0,
                numero_de_torres: 1,
                altura_da_torre: 25.0,
                area_da_torre: 300.0,
                area_de_lazer: Some(50.0),
            };

            assert_eq!(regra.validar(&empreendimento), None, "Empreendimento com 1 torre não deve aplicar regra de área de lazer");
        }

        #[test]
        fn test_empreendimento_com_2_torres_area_suficiente() {
            let regra = RegraAreaLazerMin(0.1);
            let empreendimento = Empreendimento {
                construtora: "Teste".to_string(),
                cidade: "São Paulo".to_string(),
                area_do_terreno: 1000.0,
                numero_de_torres: 2,
                altura_da_torre: 25.0,
                area_da_torre: 300.0,
                area_de_lazer: Some(150.0),
            };

            assert_eq!(regra.validar(&empreendimento), None, "Área de lazer suficiente deve ser válida");
        }

        #[test]
        fn test_empreendimento_com_2_torres_area_exatamente_no_limite() {
            let regra = RegraAreaLazerMin(0.1);
            let empreendimento = Empreendimento {
                construtora: "Teste".to_string(),
                cidade: "São Paulo".to_string(),
                area_do_terreno: 1000.0,
                numero_de_torres: 2,
                altura_da_torre: 25.0,
                area_da_torre: 300.0,
                area_de_lazer: Some(100.0),
            };

            assert_eq!(regra.validar(&empreendimento), None, "Área de lazer exatamente no limite deve ser válida");
        }

        #[test]
        fn test_empreendimento_com_2_torres_area_insuficiente() {
            let regra = RegraAreaLazerMin(0.1);
            let empreendimento = Empreendimento {
                construtora: "Teste".to_string(),
                cidade: "São Paulo".to_string(),
                area_do_terreno: 1000.0,
                numero_de_torres: 2,
                altura_da_torre: 25.0,
                area_da_torre: 300.0,
                area_de_lazer: Some(80.0),
            };

            let resultado = regra.validar(&empreendimento);
            assert!(resultado.is_some(), "Área de lazer insuficiente deve violar");
            assert!(resultado.unwrap().contains("Área de lazer insuficiente: 80.00 m²"));
        }

        #[test]
        fn test_empreendimento_sem_area_de_lazer() {
            let regra = RegraAreaLazerMin(0.1);
            let empreendimento = Empreendimento {
                construtora: "Teste".to_string(),
                cidade: "São Paulo".to_string(),
                area_do_terreno: 1000.0,
                numero_de_torres: 2,
                altura_da_torre: 25.0,
                area_da_torre: 300.0,
                area_de_lazer: None,
            };

            let resultado = regra.validar(&empreendimento);
            assert!(resultado.is_some(), "Campo faltando deve violar");
            assert_eq!(resultado.unwrap(), "Faltando campo 'area-de-lazer'.");
        }

        #[test]
        fn test_empreendimento_com_muitas_torres() {
            let regra = RegraAreaLazerMin(0.1);
            let empreendimento = Empreendimento {
                construtora: "Teste".to_string(),
                cidade: "São Paulo".to_string(),
                area_do_terreno: 1000.0,
                numero_de_torres: 10,
                altura_da_torre: 25.0,
                area_da_torre: 50.0,
                area_de_lazer: Some(80.0),
            };

            let resultado = regra.validar(&empreendimento);
            assert!(resultado.is_some(), "Deve aplicar regra independente do número de torres (>= 2)");
            assert!(resultado.unwrap().contains("Área de lazer insuficiente: 80.00 m²"));
        }

        #[test]
        fn test_nome_da_regra() {
            let regra = RegraAreaLazerMin(0.15);
            assert_eq!(regra.nome(), "RegraAreaLazerMin");
        }
    }

    mod integracao_regras {
        use super::*;

        #[test]
        fn test_empreendimento_viola_multiplas_regras() {
            let empreendimento = Empreendimento {
                construtora: "Teste".to_string(),
                cidade: "São Paulo".to_string(),
                area_do_terreno: 1000.0,
                numero_de_torres: 2,
                altura_da_torre: 35.0,
                area_da_torre: 500.0,
                area_de_lazer: Some(80.0),
            };

            let regra_altura = RegraAlturaMax(30.0);
            let regra_area_torres = RegraAreaTorresMax(0.8);
            let regra_area_lazer = RegraAreaLazerMin(0.1);

            let violacoes_altura = regra_altura.validar(&empreendimento);
            let violacoes_area_torres = regra_area_torres.validar(&empreendimento);
            let violacoes_area_lazer = regra_area_lazer.validar(&empreendimento);

            assert!(violacoes_altura.is_some(), "Deve violar regra de altura");
            assert!(violacoes_area_torres.is_some(), "Deve violar regra de área das torres");
            assert!(violacoes_area_lazer.is_some(), "Deve violar regra de área de lazer");

            assert!(violacoes_altura.unwrap().contains("Altura da torre deve ser inferior a 30m"));
            assert!(violacoes_area_torres.unwrap().contains("Área total das torres não pode exceder 80% do terreno"));
            assert!(violacoes_area_lazer.unwrap().contains("Área de lazer insuficiente: 80.00 m²"));
        }

        #[test]
        fn test_empreendimento_valido_todas_regras() {
            let empreendimento = Empreendimento {
                construtora: "Teste".to_string(),
                cidade: "São Paulo".to_string(),
                area_do_terreno: 1000.0,
                numero_de_torres: 2,
                altura_da_torre: 25.0,
                area_da_torre: 300.0,
                area_de_lazer: Some(150.0),
            };

            let regra_altura = RegraAlturaMax(30.0);
            let regra_area_torres = RegraAreaTorresMax(0.8);
            let regra_area_lazer = RegraAreaLazerMin(0.1);

            let violacoes_altura = regra_altura.validar(&empreendimento);
            let violacoes_area_torres = regra_area_torres.validar(&empreendimento);
            let violacoes_area_lazer = regra_area_lazer.validar(&empreendimento);

            assert!(violacoes_altura.is_none(), "Não deve violar regra de altura");
            assert!(violacoes_area_torres.is_none(), "Não deve violar regra de área das torres");
            assert!(violacoes_area_lazer.is_none(), "Não deve violar regra de área de lazer");
        }
    }
}
