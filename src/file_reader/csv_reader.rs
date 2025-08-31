use crate::models::empreendimento::Empreendimento;
use csv::ReaderBuilder;
use std::error::Error;

pub fn read_csv(path: &str) -> Result<Vec<Empreendimento>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(path)?;
    let mut results = Vec::new();

    for record in rdr.deserialize() {
        let empreendimento: Empreendimento = record?;
        results.push(empreendimento);
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_temp_csv(content: &str) -> NamedTempFile {
        let mut temp_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(temp_file, "{}", content).unwrap();
        temp_file
    }

    #[test]
    fn test_read_csv_kebab_case() {
        let csv_content = "construtora,cidade,area-do-terreno,numero-de-torres,altura-da-torre,area-da-torre,area-de-lazer\nAlpha,São Paulo,1000.0,2,25.0,300.0,150.0";
        
        let temp_file = create_temp_csv(csv_content);
        let result = read_csv(temp_file.path().to_str().unwrap());
        
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
    fn test_read_csv_snake_case() {
        let csv_content = "construtora,cidade,area_do_terreno,numero_de_torres,altura_da_torre,area_da_torre,area_de_lazer\nBeta,Rio de Janeiro,800.0,3,30.0,250.0,100.0";
        
        let temp_file = create_temp_csv(csv_content);
        let result = read_csv(temp_file.path().to_str().unwrap());
        
        assert!(result.is_ok());
        let empreendimentos = result.unwrap();
        assert_eq!(empreendimentos.len(), 1);
        
        let emp = &empreendimentos[0];
        assert_eq!(emp.construtora, "Beta");
        assert_eq!(emp.cidade, "Rio de Janeiro");
        assert_eq!(emp.area_do_terreno, 800.0);
        assert_eq!(emp.numero_de_torres, 3);
        assert_eq!(emp.altura_da_torre, 30.0);
        assert_eq!(emp.area_da_torre, 250.0);
        assert_eq!(emp.area_de_lazer, Some(100.0));
    }

    #[test]
    fn test_read_csv_without_area_de_lazer() {
        let csv_content = "construtora,cidade,area-do-terreno,numero-de-torres,altura-da-torre,area-da-torre\nDelta,Salvador,600.0,1,20.0,500.0";
        
        let temp_file = create_temp_csv(csv_content);
        let result = read_csv(temp_file.path().to_str().unwrap());
        
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
