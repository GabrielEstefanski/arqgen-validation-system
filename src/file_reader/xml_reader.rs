use crate::models::empreendimento::Empreendimento;
use serde_xml_rs::from_reader;
use std::error::Error;
use std::fs::File;
use serde::Deserialize;

pub fn read_xml(path: &str) -> Result<Vec<Empreendimento>, Box<dyn Error>> {
    let file = File::open(path)?;
    let wrapper: EmpreendimentosWrapper = from_reader(file)?;
    Ok(wrapper.empreendimentos)
}

#[derive(Debug, Deserialize)]
struct EmpreendimentosWrapper {
    #[serde(rename = "empreendimento")]
    pub empreendimentos: Vec<Empreendimento>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_temp_xml(content: &str) -> NamedTempFile {
        let mut temp_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(temp_file, "{}", content).unwrap();
        temp_file
    }

    #[test]
    fn test_read_xml_kebab_case() {
        let xml_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<empreendimentos>
    <empreendimento>
        <construtora>Alpha</construtora>
        <cidade>São Paulo</cidade>
        <area-do-terreno>1000.0</area-do-terreno>
        <numero-de-torres>2</numero-de-torres>
        <altura-da-torre>25.0</altura-da-torre>
        <area-da-torre>300.0</area-da-torre>
        <area-de-lazer>150.0</area-de-lazer>
    </empreendimento>
</empreendimentos>"#;
        
        let temp_file = create_temp_xml(xml_content);
        let result = read_xml(temp_file.path().to_str().unwrap());
        
        assert!(result.is_ok());
        let empreendimentos = result.unwrap();
        assert_eq!(empreendimentos.len(), 1);
        
        let emp = &empreendimentos[0];
        assert_eq!(emp.construtora, "Alpha");
        assert_eq!(emp.cidade, "São Paulo");
        assert_eq!(emp.area_do_terreno, 1000.0);
        assert_eq!(emp.numero_de_torres, 2);
        assert_eq!(emp.altura_da_torre, 25.0);
        assert_eq!(emp.area_da_torre, 300.0);
        assert_eq!(emp.area_de_lazer, Some(150.0));
    }

    #[test]
    fn test_read_xml_without_area_de_lazer() {
        let xml_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<empreendimentos>
    <empreendimento>
        <construtora>Delta</construtora>
        <cidade>Salvador</cidade>
        <area-do-terreno>600.0</area-do-terreno>
        <numero-de-torres>1</numero-de-torres>
        <altura-da-torre>20.0</altura-da-torre>
        <area-da-torre>500.0</area-da-torre>
    </empreendimento>
</empreendimentos>"#;
        
        let temp_file = create_temp_xml(xml_content);
        let result = read_xml(temp_file.path().to_str().unwrap());
        
        assert!(result.is_ok());
        let empreendimentos = result.unwrap();
        assert_eq!(empreendimentos.len(), 1);
        
        let emp = &empreendimentos[0];
        assert_eq!(emp.construtora, "Delta");
        assert_eq!(emp.cidade, "Salvador");
        assert_eq!(emp.area_do_terreno, 600.0);
        assert_eq!(emp.numero_de_torres, 1);
        assert_eq!(emp.altura_da_torre, 20.0);
        assert_eq!(emp.area_da_torre, 500.0);
        assert_eq!(emp.area_de_lazer, None);
    }
}
