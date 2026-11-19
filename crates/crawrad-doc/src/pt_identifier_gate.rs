//! Portuguese Identifier and Path Ratchet Gate (ADR 0030)
//!
//! Scans all Rust source files in `crates/` and `components/` (skipping `target`, `.git`, `i18n`, `vendor`)
//! to enforce a ratchet on Portuguese identifier definitions (fn, struct, enum, mod, trait, type,
//! const, static, let/let mut, struct fields) and Portuguese directory/file names.

use std::path::Path;

/// Maximum allowed Portuguese identifier definitions across the workspace.
/// Must equal the exact actual count; if the count decreases, this ratchet constant must be lowered in the same PR.
pub const MAX_PT_IDENTIFIERS: usize = 2_141;

/// Maximum allowed Portuguese directory and file paths across `crates/` and `components/`.
pub const MAX_PT_PATHS: usize = 10;

/// Known Portuguese sub-words used to match code identifier tokens (snake_case or CamelCase).
pub const PT_WORDS: &[&str] = &[
    "aberta",
    "aberto",
    "abertura",
    "abrir",
    "acao",
    "acoes",
    "adicionar",
    "afastar",
    "alterar",
    "alternar",
    "alvo",
    "alvos",
    "apagar",
    "aparencia",
    "aplicar",
    "aproximar",
    "arquivo",
    "arquivos",
    "arranjo",
    "arvore",
    "arvores",
    "atalho",
    "atalhos",
    "atualizar",
    "autor",
    "avancado",
    "aviso",
    "banco",
    "bloco",
    "boas",
    "borda",
    "buscar",
    "cabecalho",
    "caixa",
    "caminho",
    "caminhos",
    "campo",
    "campos",
    "carregar",
    "casca",
    "categoria",
    "categorias",
    "cem",
    "centro",
    "chave",
    "chaves",
    "cheia",
    "cheio",
    "codigo",
    "colecao",
    "colecoes",
    "coluna",
    "comando",
    "componente",
    "componentes",
    "comportamento",
    "comprimento",
    "conectar",
    "contador",
    "conteudo",
    "contexto",
    "conversao",
    "converter",
    "cor",
    "cores",
    "corpo",
    "criacao",
    "criar",
    "dados",
    "decoracao",
    "depuracao",
    "depurador",
    "depurar",
    "desatrelar",
    "desconectar",
    "desenho",
    "desenhos",
    "desfazer",
    "despachante",
    "destravar",
    "dica",
    "dicas",
    "direita",
    "direito",
    "documento",
    "documentos",
    "dono",
    "donos",
    "edicao",
    "efeito",
    "efeitos",
    "elemento",
    "encerrar",
    "encontrar",
    "entrante",
    "entrar",
    "erro",
    "erros",
    "escrita",
    "esquema",
    "esquemas",
    "esquerda",
    "estado",
    "estados",
    "estilo",
    "evento",
    "eventos",
    "excluir",
    "execucao",
    "executar",
    "extensao",
    "falha",
    "fechada",
    "fechado",
    "fechamento",
    "fechar",
    "fila",
    "filho",
    "filhos",
    "fonte",
    "forma",
    "formas",
    "formulario",
    "formularios",
    "fundo",
    "gerador",
    "gerar",
    "gerenciador",
    "gesto",
    "gestos",
    "grade",
    "gravador",
    "gravar",
    "grupo",
    "grupos",
    "habilitada",
    "habilitado",
    "habilitar",
    "hierarquica",
    "hierarquico",
    "historico",
    "icone",
    "icones",
    "identidade",
    "identificacao",
    "imagem",
    "imagens",
    "impressao",
    "iniciar",
    "inicio",
    "injetar",
    "inspecionar",
    "inspetor",
    "instalacao",
    "instalacoes",
    "instalado",
    "instalados",
    "instalar",
    "invisivel",
    "janela",
    "janelas",
    "ladrilho",
    "largura",
    "legado",
    "legenda",
    "leitor",
    "leitura",
    "ligacao",
    "ligar",
    "limite",
    "limpa",
    "limpar",
    "limpo",
    "linha",
    "linhas",
    "lista",
    "listas",
    "livro",
    "lugar",
    "mais",
    "mapa",
    "marca",
    "marcada",
    "marcado",
    "margem",
    "margens",
    "mascara",
    "matricial",
    "membro",
    "membros",
    "mensagem",
    "mensagens",
    "modelo",
    "modelos",
    "modo",
    "modos",
    "mostrar",
    "mover",
    "mudar",
    "multilinha",
    "nada",
    "nascido",
    "nativa",
    "nativo",
    "navegador",
    "nenhum",
    "nenhuma",
    "nome",
    "nomes",
    "norma",
    "normalizar",
    "nos",
    "nota",
    "notas",
    "nova",
    "novas",
    "novo",
    "novos",
    "observador",
    "obter",
    "opcao",
    "opcaoes",
    "opcoes",
    "operacao",
    "operacoes",
    "ordem",
    "ordens",
    "origem",
    "outra",
    "outras",
    "outro",
    "outros",
    "pacote",
    "pacotes",
    "pagina",
    "paginas",
    "pai",
    "paineis",
    "painel",
    "pais",
    "palavra",
    "paleta",
    "paletas",
    "parada",
    "parametro",
    "parametros",
    "parar",
    "passar",
    "passo",
    "passos",
    "pausa",
    "pausar",
    "peca",
    "pecas",
    "pedir",
    "pilha",
    "plana",
    "planas",
    "ponto",
    "pontos",
    "porta",
    "portas",
    "posicao",
    "posicoes",
    "pousar",
    "pouso",
    "predicado",
    "prefixo",
    "previa",
    "problema",
    "problemas",
    "processar",
    "processo",
    "projeto",
    "projetos",
    "propriedade",
    "propriedades",
    "quadro",
    "quadros",
    "raiz",
    "raizes",
    "reabrir",
    "recarregar",
    "recuperar",
    "recusa",
    "recusado",
    "rede",
    "redesenhar",
    "refazer",
    "referencia",
    "referencias",
    "regiao",
    "registro",
    "registros",
    "remocao",
    "remover",
    "renomear",
    "reordenar",
    "resposta",
    "resumo",
    "rodape",
    "rodar",
    "rotulo",
    "rotulos",
    "saida",
    "salvamento",
    "salvar",
    "selecao",
    "selecionada",
    "selecionado",
    "selecionar",
    "servico",
    "servidor",
    "sessao",
    "sinal",
    "soltar",
    "sucesso",
    "suja",
    "sujo",
    "tabela",
    "tabelas",
    "tamanho",
    "tecla",
    "teclado",
    "teclas",
    "tela",
    "telas",
    "tema",
    "temas",
    "tempo",
    "texto",
    "textos",
    "tipo",
    "tipos",
    "titulo",
    "titulos",
    "todas",
    "todos",
    "topo",
    "trabalho",
    "transacao",
    "transf",
    "transferencia",
    "trava",
    "travar",
    "troca",
    "trocar",
    "ultima",
    "ultimo",
    "unidade",
    "usuario",
    "validacao",
    "validar",
    "valor",
    "valores",
    "varredura",
    "varrer",
    "vazia",
    "vazio",
    "verificar",
    "visao",
    "visitante",
    "visivel",
    "vista",
    "vistas",
    "viva",
    "vivo",
    "voltar",
    "volume",
    "zoom",
];

