use std::io::{self, Write};
use std::fs;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Regra {
    premissas: Vec<String>,
    conclusao: String,
}

fn ler_entrada(mensagem: &str) -> String {
    print!("{}", mensagem);
    io::stdout().flush().unwrap();
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Falha ao ler");
    entrada.trim().to_string()
}

fn perguntar_usuario(fato: &str) -> bool {
    let resposta = ler_entrada(&format!("\n--- Pergunta: Isso está acontecendo: '{}'? (s/n): ", fato));
    resposta.to_lowercase() == "s"
}

// ... (Função provar continua a mesma lógica da v2.1)
fn provar(objetivo: &str, fatos: &mut Vec<String>, regras: &Vec<Regra>) -> bool {
    if fatos.contains(&objetivo.to_string()) { return true; }
    for regra in regras {
        if regra.conclusao == objetivo {
            let mut todas_provadas = true;
            for premissa in &regra.premissas {
                if !provar(premissa, fatos, regras) {
                    todas_provadas = false;
                    break;
                }
            }
            if todas_provadas {
                fatos.push(objetivo.to_string());
                return true;
            }
        }
    }
    if perguntar_usuario(objetivo) {
        fatos.push(objetivo.to_string());
        return true;
    }
    false
}

fn main() {
    println!("\n=== MOTOR DE INFERÊNCIA POLIGLOTA (JSON/YAML) ===");

    let nome_arquivo = ler_entrada("Digite o nome do arquivo (ex: regras.json ou regras.yaml): ");
    let conteudo = fs::read_to_string(&nome_arquivo)
        .unwrap_or_else(|_| panic!("❌ Arquivo '{}' não encontrado!", nome_arquivo));

    // A MÁGICA ACONTECE AQUI:
    let regras: Vec<Regra> = if nome_arquivo.ends_with(".yaml") || nome_arquivo.ends_with(".yml") {
        println!("📂 Detectado formato YAML...");
        serde_yaml::from_str(&conteudo).expect("❌ Erro no formato YAML!")
    } else {
        println!("📂 Detectado formato JSON...");
        serde_json::from_str(&conteudo).expect("❌ Erro no formato JSON!")
    };

    println!("✅ {} regras carregadas com sucesso!", regras.len());
    let objetivo = ler_entrada("O que deseja provar? ");
    let mut fatos_conhecidos = Vec::new();

    if provar(&objetivo, &mut fatos_conhecidos, &regras) {
        println!("\n✅ CONCLUSÃO: '{}' é VERDADEIRO!", objetivo);
    } else {
        println!("\n❌ CONCLUSÃO: Não foi possível provar '{}'.", objetivo);
    }
}