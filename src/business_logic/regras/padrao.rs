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

    fn make_empreendimento() -> Empreendimento {
        Empreendimento {
            construtora: "Teste".to_string(),
            cidade: "São Paulo".to_string(),
            area_do_terreno: 1000.0,
            numero_de_torres: 2,
            altura_da_torre: 25.0,
            area_da_torre: 300.0,
            area_de_lazer: Some(150.0),
        }
    }

    mod regra_altura_max {
        use super::*;

        #[test]
        fn altura_abaixo_limite_deve_passar() {
            let regra = RegraAlturaMax(30.0);
            let mut empreendimento = make_empreendimento();
            empreendimento.altura_da_torre = 25.0;

            assert!(regra.validar(&empreendimento).is_none());
        }

        #[test]
        fn altura_no_limite_deve_falhar() {
            let regra = RegraAlturaMax(30.0);
            let mut empreendimento = make_empreendimento();
            empreendimento.altura_da_torre = 30.0;

            let resultado = regra.validar(&empreendimento);
            assert!(resultado.is_some());
            assert!(
                resultado
                    .unwrap()
                    .contains("Altura da torre deve ser inferior a 30m")
            );
        }

        #[test]
        fn altura_acima_limite_deve_falhar() {
            let regra = RegraAlturaMax(30.0);
            let mut empreendimento = make_empreendimento();
            empreendimento.altura_da_torre = 35.0;

            let resultado = regra.validar(&empreendimento);
            assert!(resultado.is_some());
            assert!(
                resultado
                    .unwrap()
                    .contains("Altura da torre deve ser inferior a 30m")
            );
        }
    }

    mod regra_area_torres_max {
        use super::*;

        #[test]
        fn area_torres_abaixo_limite_deve_passar() {
            let regra = RegraAreaTorresMax(0.8);
            let mut empreendimento = make_empreendimento();
            empreendimento.numero_de_torres = 2;
            empreendimento.area_da_torre = 300.0; // 600 < 800

            assert!(regra.validar(&empreendimento).is_none());
        }

        #[test]
        fn area_torres_no_limite_deve_falhar() {
            let regra = RegraAreaTorresMax(0.8);
            let mut empreendimento = make_empreendimento();
            empreendimento.numero_de_torres = 4;
            empreendimento.area_da_torre = 200.0; // 800 == 80%

            let resultado = regra.validar(&empreendimento);
            assert!(resultado.is_some());
            assert!(
                resultado
                    .unwrap()
                    .contains("Área total das torres não pode exceder 80%")
            );
        }

        #[test]
        fn area_torres_acima_limite_deve_falhar() {
            let regra = RegraAreaTorresMax(0.8);
            let mut empreendimento = make_empreendimento();
            empreendimento.numero_de_torres = 5;
            empreendimento.area_da_torre = 200.0; // 1000 > 80%

            let resultado = regra.validar(&empreendimento);
            assert!(resultado.is_some());
            assert!(
                resultado
                    .unwrap()
                    .contains("Área total das torres não pode exceder 80%")
            );
        }
    }

    mod regra_area_lazer_min {
        use super::*;

        #[test]
        fn com_1_torre_nao_aplica() {
            let regra = RegraAreaLazerMin(0.1);
            let mut empreendimento = make_empreendimento();
            empreendimento.numero_de_torres = 1;

            assert!(regra.validar(&empreendimento).is_none());
        }

        #[test]
        fn com_area_suficiente_deve_passar() {
            let regra = RegraAreaLazerMin(0.1);
            let empreendimento = make_empreendimento(); // 150 >= 100

            assert!(regra.validar(&empreendimento).is_none());
        }

        #[test]
        fn com_area_no_limite_deve_passar() {
            let regra = RegraAreaLazerMin(0.1);
            let mut empreendimento = make_empreendimento();
            empreendimento.area_de_lazer = Some(100.0);

            assert!(regra.validar(&empreendimento).is_none());
        }

        #[test]
        fn com_area_insuficiente_deve_falhar() {
            let regra = RegraAreaLazerMin(0.1);
            let mut empreendimento = make_empreendimento();
            empreendimento.area_de_lazer = Some(80.0);

            let resultado = regra.validar(&empreendimento);
            assert!(resultado.is_some());
            assert!(
                resultado
                    .unwrap()
                    .contains("Área de lazer insuficiente: 80.00 m²")
            );
        }

        #[test]
        fn sem_area_de_lazer_deve_falhar() {
            let regra = RegraAreaLazerMin(0.1);
            let mut empreendimento = make_empreendimento();
            empreendimento.area_de_lazer = None;

            let resultado = regra.validar(&empreendimento);
            assert_eq!(resultado.unwrap(), "Faltando campo 'area-de-lazer'.");
        }
    }

    mod integracao {
        use super::*;

        #[test]
        fn empreendimento_invalido_quebra_multiplas_regras() {
            let mut empreendimento = make_empreendimento();
            empreendimento.altura_da_torre = 35.0; // viola altura
            empreendimento.numero_de_torres = 5; // viola área torres
            empreendimento.area_de_lazer = Some(80.0); // viola lazer

            let regra_altura = RegraAlturaMax(30.0);
            let regra_torres = RegraAreaTorresMax(0.8);
            let regra_lazer = RegraAreaLazerMin(0.1);

            assert!(regra_altura.validar(&empreendimento).is_some());
            assert!(regra_torres.validar(&empreendimento).is_some());
            assert!(regra_lazer.validar(&empreendimento).is_some());
        }

        #[test]
        fn empreendimento_valido_passa_todas_regras() {
            let empreendimento = make_empreendimento();

            let regra_altura = RegraAlturaMax(30.0);
            let regra_torres = RegraAreaTorresMax(0.8);
            let regra_lazer = RegraAreaLazerMin(0.1);

            assert!(regra_altura.validar(&empreendimento).is_none());
            assert!(regra_torres.validar(&empreendimento).is_none());
            assert!(regra_lazer.validar(&empreendimento).is_none());
        }
    }
}