/// Strips comments (`//...` and `/* ... */`) and string literals (`"..."`, `r#"..."#`, `'...'`) from Rust source.
pub fn strip_comments_and_strings(code: &str) -> String {
    let mut out = String::with_capacity(code.len());
    let chars: Vec<char> = code.chars().collect();
    let n = chars.len();
    let mut i = 0;

    while i < n {
        // Line comment
        if chars[i] == '/' && i + 1 < n && chars[i + 1] == '/' {
            while i < n && chars[i] != '\n' {
                i += 1;
            }
            if i < n {
                out.push('\n');
                i += 1;
            }
            continue;
        }

        // Block comment
        if chars[i] == '/' && i + 1 < n && chars[i + 1] == '*' {
            i += 2;
            let mut depth = 1;
            while i < n && depth > 0 {
                if chars[i] == '/' && i + 1 < n && chars[i + 1] == '*' {
                    depth += 1;
                    i += 2;
                } else if chars[i] == '*' && i + 1 < n && chars[i + 1] == '/' {
                    depth -= 1;
                    i += 2;
                } else {
                    if chars[i] == '\n' {
                        out.push('\n');
                    }
                    i += 1;
                }
            }
            continue;
        }

        // Raw string r#"..."#
        if chars[i] == 'r' && (i + 1 < n && (chars[i + 1] == '#' || chars[i + 1] == '"')) {
            i += 1; // skip 'r'
            let mut hashes = 0;
            while i < n && chars[i] == '#' {
                hashes += 1;
                i += 1;
            }
            if i < n && chars[i] == '"' {
                i += 1;
                while i < n {
                    if chars[i] == '"' {
                        let mut match_hashes = 0;
                        while i + 1 + match_hashes < n
                            && chars[i + 1 + match_hashes] == '#'
                            && match_hashes < hashes
                        {
                            match_hashes += 1;
                        }
                        if match_hashes == hashes {
                            i += 1 + hashes;
                            break;
                        }
                    }
                    if chars[i] == '\n' {
                        out.push('\n');
                    }
                    i += 1;
                }
                out.push(' ');
                continue;
            }
        }

        // String literal "..."
        if chars[i] == '"' {
            i += 1;
            while i < n {
                if chars[i] == '\\' && i + 1 < n {
                    i += 2;
                } else if chars[i] == '"' {
                    i += 1;
                    break;
                } else {
                    if chars[i] == '\n' {
                        out.push('\n');
                    }
                    i += 1;
                }
            }
            out.push(' ');
            continue;
        }

        // Char literal 'c'
        if chars[i] == '\'' {
            out.push(' ');
            i += 1;
            while i < n && chars[i] != '\'' && chars[i] != '\n' {
                if chars[i] == '\\' && i + 1 < n {
                    i += 2;
                } else {
                    i += 1;
                }
            }
            if i < n && chars[i] == '\'' {
                i += 1;
            }
            continue;
        }

        out.push(chars[i]);
        i += 1;
    }

    out
}

