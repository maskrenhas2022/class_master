mod utils;



use std::process::exit;

// Importando as funções do seu módulo terminal
use utils::terminal::{esperar_enter, exibir_menu};

fn main() {

     loop {
         
         // Removi o ": [&str; 5]" para o Rust inferir o tamanho automaticamente
    let itens = [
        "Fundamentos", 
        "Tipos", 
        "Controle", 
        "Funções", 
        "Ownership"
    ]; 
  
    // Note que agora passei 5 itens para combinar com o que você queria
    
 let _selecionado = exibir_menu("Principal", &itens, true);
    
    match _selecionado {
        1 => println!("1"),
        2 => println!("2"),
        3 => println!("3"),
        4 => println!("4"),
        5 => println!("5"),

        _ => exit(0),
    }
    
    esperar_enter();
}


     }





    