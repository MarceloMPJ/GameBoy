extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
  let numero_secreto = rand::thread_rng().gen_range(0..100);

  println!("Adivinhe o Número!");
  println!("Digite um número:");

  let mut palpite = String::new();

  io::stdin().read_line(&mut palpite)
    .expect("Falha a ler a entrada!");

  let palpite: u32 = palpite.trim().parse()
    .expect("Por favor, digite um número!");

  println!("Voce digitou: {}", palpite);
  println!("O numero secreto foi: {}", numero_secreto);

  match palpite.cmp(&numero_secreto) {
    Ordering::Less => println!("Muito baixo!"),
    Ordering::Greater => println!("Muito alto!"),
    Ordering::Equal => println!("Voce acertou!"),
  }
}
