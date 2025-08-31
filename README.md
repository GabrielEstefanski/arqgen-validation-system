# 🏗️ ArqGen - Sistema de Validação de Empreendimentos

Sistema robusto e escalável para validação de empreendimentos imobiliários com regras de negócio flexíveis e modulares.

## 📋 Descrição do Sistema

O ArqGen é um sistema que lê arquivos de empreendimentos em múltiplos formatos (JSON, CSV, XML, Parquet) e aplica regras de validação de negócio configuráveis por cidade e construtora.

### 📊 Dados dos Empreendimentos

Cada empreendimento contém:
- **construtora**: Nome da construtora
- **cidade**: Localização do empreendimento
- **area-do-terreno**: Área total em m²
- **numero-de-torres**: Quantidade de torres
- **altura-da-torre**: Altura de cada torre em m
- **area-da-torre**: Área de cada torre em m²
- **area-de-lazer**: Área de lazer em m² (opcional)

## 🏛️ Arquitetura do Sistema

### Estrutura de Pastas

```
src/
├── business_logic/          # Lógica de negócio e regras
│   ├── regras/              # Implementações das regras
│   │   ├── padrao.rs        # Regras padrão do sistema
│   │   ├── cidades.rs       # Regras específicas por cidade
│   │   ├── construtoras.rs  # Regras específicas por construtora
│   │   ├── ignoradas.rs     # Regras que podem ser ignoradas
│   │   ├── factory.rs       # Fábrica de regras
│   │   └── mod.rs           # Módulo de regras
│   ├── validator.rs         # Validador principal
│   └── mod.rs               # Módulo de lógica de negócio
├── file_reader/             # Leitura de arquivos
├── file_generator/          # Geração de arquivos
├── models/                  # Modelos de dados
└── utils/                   # Utilitários
```

## 🎯 Sistema de Regras de Negócio

### 🔧 Arquitetura das Regras

O sistema utiliza o padrão **Strategy** com uma hierarquia flexível de regras:

```rust
pub trait RegraNegocio {
    fn nome(&self) -> &'static str;
    fn aplicar(&self, empreendimento: &Empreendimento) -> Result<(), String>;
}
```

### 📋 Regras Padrão

1. **RegraAlturaMax**: Altura máxima de 30m para todas as torres
2. **RegraAreaTorresMax**: Área total das torres deve ser < 80% do terreno
3. **RegraAreaLazerMin**: Com 2+ torres, área de lazer deve ser ≥ 10% do terreno

### 🏙️ Regras por Cidade

#### Rio de Janeiro
- ✅ Regra 1: Altura < 30m
- ✅ Regra 2: Área torres < 80% do terreno
- ❌ Regra 3: Área de lazer (não aplica)

#### São Paulo
- ❌ Regra 1: Altura (não aplica)
- ✅ Regra 2: Área torres < 80% do terreno
- ✅ Regra 3: Área de lazer ≥ 10% do terreno

#### Boituva
- **RegraMaxTorres**: Máximo de 5 torres por terreno

#### Guaratinguetá
- **RegraAlturaPorTorresGuaratingueta**: Altura limitada por número de torres
  - 1-2 torres: 25m
  - 3 torres: 20m
  - 4+ torres: 15m

### 🏢 Regras por Construtora

#### Alpha
- **RegraAreaLazerAlpha**: Sempre deve ter área de lazer ≥ 10% do terreno

### 🚫 Sistema de Regras Ignoradas

O sistema permite que certas regras sejam **ignoradas** para cidades específicas, oferecendo flexibilidade adicional:

#### Rio de Janeiro
- **Ignora**: `RegraAreaLazerMin` (não exige área de lazer)
- **Aplica**: `RegraAlturaMax` e `RegraAreaTorresMax`

#### São Paulo  
- **Ignora**: `RegraAlturaMax` (não limita altura das torres)
- **Aplica**: `RegraAreaTorresMax` e `RegraAreaLazerMin`

