use arqgen::business_logic::validar_empreendimentos;
use arqgen::models::empreendimento::Empreendimento;
mod common;
use common::*;

#[test]
fn test_integracao_completa_regras_negocio() {
    let empreendimento_complexo = criar_empreendimento_com_violacoes();

    let resultado = validar_empreendimentos(&[empreendimento_complexo]);
    
    assert_eq!(resultado.len(), 1);
    assert!(!resultado[0].regras_ok, "Deve violar múltiplas regras");

    assert!(
        tem_pelo_menos_mensagens(&resultado[0], 4),
        "Deve detectar pelo menos 4 violações, encontrou {}",
        resultado[0].mensagens.len()
    );

    assert!(
        contem_mensagem(&resultado[0], "Altura da torre deve ser inferior a 30"),
        "Deve detectar violação de altura máxima"
    );
    assert!(
        contem_mensagem(&resultado[0], "Área total das torres não pode exceder 80%"),
        "Deve detectar violação de área das torres"
    );
    assert!(
        contem_mensagem(&resultado[0], "Área de lazer insuficiente"),
        "Deve detectar violação de área de lazer Alpha"
    );
}

#[test]
fn test_integracao_regras_padrao_especificas() {
    let empreendimentos = vec![
        Empreendimento {
            construtora: "Beta".to_string(),
            cidade: "São Paulo".to_string(),
            area_do_terreno: 1000.0,
            numero_de_torres: 2,
            altura_da_torre: 25.0,
            area_da_torre: 300.0,
            area_de_lazer: Some(120.0),
        },
        Empreendimento {
            construtora: "Alpha".to_string(),
            cidade: "São Paulo".to_string(),
            area_do_terreno: 1000.0,
            numero_de_torres: 2,
            altura_da_torre: 25.0,
            area_da_torre: 300.0,
            area_de_lazer: Some(80.0),
        },
    ];

    let resultados = validar_empreendimentos(&empreendimentos);

    assert!(resultados[0].regras_ok, "Primeiro empreendimento deve ser válido");
    assert!(resultados[0].mensagens.is_empty(), "Primeiro não deve ter mensagens de erro");

    assert!(!resultados[1].regras_ok, "Segundo deve violar regras");
    assert!(
        tem_pelo_menos_mensagens(&resultados[1], 2),
        "Deve violar pelo menos 2 regras (padrão + Alpha)"
    );
    assert!(
        contem_mensagem(&resultados[1], "Área de lazer insuficiente"),
        "Deve violar regra de área de lazer"
    );
}

#[test]
fn test_integracao_regras_ignoradas() {
    let empreendimento = criar_empreendimento_por_cidade("CidadeEspecial");

    let resultados = validar_empreendimentos(&[empreendimento]);
    
    assert_eq!(resultados.len(), 1);
    
    assert!(
        resultados[0].empreendimento == "Teste",
        "Deve retornar o nome da construtora corretamente"
    );
}

#[test]
fn test_integracao_diferentes_combinacoes() {
    let empreendimentos = gerar_empreendimentos_teste();

    let resultados = validar_empreendimentos(&empreendimentos);
    
    assert_eq!(resultados.len(), empreendimentos.len(), "Deve processar todos os empreendimentos");
    
    let validos = resultados.iter().filter(|r| r.regras_ok).count();
    let invalidos = resultados.iter().filter(|r| !r.regras_ok).count();
    
    assert!(validos > 0, "Deve haver pelo menos alguns empreendimentos válidos");
    assert!(invalidos > 0, "Deve haver pelo menos alguns empreendimentos inválidos");
    
    for resultado in &resultados {
        assert!(!resultado.empreendimento.is_empty(), "Campo empreendimento deve estar preenchido");
    }
}

#[test]
fn test_integracao_cenarios_borda_extremos() {
    let empreendimentos = vec![
        Empreendimento {
            construtora: "Extrema".to_string(),
            cidade: "São Paulo".to_string(),
            area_do_terreno: 0.1,
            numero_de_torres: 1,
            altura_da_torre: 0.1,
            area_da_torre: 0.05,
            area_de_lazer: Some(0.01),
        },
        Empreendimento {
            construtora: "Mega".to_string(),
            cidade: "São Paulo".to_string(),
            area_do_terreno: 10000.0,
            numero_de_torres: 20,
            altura_da_torre: 100.0,
            area_da_torre: 1000.0,
            area_de_lazer: Some(500.0),
        },
        Empreendimento {
            construtora: "Limite".to_string(),
            cidade: "São Paulo".to_string(),
            area_do_terreno: 1000.0,
            numero_de_torres: 2,
            altura_da_torre: 29.999,
            area_da_torre: 399.999,
            area_de_lazer: Some(99.999),
        },
    ];

    let resultados = validar_empreendimentos(&empreendimentos);
    
    assert_eq!(resultados.len(), 3);
    
    assert!(resultados[0].regras_ok, "Empreendimento com 1 torre em São Paulo deve ser válido");
    assert!(!resultados[1].regras_ok, "Empreendimento extremamente grande deve violar regras");
    assert!(!resultados[2].regras_ok, "Empreendimento no limite deve violar regra de área de lazer");
}

#[test]
fn test_integracao_cenarios_especificos() {
    let empreendimento_especial = Empreendimento {
        construtora: "Alpha".to_string(),
        cidade: "Boituva".to_string(),
        area_do_terreno: 1000.0,
        numero_de_torres: 6,
        altura_da_torre: 25.0,
        area_da_torre: 300.0,
        area_de_lazer: Some(80.0),
    };

    let resultado = validar_empreendimentos(&[empreendimento_especial]);
    
    assert_eq!(resultado.len(), 1);
    assert!(!resultado[0].regras_ok, "Deve violar regras de cidade e construtora");
    
    assert!(
        tem_pelo_menos_mensagens(&resultado[0], 2),
        "Deve violar pelo menos 2 regras específicas"
    );
    
    let mensagens = &resultado[0].mensagens;    
    let tem_regra_cidade = mensagens.iter().any(|m| m.contains("torres"));
    let tem_regra_construtora = mensagens.iter().any(|m| m.contains("Área de lazer insuficiente"));
    
    assert!(tem_regra_cidade || tem_regra_construtora, "Deve aplicar pelo menos uma regra específica");
}
