mod business_logic;
mod file_reader;
mod file_generator;
mod models;

use business_logic::validar_empreendimentos;
use clap::Parser;
use colored::*;
use file_reader::{FileType, read_file};

#[derive(Parser)]
#[command(name = "arqgen")]
#[command(author = "Gabriel")]
#[command(version = "1.0.0")]
#[command(about = "Valida empreendimentos a partir de arquivos CSV ou Parquet", long_about = None)]
struct Cli {
    #[arg(short, long)]
    path: String,

    #[arg(short, long, default_value = "csv")]
    file_type: String,
}

fn main() {
    let cli = Cli::parse();

    let file_type = match std::path::Path::new(&cli.path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase()
        .as_str()
    {
        "csv" => FileType::Csv,
        "json" => FileType::Json,
        "parquet" => FileType::Parquet,
        "xml" => FileType::Xml,
        _ => {
            eprintln!(
                "{}",
                "Tipo de arquivo inválido. Use csv, json, parquet ou xml.".red()
            );
            std::process::exit(1);
        }
    };

    let empreendimentos = match read_file(file_type, &cli.path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("{}: {}", "Erro ao ler o arquivo".red(), e);
            std::process::exit(1);
        }
    };

    let resultados = validar_empreendimentos(&empreendimentos);

    println!("{}", "===== Resultados da Validação =====".bold().blue());
    for resultado in resultados {
        println!("Empreendimento: {}", resultado.empreendimento.bold());
        if resultado.regras_ok {
            println!("  {}", "✅ Todas as regras foram atendidas!".green());
        } else {
            println!("  {}", "❌ Regras violadas:".red());
            for msg in resultado.mensagens {
                println!("    - {}", msg.yellow());
            }
        }
        println!("{}", "-".repeat(40));
    }
}
