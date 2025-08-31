use arqgen::file_reader::read_xml;
use arqgen::file_generator::generate_all_files;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("🔄 Lendo dados do arquivo XML...");
    
    let empreendimentos = read_xml("dados.xml")?;
    println!("✅ Lidos {} empreendimentos do XML", empreendimentos.len());
    
    generate_all_files(&empreendimentos, "dados")?;
    
    println!("\n📊 Resumo dos dados:");
    for (i, emp) in empreendimentos.iter().enumerate() {
        println!("  {}. {} - {} ({} torres, {}m² terreno)", 
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
