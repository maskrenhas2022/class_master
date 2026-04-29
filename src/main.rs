mod utils;

// Importando as funções do seu módulo terminal
use utils::terminal::{esperar_enter, exibir_menu};

fn main() {
    // Removi o ": [&str; 5]" para o Rust inferir o tamanho automaticamente
    let itens = [
        "Fundamentos", 
        "Tipos", 
        "Controle", 
        "Funções", 
        "Ownership"
    ]; 

    // Note que agora passei 5 itens para combinar com o que você queria
    exibir_menu("Principal", &itens, true);
    esperar_enter();
}