#### Como Funciona
```rust
// src/business_logic/regras/ignoradas.rs
lazy_static! {
    pub static ref REGRAS_IGNORADAS_POR_CIDADE: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("Rio de Janeiro", vec!["RegraAreaLazerMin"]);
        m.insert("São Paulo", vec!["RegraAlturaMax"]);
        m
    };
}
```

#### Benefícios
- **Flexibilidade**: Cada cidade pode ter suas próprias exceções
- **Manutenibilidade**: Fácil configuração sem alterar regras existentes
- **Escalabilidade**: Novas cidades podem ignorar regras específicas

## 🚀 Como Escalar as Regras

### 1. 📝 Criando uma Nova Regra

#### Passo 1: Implementar a Trait
```rust
// src/business_logic/regras/minha_regra.rs
use super::padrao::RegraNegocio;
use crate::models::empreendimento::Empreendimento;

pub struct MinhaNovaRegra {
    pub parametro: f64,
}

impl RegraNegocio for MinhaNovaRegra {
    fn nome(&self) -> &'static str {
        "MinhaNovaRegra"
    }

    fn aplicar(&self, empreendimento: &Empreendimento) -> Result<(), String> {
        // Lógica da regra aqui
        if empreendimento.altura_da_torre > self.parametro {
            Err(format!("Altura {}m excede o limite de {}m", 
                empreendimento.altura_da_torre, self.parametro))
        } else {
            Ok(())
        }
    }
}
```

#### Passo 2: Adicionar ao Módulo
```rust
// src/business_logic/regras/mod.rs
pub mod minha_regra;
pub use minha_regra::MinhaNovaRegra;
```

#### Passo 3: Registrar na Factory
```rust
// src/business_logic/regras/factory.rs
impl RegrasFactory {
    pub fn por_cidade(cidade: &str) -> Vec<Box<dyn RegraNegocio>> {
        match cidade {
            "MinhaCidade" => vec![Box::new(MinhaNovaRegra { parametro: 25.0 })],
            // ... outras cidades
            _ => vec![],
        }
    }
}
```

### 2. 🏭 Adicionando Nova Cidade

```rust
// src/business_logic/regras/factory.rs
pub fn por_cidade(cidade: &str) -> Vec<Box<dyn RegraNegocio>> {
    match cidade {
        "NovaCidade" => vec![
            Box::new(RegraAlturaMax(25.0)),        // Altura específica
            Box::new(RegraAreaTorresMax(0.7)),     // Limite específico
        ],
        // ... outras cidades
        _ => vec![],
    }
}
```

### 3. 🏢 Adicionando Nova Construtora

```rust
// src/business_logic/regras/factory.rs
pub fn por_construtora(construtora: &str) -> Vec<Box<dyn RegraNegocio>> {
    match construtora {
        "NovaConstrutora" => vec![
            Box::new(RegraAreaLazerMin(0.15)),    // 15% mínimo
            Box::new(RegraAlturaMax(20.0)),        // Altura limitada
        ],
        // ... outras construtoras
        _ => vec![],
    }
}
```

### 4. 🔄 Combinando Regras

O sistema automaticamente combina:
- Regras padrão (sempre aplicam)
- Regras da cidade (se existirem)
- Regras da construtora (se existirem)

```rust
let todas_regras: Vec<Box<dyn RegraNegocio>> = RegrasFactory::padrao()
    .into_iter()
    .chain(RegrasFactory::por_cidade(&empreendimento.cidade))
    .chain(RegrasFactory::por_construtora(&empreendimento.construtora))
    .collect();
```

### 5. 🚫 Adicionando Regras Ignoradas

Para fazer uma cidade ignorar uma regra específica:

```rust
// src/business_logic/regras/ignoradas.rs
lazy_static! {
    pub static ref REGRAS_IGNORADAS_POR_CIDADE: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("Rio de Janeiro", vec!["RegraAreaLazerMin"]);
        m.insert("São Paulo", vec!["RegraAlturaMax"]);
        m.insert("NovaCidade", vec!["RegraAreaTorresMax"]); // Nova cidade ignorando regra
        m
    };
}
```

