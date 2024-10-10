// Disciplina : Linguagem e Lógica de Programação
// Professor : Alan Furukawa
// Descrição : Aqui você descreve o que o programa faz! (função)
// Autor(a) : Gabriel Aguiar Rocha
// Data atual : 04/10/2021
use std::io;

fn ler_numero() -> f64 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a linha");

    input.trim().parse().unwrap_or_else(|_| {
        println!("Entrada inválida: por favor, digite um número.");
        ler_numero() // Chamar a função novamente para solicitar uma nova entrada
    })
}

fn main() {
    const NUMERO_DE_NOTAS: usize = 3;
    const FREQUENCIA_MINIMA: f64 = 0.75;
    const MEDIA_MINIMA_APROVADO: f64 = 6.0;

    let mut notas = [0.0; NUMERO_DE_NOTAS];
    let mut soma = 0.0;

    for i in 0..NUMERO_DE_NOTAS {
        println!("Digite a nota {}: ", i);
        notas[i] = ler_numero();
        soma += notas[i];
    }

    println!("Digite sua frequência em porcentagem:");
    let frequencia = ler_numero() / 100.0;

    let media = soma / NUMERO_DE_NOTAS as f64;

    if media >= MEDIA_MINIMA_APROVADO && frequencia >= FREQUENCIA_MINIMA {
        println!("Aprovado");
    } else if media < MEDIA_MINIMA_APROVADO && frequencia >= FREQUENCIA_MINIMA {
        println!("Está de prova final");
    } else if frequencia < FREQUENCIA_MINIMA {
        println!("Reprovado por falta");
    }
}