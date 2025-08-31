use super::padrao::RegraNegocio;
use crate::models::empreendimento::Empreendimento;

#[derive(Debug)]
pub struct RegraMaxTorres(pub u32);
#[derive(Debug)]
pub struct RegraAlturaPorTorresGuaratingueta;

impl RegraNegocio for RegraMaxTorres {
    fn validar(&self, e: &Empreendimento) -> Option<String> {
        if e.numero_de_torres > self.0 {
            Some(format!(
                "Número de torres ({}) excede o máximo permitido ({})",
                e.numero_de_torres, self.0
            ))
        } else {
            None
        }
    }
    fn nome(&self) -> &'static str {
        "RegraMaxTorres"
    }
}

impl RegraNegocio for RegraAlturaPorTorresGuaratingueta {
    fn validar(&self, e: &Empreendimento) -> Option<String> {
        let limite = match e.numero_de_torres {
            1..=2 => 25.0,
            3 => 20.0,
            _ => 15.0,
        };
        if e.altura_da_torre >= limite {
            Some(format!(
                "Altura da torre ({}) excede o limite para {} torres ({})",
                e.altura_da_torre, e.numero_de_torres, limite
            ))
        } else {
            None
        }
    }
    fn nome(&self) -> &'static str {
        "RegraAlturaPorTorresGuaratingueta"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::empreendimento::Empreendimento;

    #[test]
    fn test_regra_max_torres() {
        let regra = RegraMaxTorres(3);

        let caso_ok = Empreendimento {
            construtora: "Alpha".to_string(),
            numero_de_torres: 3,
            area_do_terreno: 1000.0,
            area_da_torre: 100.0,
            altura_da_torre: 10.0,
            area_de_lazer: Some(150.0),
            cidade: "Guaratinguetá".to_string(),
        };

        let caso_falha = Empreendimento {
            construtora: "Alpha".to_string(),
            numero_de_torres: 4,
            area_do_terreno: 1000.0,
            area_da_torre: 100.0,
            altura_da_torre: 10.0,
            area_de_lazer: Some(150.0),
            cidade: "Guaratinguetá".to_string(),
        };

        assert_eq!(regra.validar(&caso_ok), None);

        let msg = regra.validar(&caso_falha).unwrap();
        assert_eq!(msg, "Número de torres (4) excede o máximo permitido (3)");
    }

    #[test]
    fn test_regra_altura_por_torres_guaratingueta() {
        let regra = RegraAlturaPorTorresGuaratingueta;

        let casos = vec![
            (1, 24.0, true),
            (2, 25.0, false),
            (3, 19.5, true),
            (3, 21.0, false),
            (4, 14.0, true),
            (5, 16.0, false),
        ];

        for (num_torres, altura, ok) in casos {
            let empreendimento = Empreendimento {
                construtora: "Alpha".to_string(),
                numero_de_torres: num_torres,
                area_do_terreno: 1000.0,
                area_da_torre: 100.0,
                altura_da_torre: altura,
                area_de_lazer: Some(150.0),
                cidade: "Guaratinguetá".to_string(),
            };

            let resultado = regra.validar(&empreendimento);
            if ok {
                assert!(
                    resultado.is_none(),
                    "Esperava None para {} torres e altura {}, mas obteve {:?}",
                    num_torres,
                    altura,
                    resultado
                );
            } else {
                assert!(
                    resultado.is_some(),
                    "Esperava Some para {} torres e altura {}, mas obteve None",
                    num_torres,
                    altura
                );
            }
        }
    }
}
