//! **Find & replace in the editor: the model** — the occurrences of a term
//! in a text, the current one, and replacing one or all. Pure; the floating
//! bar that drives it is the shell's.

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct EditorFind {
    pub aberto: bool,
    pub modo_substituir: bool,
    pub busca: String,
    pub substituicao: String,
    pub ocorrencias: Vec<(usize, usize)>, // (linha, byte_inicio)
    pub indice_atual: Option<usize>,
}

impl EditorFind {
    pub fn abrir(&mut self, modo_substituir: bool, termo_inicial: Option<String>, texto: &str) {
        self.aberto = true;
        self.modo_substituir = modo_substituir;
        if let Some(t) = termo_inicial
            && !t.is_empty()
        {
            self.busca = t;
        }
        self.atualizar(texto);
    }

    pub fn fechar(&mut self) {
        self.aberto = false;
    }

    pub fn alternar_modo_substituir(&mut self) {
        self.modo_substituir = !self.modo_substituir;
    }

    pub fn atualizar(&mut self, texto: &str) {
        self.ocorrencias.clear();
        if self.busca.is_empty() {
            self.indice_atual = None;
            return;
        }

        for (l_idx, linha) in texto.lines().enumerate() {
            let mut start = 0;
            while start < linha.len() {
                if let Some(pos) = linha[start..].find(&self.busca) {
                    let col = start + pos;
                    self.ocorrencias.push((l_idx, col));
                    start = col + self.busca.len().max(1);
                } else {
                    break;
                }
            }
        }

        if self.ocorrencias.is_empty() {
            self.indice_atual = None;
        } else if self
            .indice_atual
            .is_none_or(|i| i >= self.ocorrencias.len())
        {
            self.indice_atual = Some(0);
        }
    }

    pub fn proxima(&mut self) -> Option<(usize, usize)> {
        if self.ocorrencias.is_empty() {
            return None;
        }
        let prox = match self.indice_atual {
            Some(i) => (i + 1) % self.ocorrencias.len(),
            None => 0,
        };
        self.indice_atual = Some(prox);
        Some(self.ocorrencias[prox])
    }

    pub fn anterior(&mut self) -> Option<(usize, usize)> {
        if self.ocorrencias.is_empty() {
            return None;
        }
        let prev = match self.indice_atual {
            Some(i) if i > 0 => i - 1,
            _ => self.ocorrencias.len() - 1,
        };
        self.indice_atual = Some(prev);
        Some(self.ocorrencias[prev])
    }

    pub fn contagem_texto(&self) -> String {
        if self.busca.is_empty() {
            "0/0".to_string()
        } else if self.ocorrencias.is_empty() {
            "Sem resultados".to_string()
        } else {
            let atual = self.indice_atual.map_or(0, |i| i + 1);
            format!("{atual}/{}", self.ocorrencias.len())
        }
    }

    pub fn substituir_atual(&mut self, texto: &str) -> Option<(String, (usize, usize))> {
        if self.busca.is_empty() || self.ocorrencias.is_empty() {
            return None;
        }
        let idx = self.indice_atual?;
        let (linha_idx, byte_idx) = self.ocorrencias.get(idx).copied()?;

        let mut linhas: Vec<String> = texto.lines().map(|s| s.to_string()).collect();
        if linha_idx >= linhas.len() {
            return None;
        }
        let linha = &linhas[linha_idx];
        if byte_idx + self.busca.len() > linha.len() {
            return None;
        }

        let mut nova_linha = String::new();
        nova_linha.push_str(&linha[..byte_idx]);
        nova_linha.push_str(&self.substituicao);
        nova_linha.push_str(&linha[byte_idx + self.busca.len()..]);
        linhas[linha_idx] = nova_linha;

        let novo_texto = linhas.join("\n");
        self.atualizar(&novo_texto);
        Some((novo_texto, (linha_idx, byte_idx)))
    }

    pub fn substituir_tudo(&mut self, texto: &str) -> (String, usize) {
        if self.busca.is_empty() {
            return (texto.to_string(), 0);
        }
        let count = self.ocorrencias.len();
        let novo_texto = texto.replace(&self.busca, &self.substituicao);
        self.atualizar(&novo_texto);
        (novo_texto, count)
    }
}

#[cfg(test)]
#[path = "test/find/test.rs"]
mod tests;
