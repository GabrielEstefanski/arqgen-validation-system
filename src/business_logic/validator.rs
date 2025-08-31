use crate::business_logic::regras::REGRAS_IGNORADAS_POR_CIDADE;
use crate::business_logic::regras::RegraNegocio;
use crate::business_logic::regras::RegrasFactory;
use crate::models::empreendimento::Empreendimento;

pub struct ValidationResult {
    pub empreendimento: String,
    pub regras_ok: bool,
    pub mensagens: Vec<String>,
}

pub fn validar_empreendimento(e: &Empreendimento) -> ValidationResult {
    let mut mensagens = Vec::new();

    let ignoradas = REGRAS_IGNORADAS_POR_CIDADE
        .get(e.cidade.as_str())
        .map(|v| v.as_slice())
        .unwrap_or(&[]);

    let todas: Vec<Box<dyn RegraNegocio>> = RegrasFactory::padrao()
        .into_iter()
        .chain(RegrasFactory::por_cidade(&e.cidade))
        .chain(RegrasFactory::por_construtora(&e.construtora))
        .filter(|r| !ignoradas.contains(&r.nome()))
        .collect();

    for regra in todas {
        if let Some(msg) = regra.validar(e) {
            mensagens.push(msg);
        }
    }

    ValidationResult {
        empreendimento: e.construtora.clone(),
        regras_ok: mensagens.is_empty(),
        mensagens,
    }
}

pub fn validar_empreendimentos(empreendimentos: &[Empreendimento]) -> Vec<ValidationResult> {
    empreendimentos
        .iter()
        .map(|e| validar_empreendimento(e))
        .collect()
}
