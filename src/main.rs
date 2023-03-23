use num::pow::checked_pow;
use rand::Rng;
use std::io;
use std::char;
use std::io::Write;
use std::process::exit;
use rand::prelude::IteratorRandom;
use strsim::hamming;

#[derive(Debug, Clone)]
struct Word {
    alvo: String,
    texto: String,
}

struct Mutation {
    index: u32,
    swap_char: char
}

impl Word {
    fn new(alvo: &String, texto: &String) -> Result<Self, &'static str> {
        if alvo.chars().count() != texto.chars().count() {
            return Err("Quantidade de caracteres não é a mesma");
        } else if alvo.len() <= 0 {
            return Err("");
        } else {
            return Ok(Word {
                alvo: alvo.to_owned(),
                texto: texto.to_owned()
            });
        }
    }

    fn mutate(&self) -> Self {
        let symbols = "\"\'`/? !@#$%*()-_=+;:~[]{}.,><|";
        let numbers = "1234567890";
        let lowercase = "abcdefghijklmnopqrstuvwxyz";
        let uppercase = lowercase.to_ascii_uppercase();

        let all_chars = format!("{symbols}{numbers}{uppercase}{lowercase}");

        let mut mutations: Vec<Mutation> = Vec::new();
        //let mutation_count: u32 = (self.texto.chars().count() as f64 / 2.0).ceil() as u32;
        //let mutation_count = rand::thread_rng().gen_range(1..=self.texto.chars().count());
        let mutation_count = 1;

        // Create the random mutations
        for _ in 0..mutation_count {
            mutations.append(
                &mut vec![Mutation {
                    index: rand::thread_rng()
                        .gen_range(0..=(self.alvo.chars().count() - 1))
                        .try_into()
                        .expect("Sampling error"),
                    swap_char: all_chars.chars().choose(&mut rand::thread_rng()).unwrap()
                }]
            );
        }

        let mutated_str = self.texto.chars().enumerate().map( |ch| {
            let index = ch.0 as u32;
            let mut return_char = ch.1;

            for m in &mutations {
                if index == m.index {
                    return_char = m.swap_char
                }
            }

            return return_char;
        }).collect::<String>();

        return Self {
            texto: mutated_str,
            alvo: self.alvo.clone()
        }
    }

    fn bitcmp(&self) -> usize {
        return hamming(&self.alvo, &self.texto).unwrap()
    }

    fn match_alvo(&self) -> bool {
        if self.alvo == self.texto {
            return true
        }

        return false
    }
}

fn main() {
    let mut obj = String::new();
    let in_matriz: String;
    let mut gens = 1;

    // Ler o objetivo
    println!("Simulador de evolução");
    print!("Digite um objetivo: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut obj)
        .expect("Falha ao ler");

    // Ler a matriz
    /* 
    print!("Digite uma matriz: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut in_matriz)
        .expect("Falha ao ler");
    */

    // Remove espaçoes do final
    //in_matriz = in_matriz.replace("\n", "");
    obj = obj.replace("\n", "");
    in_matriz = std::iter::repeat("X").take(obj.chars().count()).collect::<String>();

    // Sai do programa se as entradas forem iguais
    if in_matriz == obj {
        exit(0);
    }

    let mut reprodutor = Word::new(&obj, &in_matriz).unwrap();

    loop {
        let mut is_complete = false;
        //println!("Geração {gens} com {} indivíduos", count);

        // Number of individuals created on this generations
        let count = checked_pow(2 as u16, gens).unwrap_or(u16::MAX);
        //let count: u128 = gens;
        //let count: u128 = pow(2, gens);

        // Create mutations
        for _ in 1..=count{
            let mutated = reprodutor.mutate();

            if mutated.match_alvo() {
                is_complete = true;
                reprodutor = mutated;
                break;
            } else if mutated.bitcmp() < reprodutor.bitcmp() {
                reprodutor = mutated;
                break;
            }   
        }


        gens += 1;
        println!("Ger. {gens} <{count} ind.> -> Similaridade +{} | Melhor \"{}\"", reprodutor.bitcmp(), reprodutor.texto);

        // Finish loop if both sentences match
        if is_complete {
            break;
        }
    }

    println!("Finalizado na geração {gens}")
}
