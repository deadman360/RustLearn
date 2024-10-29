use std::time::Instant;
fn main() {
    let mut lista_ordenada: Vec<i64> = Vec::new();
    let mut contador: i64 = 0;
    while contador < 4493970 {
        lista_ordenada.push(contador);
        contador += 2;
        println!("{}", contador);
    }
}

fn pesquisa_binaria(target: i64, lista: &mut Vec<i64>) {
    if target < lista[lista.len() / 2] {
        let _ = lista.split_off(lista.len() / 2);
        pesquisa_binaria(target, lista);
    } else if target > lista[lista.len() / 2] {
        let mut tail: Vec<i64> = lista.split_off(lista.len() / 2);
        pesquisa_binaria(target, &mut tail);
    }
}
