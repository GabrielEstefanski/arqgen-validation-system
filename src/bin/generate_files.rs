use arqgen::file_generator::generate_all_files;
use arqgen::models::empreendimento::Empreendimento;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("🔄 Gerando dados mockados de empreendimentos...");

    let empreendimentos = mock_empreendimentos();
    println!(
        "✅ Gerados {} empreendimentos de teste",
        empreendimentos.len()
    );

    generate_all_files(&empreendimentos, "dados")?;

    println!("\n📊 Resumo dos dados:");
    for (i, emp) in empreendimentos.iter().enumerate() {
        println!(
            "  {}. {} - {} ({} torres, {}m² terreno)",
            i + 1,
            emp.construtora,
            emp.cidade,
            emp.numero_de_torres,
            emp.area_do_terreno
        );
    }

    println!("\n🎯 Arquivos gerados:");
    println!("  • dados.csv");
    println!("  • dados.json");
    println!("  • dados.parquet");
    println!("  • dados.xml");

    Ok(())
}

fn mock_empreendimentos() -> Vec<Empreendimento> {
    vec![
        Empreendimento {
            construtora: "Construtora 1".to_string(),
            cidade: "São Paulo".to_string(),
            area_do_terreno: 1200.0,
            numero_de_torres: 1,
            altura_da_torre: 100.0,
            area_da_torre: 300.0,
            area_de_lazer: None,
        },
        Empreendimento {
            construtora: "Construtora 2".to_string(),
            cidade: "Rio de Janeiro".to_string(),
            area_do_terreno: 800.0,
            numero_de_torres: 4,
            altura_da_torre: 25.0,
            area_da_torre: 100.0,
            area_de_lazer: None,
        },
        Empreendimento {
            construtora: "Construtora 3".to_string(),
            cidade: "Belo Horizonte".to_string(),
            area_do_terreno: 900.0,
            numero_de_torres: 2,
            altura_da_torre: 20.0,
            area_da_torre: 200.0,
            area_de_lazer: Some(50.0),
        },
        Empreendimento {
            construtora: "Construtora 4".to_string(),
            cidade: "Curitiba".to_string(),
            area_do_terreno: 1100.0,
            numero_de_torres: 2,
            altura_da_torre: 22.0,
            area_da_torre: 280.0,
            area_de_lazer: None,
        },
        Empreendimento {
            construtora: "Construtora 5".to_string(),
            cidade: "Porto Alegre".to_string(),
            area_do_terreno: 1000.0,
            numero_de_torres: 2,
            altura_da_torre: 29.0,
            area_da_torre: 250.0,
            area_de_lazer: Some(150.0),
        },
        Empreendimento {
            construtora: "Construtora 6".to_string(),
            cidade: "Boituva".to_string(),
            area_do_terreno: 1500.0,
            numero_de_torres: 6,
            altura_da_torre: 18.0,
            area_da_torre: 150.0,
            area_de_lazer: Some(150.0),
        },
        Empreendimento {
            construtora: "Alpha".to_string(),
            cidade: "Salvador".to_string(),
            area_do_terreno: 100.0,
            numero_de_torres: 1,
            altura_da_torre: 20.0,
            area_da_torre: 50.0,
            area_de_lazer: None,
        },
        Empreendimento {
            construtora: "Beta".to_string(),
            cidade: "Guaratinguetá".to_string(),
            area_do_terreno: 1300.0,
            numero_de_torres: 2,
            altura_da_torre: 26.0,
            area_da_torre: 220.0,
            area_de_lazer: Some(150.0),
        },
        Empreendimento {
            construtora: "Eta".to_string(),
            cidade: "Guaratinguetá".to_string(),
            area_do_terreno: 1300.0,
            numero_de_torres: 3,
            altura_da_torre: 21.0,
            area_da_torre: 220.0,
            area_de_lazer: Some(150.0),
        },
        Empreendimento {
            construtora: "Épsilon".to_string(),
            cidade: "Guaratinguetá".to_string(),
            area_do_terreno: 1300.0,
            numero_de_torres: 4,
            altura_da_torre: 16.0,
            area_da_torre: 220.0,
            area_de_lazer: Some(150.0),
        },
    ]
}