#### Exemplo de Uso
```rust
// Uma cidade que não quer limitar a área das torres
m.insert("CidadeSemLimite", vec!["RegraAreaTorresMax"]);

// Uma cidade que não quer regras de altura
m.insert("CidadeSemAltura", vec!["RegraAlturaMax"]);

// Uma cidade que ignora múltiplas regras
m.insert("CidadeFlexivel", vec!["RegraAlturaMax", "RegraAreaLazerMin"]);
```

## 🧪 Como Testar

### 1. 🚀 Executando Todos os Testes
```bash
cargo test
```

### 2. 🎯 Testando Regras Específicas
```bash
# Testes de regras de cidades
cargo test test_regra_max_torres
cargo test test_regra_altura_por_torres_guaratingueta

# Testes de regras de construtoras
cargo test test_construtora_alpha
cargo test test_regra_area_lazer_alpha

# Testes de integração
cargo test test_integracao_completa_regras_negocio
```

### 3. 📝 Criando Novos Testes

#### Teste de Regra Individual
```rust
#[test]
fn test_minha_nova_regra() {
    let regra = MinhaNovaRegra { parametro: 25.0 };
    
    // Teste de sucesso
    let empreendimento_valido = Empreendimento {
        altura_da_torre: 20.0,
        // ... outros campos
    };
    assert!(regra.aplicar(&empreendimento_valido).is_ok());
    
    // Teste de falha
    let empreendimento_invalido = Empreendimento {
        altura_da_torre: 30.0,
        // ... outros campos
    };
    assert!(regra.aplicar(&empreendimento_invalido).is_err());
}
```

#### Teste de Integração
```rust
#[test]
fn test_integracao_nova_cidade() {
    let empreendimento = Empreendimento {
        cidade: "NovaCidade".to_string(),
        // ... outros campos
    };
    
    let regras = RegrasFactory::por_cidade(&empreendimento.cidade);
    assert_eq!(regras.len(), 2); // Deve ter 2 regras específicas
    
    // Testar aplicação das regras
    for regra in &regras {
        let resultado = regra.aplicar(&empreendimento);
        // Verificar se a regra foi aplicada corretamente
    }
}
```

## 📊 Estrutura de Testes

```
tests/
├── common/                   # 🛠️ Utilitários de teste
│   └── mod.rs              # Funções auxiliares para testes
├── business_logic_integration.rs  # 🧪 Testes de integração das regras
└── integration_tests.rs     # 🔗 Testes de integração do sistema
```

### 🎯 Tipos de Teste

1. **Testes Unitários**: Cada regra individual
2. **Testes de Integração**: Combinação de regras
3. **Testes de Cenários**: Casos de borda e extremos
4. **Testes de Performance**: Múltiplos empreendimentos

## 🔧 Configuração e Uso

### 1. 🚀 Executando o Sistema
```bash
# Validação de empreendimentos
cargo run -- --path dados.json

# Geração de todos os formatos
cargo run --bin generate_files
```

### 2. 📁 Formatos Suportados
- **Entrada**: JSON, CSV, XML, Parquet
- **Saída**: JSON, CSV, XML, Parquet

### 3. ⚙️ Configuração de Regras
As regras são configuradas diretamente no código através da `RegrasFactory`, permitindo:
- Fácil modificação de parâmetros
- Adição/remoção de regras
- Configuração específica por cidade/construtora

## 🚀 Benefícios da Arquitetura

### ✅ **Escalabilidade**
- Fácil adição de novas regras
- Configuração flexível por cidade/construtora
- Sistema modular e extensível

### ✅ **Manutenibilidade**
- Código limpo e bem estruturado
- Separação clara de responsabilidades
- Testes abrangentes

### ✅ **Flexibilidade**
- Regras podem ser combinadas dinamicamente
- Suporte a múltiplos formatos de arquivo
- Configuração específica por contexto

### ✅ **Testabilidade**
- Arquitetura orientada a testes
- Mocks e fixtures disponíveis
- Cobertura completa de cenários

---

**ArqGen** - Sistema de validação de empreendimentos escalável e flexível 🏗️✨
