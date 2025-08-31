use crate::models::empreendimento::Empreendimento;
use std::error::Error;
use std::fs::File;
use std::io::Write;

pub fn write_parquet(path: &str, empreendimentos: &[Empreendimento]) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(path)?;
    
    writeln!(file, "# Parquet-like format (simplified)")?;
    writeln!(file, "# construtora,cidade,area_do_terreno,numero_de_torres,altura_da_torre,area_da_torre,area_de_lazer")?;
    
    for empreendimento in empreendimentos {
        let area_de_lazer = empreendimento.area_de_lazer
            .map(|v| v.to_string())
            .unwrap_or_else(|| "0".to_string());
            
        writeln!(
            file,
            "{},{},{},{},{},{},{}",
            empreendimento.construtora,
            empreendimento.cidade,
            empreendimento.area_do_terreno,
            empreendimento.numero_de_torres,
            empreendimento.altura_da_torre,
            empreendimento.area_da_torre,
            area_de_lazer
        )?;
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    
    #[test]
    fn test_write_parquet() {
        let empreendimentos = vec![
            Empreendimento {
                construtora: "Teste".to_string(),
                cidade: "São Paulo".to_string(),
                area_do_terreno: 1000.0,
                numero_de_torres: 2,
                altura_da_torre: 25.0,
                area_da_torre: 300.0,
                area_de_lazer: Some(150.0),
            }
        ];
        
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_str().unwrap();
        
        let result = write_parquet(path, &empreendimentos);
        assert!(result.is_ok());

        let file_content = std::fs::read_to_string(path).unwrap();
        assert!(!file_content.is_empty());
        assert!(file_content.contains("Teste,São Paulo,1000,2,25,300,150"));
    }
}