/// Splits an identifier into sub-words (handling snake_case and CamelCase) and checks if any sub-word matches Portuguese.
pub fn is_pt_identifier(ident: &str) -> Option<&'static str> {
    if ident.is_empty() || (ident.starts_with('_') && ident.trim_matches('_').is_empty()) {
        return None;
    }

    // `PortaDb` is a product name, not the Portuguese word `porta` (door).
    let ident = ident.replace("PortaDb", "Portadb");

    // Split by underscore first
    for part in ident.split('_') {
        if part.is_empty() {
            continue;
        }

        // Split CamelCase in part
        let mut sub_words = Vec::new();
        let mut current = String::new();

        for c in part.chars() {
            if c.is_uppercase() && !current.is_empty() {
                sub_words.push(current.to_lowercase());
                current = String::new();
            }
            current.push(c);
        }
        if !current.is_empty() {
            sub_words.push(current.to_lowercase());
        }

        for word in sub_words {
            if (word.len() >= 3 || word == "e" || word == "do" || word == "da" || word == "em")
                && let Ok(idx) = PT_WORDS.binary_search(&word.as_str())
            {
                return Some(PT_WORDS[idx]);
            }
        }
    }

    None
}

/// A definition site with PT word.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PtDefinition {
    pub file: String,
    pub line: usize,
    pub kind: String,
    pub identifier: String,
    pub matched_pt_word: String,
}

/// Helper to test if string is a valid Rust identifier token
fn is_valid_ident(s: &str) -> bool {
    let mut chars = s.chars();
    if let Some(first) = chars.next() {
        (first.is_alphabetic() || first == '_') && chars.all(|c| c.is_alphanumeric() || c == '_')
    } else {
        false
    }
}

