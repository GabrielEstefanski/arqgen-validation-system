use crate::models::empreendimento::Empreendimento;
use std::error::Error;
use std::fs::File;
use std::io::Write;

pub fn write_xml(path: &str, empreendimentos: &[Empreendimento]) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(path)?;

    writeln!(file, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>")?;
    writeln!(file, "<empreendimentos>")?;

    for empreendimento in empreendimentos {
        writeln!(file, "  <empreendimento>")?;
        writeln!(file, "    <construtora>{}</construtora>", empreendimento.construtora)?;
        writeln!(file, "    <cidade>{}</cidade>", empreendimento.cidade)?;
        writeln!(file, "    <area_do_terreno>{}</area_do_terreno>", empreendimento.area_do_terreno)?;
        writeln!(file, "    <numero_de_torres>{}</numero_de_torres>", empreendimento.numero_de_torres)?;
        writeln!(file, "    <altura_da_torre>{}</altura_da_torre>", empreendimento.altura_da_torre)?;
        writeln!(file, "    <area_da_torre>{}</area_da_torre>", empreendimento.area_da_torre)?;

        if let Some(area_lazer) = empreendimento.area_de_lazer {
            writeln!(file, "    <area_de_lazer>{}</area_de_lazer>", area_lazer)?;
        } else {
            writeln!(file, "    <area_de_lazer>0</area_de_lazer>")?;
        }
        
        writeln!(file, "  </empreendimento>")?;
    }
    
    writeln!(file, "</empreendimentos>")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    
    #[test]
    fn test_write_xml() {
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
        
        let result = write_xml(path, &empreendimentos);
        assert!(result.is_ok());

        let content = std::fs::read_to_string(path).unwrap();
        assert!(content.contains("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
        assert!(content.contains("<empreendimentos>"));
        assert!(content.contains("<construtora>Teste</construtora>"));
        assert!(content.contains("<cidade>São Paulo</cidade>"));
        assert!(content.contains("<area_do_terreno>1000</area_do_terreno>"));
    }
}
