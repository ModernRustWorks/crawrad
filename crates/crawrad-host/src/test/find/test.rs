//! Finding and replacing in a text.

use super::*;

#[test]
fn busca_encontra_ocorrencias_e_navega() {
    let texto = "fn teste() {\n    let x = 10;\n    let y = 10;\n}";
    let mut find = EditorFind::default();
    find.abrir(false, Some("10".to_string()), texto);

    assert_eq!(find.ocorrencias.len(), 2);
    assert_eq!(find.contagem_texto(), "1/2");
    assert_eq!(find.ocorrencias[0], (1, 12));
    assert_eq!(find.ocorrencias[1], (2, 12));

    // Next occurrence
    let p = find.proxima();
    assert_eq!(p, Some((2, 12)));
    assert_eq!(find.contagem_texto(), "2/2");

    // Wrap around to the start
    let p = find.proxima();
    assert_eq!(p, Some((1, 12)));
    assert_eq!(find.contagem_texto(), "1/2");

    // Previous
    let p = find.anterior();
    assert_eq!(p, Some((2, 12)));
    assert_eq!(find.contagem_texto(), "2/2");
}

#[test]
fn substituicao_atual_e_tudo() {
    let texto = "foo bar foo";
    let mut find = EditorFind::default();
    find.abrir(true, Some("foo".to_string()), texto);
    find.substituicao = "baz".to_string();

    assert_eq!(find.ocorrencias.len(), 2);

    // Replace first
    let (novo_texto, pos) = find.substituir_atual(texto).expect("substituir");
    assert_eq!(novo_texto, "baz bar foo");
    assert_eq!(pos, (0, 0));
    assert_eq!(find.ocorrencias.len(), 1);

    // Replace all
    let (tudo, total) = find.substituir_tudo("foo bar foo");
    assert_eq!(tudo, "baz bar baz");
    assert_eq!(total, 1); // find had 1 remaining occurrence from the last count
}