/// Extracts identifier definitions (`fn`, `struct`, `enum`, `mod`, `trait`, `type`, `const`, `static`, `let`, fields) from stripped Rust code.
pub fn scan_pt_definitions(file_path: &str, code: &str) -> Vec<PtDefinition> {
    let stripped = strip_comments_and_strings(code);
    let mut results = Vec::new();

    let keywords = [
        "fn", "struct", "enum", "mod", "trait", "type", "const", "static", "let",
    ];

    let mut in_struct = false;

    for (line_idx, line) in stripped.lines().enumerate() {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let num_tokens = tokens.len();

        for i in 0..num_tokens {
            let token = tokens[i];

            if token == "struct" {
                in_struct = true;
            } else if token == "}" {
                in_struct = false;
            }

            if keywords.contains(&token) {
                let kind = token;
                let mut target_idx = i + 1;

                // Handle `let mut` or `pub fn` or `pub(crate) struct`
                if kind == "let" && target_idx < num_tokens && tokens[target_idx] == "mut" {
                    target_idx += 1;
                }

                if target_idx < num_tokens {
                    let raw_ident = tokens[target_idx];
                    let ident = raw_ident.trim_matches(|c: char| !c.is_alphanumeric() && c != '_');

                    if is_valid_ident(ident)
                        && let Some(matched) = is_pt_identifier(ident)
                    {
                        results.push(PtDefinition {
                            file: file_path.to_string(),
                            line: line_idx + 1,
                            kind: kind.to_string(),
                            identifier: ident.to_string(),
                            matched_pt_word: matched.to_string(),
                        });
                    }
                }
            } else if in_struct
                && token.contains(':')
                && !token.starts_with(':')
                && !token.ends_with("::")
                && !token.contains("::")
            {
                // Struct field declaration: `field_name: Type`
                let parts: Vec<&str> = token.split(':').collect();
                let ident = parts[0].trim_matches(|c: char| !c.is_alphanumeric() && c != '_');

                if is_valid_ident(ident)
                    && ident != "pub"
                    && ident != "mut"
                    && ident != "self"
                    && ident != "ref"
                    && let Some(matched) = is_pt_identifier(ident)
                {
                    results.push(PtDefinition {
                        file: file_path.to_string(),
                        line: line_idx + 1,
                        kind: "struct field".to_string(),
                        identifier: ident.to_string(),
                        matched_pt_word: matched.to_string(),
                    });
                }
            }
        }
    }

    results
}

/// Recursively scans workspace `crates/` and `components/` for PT identifiers and PT paths.
pub fn scan_workspace_pt_gate(root_dir: &Path) -> (Vec<PtDefinition>, Vec<String>) {
    let mut pt_defs = Vec::new();
    let mut pt_paths = Vec::new();

    let scan_dirs = ["crates", "components"];

    for scan_dir_name in scan_dirs {
        let dir_path = root_dir.join(scan_dir_name);
        if dir_path.is_dir() {
            walk_dir_for_pt(&dir_path, root_dir, &mut pt_defs, &mut pt_paths);
        }
    }

    (pt_defs, pt_paths)
}

fn walk_dir_for_pt(
    dir: &Path,
    root_dir: &Path,
    pt_defs: &mut Vec<PtDefinition>,
    pt_paths: &mut Vec<String>,
) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        // Skip ignored directories
        if name == "target" || name == ".git" || name == "i18n" || name == "vendor" {
            continue;
        }

        let rel_path = path
            .strip_prefix(root_dir)
            .unwrap_or(&path)
            .display()
            .to_string()
            .replace('\\', "/");

        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");

        if is_pt_identifier(stem).is_some() {
            pt_paths.push(rel_path.clone());
        }

        if path.is_dir() {
            walk_dir_for_pt(&path, root_dir, pt_defs, pt_paths);
        } else if path.is_file()
            && path.extension().is_some_and(|ext| ext == "rs")
            && let Ok(content) = std::fs::read_to_string(&path)
        {
            let defs = scan_pt_definitions(&rel_path, &content);
            pt_defs.extend(defs);
        }
    }
}

#[cfg(test)]
#[path = "test/pt_identifier_gate/test.rs"]
mod tests;
