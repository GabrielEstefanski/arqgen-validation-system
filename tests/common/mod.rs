use arqgen::models::empreendimento::Empreendimento;
use arqgen::business_logic::validator::ValidationResult;

pub fn criar_empreendimento_valido() -> Empreendimento {
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

pub fn criar_empreendimento_com_violacoes() -> Empreendimento {
    Empreendimento {
        construtora: "Alpha".to_string(),
        cidade: "Boituva".to_string(),
        area_do_terreno: 800.0,
        numero_de_torres: 6,
        altura_da_torre: 35.0,
        area_da_torre: 200.0,
        area_de_lazer: Some(50.0),
    }
}

pub fn criar_empreendimento_por_cidade(cidade: &str) -> Empreendimento {
    Empreendimento {
        construtora: "Teste".to_string(),
        cidade: cidade.to_string(),
        area_do_terreno: 1000.0,
        numero_de_torres: 2,
        altura_da_torre: 25.0,
        area_da_torre: 300.0,
        area_de_lazer: Some(150.0),
    }
}

pub fn gerar_empreendimentos_teste() -> Vec<Empreendimento> {
    vec![
        criar_empreendimento_valido(),
        criar_empreendimento_com_violacoes(),
        Empreendimento {
            construtora: "Alpha".to_string(),
            cidade: "São Paulo".to_string(),
            area_do_terreno: 1000.0,
            numero_de_torres: 2,
            altura_da_torre: 25.0,
            area_da_torre: 300.0,
            area_de_lazer: Some(80.0),
        },
        Empreendimento {
            construtora: "Teste".to_string(),
            cidade: "Guaratinguetá".to_string(),
            area_do_terreno: 800.0,
            numero_de_torres: 4,
            altura_da_torre: 40.0,
            area_da_torre: 150.0,
            area_de_lazer: Some(80.0),
        },
    ]
}

pub fn contem_mensagem(resultado: &ValidationResult, texto: &str) -> bool {
    resultado.mensagens.iter().any(|m| m.contains(texto))
}

pub fn tem_pelo_menos_mensagens(resultado: &ValidationResult, count: usize) -> bool {
    resultado.mensagens.len() >= count
}
