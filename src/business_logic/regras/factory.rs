use super::cidades::*;
use super::construtoras::*;
use super::padrao::RegraNegocio;
use super::padrao::*;

pub struct RegrasFactory;

impl RegrasFactory {
    pub fn padrao() -> Vec<Box<dyn RegraNegocio>> {
        vec![
            Box::new(RegraAlturaMax(30.0)),
            Box::new(RegraAreaTorresMax(0.8)),
            Box::new(RegraAreaLazerMin(0.1)),
        ]
    }

    pub fn por_cidade(cidade: &str) -> Vec<Box<dyn RegraNegocio>> {
        match cidade {
            "Boituva" => vec![Box::new(RegraMaxTorres(5))],
            "Guaratinguetá" => vec![Box::new(RegraAlturaPorTorresGuaratingueta)],
            _ => vec![],
        }
    }

    pub fn por_construtora(construtora: &str) -> Vec<Box<dyn RegraNegocio>> {
        match construtora {
            "Alpha" => vec![Box::new(RegraAreaLazerAlpha)],
            _ => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::empreendimento::Empreendimento;

    #[test]
    fn test_padrao_regras() {
        let regras = RegrasFactory::padrao();

        assert_eq!(regras.len(), 3, "Deve retornar 3 regras padrão");

        let mut tipos = regras.iter().map(|r| r.nome()).collect::<Vec<_>>();
        tipos.sort();
        let esperado = vec!["RegraAlturaMax", "RegraAreaLazerMin", "RegraAreaTorresMax"];
        assert_eq!(
            tipos, esperado,
            "As regras padrão não correspondem ao esperado"
        );
    }

    #[test]
    fn test_regras_por_cidade() {
        let boituva = RegrasFactory::por_cidade("Boituva");
        assert_eq!(boituva.len(), 1);
        assert_eq!(boituva[0].nome(), "RegraMaxTorres");

        let guaratingueta = RegrasFactory::por_cidade("Guaratinguetá");
        assert_eq!(guaratingueta.len(), 1);
        assert_eq!(guaratingueta[0].nome(), "RegraAlturaPorTorresGuaratingueta");

        let desconhecida = RegrasFactory::por_cidade("CidadeX");
        assert!(
            desconhecida.is_empty(),
            "Cidades desconhecidas devem retornar vazio"
        );
    }

    #[test]
    fn test_regras_por_construtora() {
        let alpha = RegrasFactory::por_construtora("Alpha");
        assert_eq!(alpha.len(), 1);
        assert_eq!(alpha[0].nome(), "RegraAreaLazerAlpha");

        let beta = RegrasFactory::por_construtora("Beta");
        assert!(
            beta.is_empty(),
            "Construtoras sem regras devem retornar vazio"
        );
    }

    #[test]
    fn test_combinacao_de_regras_em_empreendimento() {
        let e = Empreendimento {
            construtora: "Alpha".to_string(),
            numero_de_torres: 2,
            area_do_terreno: 1000.0,
            area_da_torre: 100.0,
            altura_da_torre: 20.0,
            area_de_lazer: Some(50.0),
            cidade: "Boituva".to_string(),
        };

        let todas: Vec<Box<dyn RegraNegocio>> = RegrasFactory::padrao()
            .into_iter()
            .chain(RegrasFactory::por_cidade(&e.cidade))
            .chain(RegrasFactory::por_construtora(&e.construtora))
            .collect();

        let mut mensagens = Vec::new();
        for regra in todas {
            if let Some(msg) = regra.validar(&e) {
                mensagens.push(msg);
            }
        }

        assert!(
            !mensagens.is_empty(),
            "Deve haver pelo menos uma regra violada"
        );
        assert!(
            mensagens
                .iter()
                .any(|m| m.contains("Área de lazer insuficiente")),
            "Deve detectar violação da RegraAreaLazerAlpha"
        );
    }
}
