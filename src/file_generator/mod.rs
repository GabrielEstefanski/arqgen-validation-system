pub mod csv_generator;
pub mod json_generator;
pub mod parquet_generator;
pub mod xml_generator;

pub use csv_generator::write_csv;
pub use json_generator::write_json;
pub use parquet_generator::write_parquet;
pub use xml_generator::write_xml;

use crate::models::empreendimento::Empreendimento;
use std::error::Error;

pub fn generate_all_files(empreendimentos: &[Empreendimento], base_name: &str) -> Result<(), Box<dyn Error>> {
    println!("🔄 Gerando todos os tipos de arquivo...");
    
    let csv_path = format!("{}.csv", base_name);
    write_csv(&csv_path, empreendimentos)?;
    println!("✅ CSV gerado: {}", csv_path);
    
    let json_path = format!("{}.json", base_name);
    write_json(&json_path, empreendimentos)?;
    println!("✅ JSON gerado: {}", json_path);
    
    let parquet_path = format!("{}.parquet", base_name);
    write_parquet(&parquet_path, empreendimentos)?;
    println!("✅ Parquet gerado: {}", parquet_path);
    
    let xml_path = format!("{}.xml", base_name);
    write_xml(&xml_path, empreendimentos)?;
    println!("✅ XML gerado: {}", xml_path);
    
    println!("🎉 Todos os arquivos foram gerados com sucesso!");
    Ok(())
}
