use crate::models::empreendimento::Empreendimento;
use serde_json;
use std::error::Error;
use std::fs::File;
use std::io::Write;

pub fn write_json(path: &str, empreendimentos: &[Empreendimento]) -> Result<(), Box<dyn Error>> {
    let json_string = serde_json::to_string_pretty(empreendimentos)?;
    let mut file = File::create(path)?;
    write!(file, "{}", json_string)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    
    #[test]
    fn test_write_json() {
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
        
        let result = write_json(path, &empreendimentos);
        assert!(result.is_ok());
            
        let content = std::fs::read_to_string(path).unwrap();
        assert!(content.contains("\"construtora\": \"Teste\""));
        assert!(content.contains("\"cidade\": \"São Paulo\""));
        assert!(content.contains("\"area-do-terreno\": 1000.0"));
    }
}
