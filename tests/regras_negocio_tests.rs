use arqgen::business_logic::validar_empreendimentos;
use arqgen::models::empreendimento::Empreendimento;

#[test]
fn deve_validar_multiplos_empreendimentos_com_diferentes_regras() {
    let empreendimentos = vec![
        Empreendimento {
            construtora: "Alpha".to_string(),
            cidade: "São Paulo".to_string(),
            area_do_terreno: 1000.0,
            numero_de_torres: 2,
            altura_da_torre: 25.0,
            area_da_torre: 300.0,
            area_de_lazer: Some(150.0),
        },
        Empreendimento {
            construtora: "Beta".to_string(),
            cidade: "Boituva".to_string(),
            area_do_terreno: 800.0,
            numero_de_torres: 6,
            altura_da_torre: 35.0,
            area_da_torre: 200.0,
            area_de_lazer: Some(50.0),
        },
        Empreendimento {
            construtora: "Alpha".to_string(),
            cidade: "Guaratinguetá".to_string(),
            area_do_terreno: 1200.0,
            numero_de_torres: 3,
            altura_da_torre: 28.0,
            area_da_torre: 250.0,
            area_de_lazer: Some(80.0),
        },
    ];

    let resultados = validar_empreendimentos(&empreendimentos);

    assert_eq!(resultados.len(), 3, "Deve retornar 3 resultados");

    assert!(
        resultados[0].regras_ok,
        "Primeiro empreendimento deve ser válido"
    );
    assert!(
        resultados[0].mensagens.is_empty(),
        "Primeiro empreendimento não deve ter mensagens de erro"
    );

    assert!(
        !resultados[1].regras_ok,
        "Segundo empreendimento deve ter violações"
    );
    assert!(
        !resultados[1].mensagens.is_empty(),
        "Segundo empreendimento deve ter mensagens de erro"
    );

    assert!(
        resultados[1]
            .mensagens
            .iter()
            .any(|m| m.contains("Altura da torre deve ser inferior a 30")),
        "Deve detectar violação de altura máxima"
    );

    assert!(
        !resultados[2].regras_ok,
        "Terceiro empreendimento deve ter violações"
    );
    assert!(
        resultados[2]
            .mensagens
            .iter()
            .any(|m| m.contains("Área de lazer insuficiente")),
        "Deve detectar violação da regra Alpha de área de lazer"
    );
}

#[test]
fn deve_aplicar_regras_especificas_por_cidade() {
    let empreendimentos = vec![
        Empreendimento {
            construtora: "Gamma".to_string(),
            cidade: "Boituva".to_string(),
            area_do_terreno: 1000.0,
            numero_de_torres: 6,
            altura_da_torre: 20.0,
            area_da_torre: 100.0,
            area_de_lazer: Some(100.0),
        },
        Empreendimento {
            construtora: "Delta".to_string(),
            cidade: "Guaratinguetá".to_string(),
            area_do_terreno: 800.0,
            numero_de_torres: 4,
            altura_da_torre: 40.0,
            area_da_torre: 150.0,
            area_de_lazer: Some(80.0),
        },
    ];

    let resultados = validar_empreendimentos(&empreendimentos);

    assert!(
        !resultados[0].regras_ok,
        "Empreendimento em Boituva deve violar regra de torres"
    );
    assert!(
        resultados[0].mensagens.iter().any(|m| m.contains("torres")),
        "Deve detectar violação da regra de Boituva"
    );

    assert!(
        !resultados[1].regras_ok,
        "Empreendimento em Guaratinguetá deve ter violações"
    );
    assert!(
        resultados[1]
            .mensagens
            .iter()
            .any(|m| m.contains("Altura da torre")),
        "Deve detectar violação da regra de altura específica de Guaratinguetá"
    );
}

#[test]
fn deve_aplicar_regras_especificas_por_construtora() {
    let empreendimentos = vec![
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
            construtora: "Beta".to_string(),
            cidade: "São Paulo".to_string(),
            area_do_terreno: 1000.0,
            numero_de_torres: 2,
            altura_da_torre: 25.0,
            area_da_torre: 300.0,
            area_de_lazer: Some(80.0),
        },
    ];

    let resultados = validar_empreendimentos(&empreendimentos);

    assert!(
        !resultados[0].regras_ok,
        "Empreendimento Alpha deve violar regra de área de lazer"
    );
    assert!(
        resultados[0]
            .mensagens
            .iter()
            .any(|m| m.contains("Área de lazer insuficiente")),
        "Deve detectar violação da regra Alpha de área de lazer"
    );

    assert!(
        !resultados[1].regras_ok,
        "Empreendimento Beta deve violar regra padrão de área de lazer"
    );
    assert!(
        resultados[1]
            .mensagens
            .iter()
            .any(|m| m.contains("Área de lazer insuficiente")),
        "Deve detectar violação da regra padrão de área de lazer"
    );
}

#[test]
fn deve_tratar_cenarios_limite_e_edge_cases() {
    let empreendimentos = vec![
        Empreendimento {
            construtora: "Zeta".to_string(),
            cidade: "São Paulo".to_string(),
            area_do_terreno: 100.0,
            numero_de_torres: 1,
            altura_da_torre: 1.0,
            area_da_torre: 50.0,
            area_de_lazer: None,
        },
        Empreendimento {
            construtora: "Eta".to_string(),
            cidade: "São Paulo".to_string(),
            area_do_terreno: 1000.0,
            numero_de_torres: 2,
            altura_da_torre: 30.0,
            area_da_torre: 300.0,
            area_de_lazer: Some(100.0),
        },
        Empreendimento {
            construtora: "Theta".to_string(),
            cidade: "São Paulo".to_string(),
            area_do_terreno: 1000.0,
            numero_de_torres: 4,
            altura_da_torre: 25.0,
            area_da_torre: 200.0,
            area_de_lazer: Some(100.0),
        },
    ];

    let resultados = validar_empreendimentos(&empreendimentos);

    assert!(
        resultados[0].regras_ok,
        "Empreendimento com 1 torre deve ser válido (não aplica regra de área de lazer)"
    );
    assert!(
        resultados[1].regras_ok,
        "Empreendimento em São Paulo com altura no limite deve ser válido (ignora regra de altura)"
    );
    assert!(
        !resultados[2].regras_ok,
        "Empreendimento com área das torres no limite deve violar"
    );
    assert!(
        resultados[2]
            .mensagens
            .iter()
            .any(|m| m.contains("Área total das torres não pode exceder 80%")),
        "Deve detectar violação de área máxima das torres"
    );
}

#[test]
fn deve_ignorar_regras_em_cidades_especificas() {
    let empreendimentos = vec![Empreendimento {
        construtora: "Iota".to_string(),
        cidade: "São Paulo".to_string(),
        area_do_terreno: 1000.0,
        numero_de_torres: 2,
        altura_da_torre: 35.0,
        area_da_torre: 300.0,
        area_de_lazer: Some(100.0),
    }];

    let resultados = validar_empreendimentos(&empreendimentos);

    assert!(
        resultados[0].regras_ok,
        "Empreendimento em São Paulo deve ser válido (ignora regra de altura)"
    );
    assert!(
        resultados[0].mensagens.is_empty(),
        "Não deve ter mensagens de erro"
    );
}
