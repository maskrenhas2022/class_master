use std::io::Write;

use rpassword::prompt_password;

 pub fn exibir_menu(titulo: &str, itens: &[&str], sair: bool)-> u32{
    limpar_tela();

    let completo: String= String::from("ClassMaster Rust::") + titulo;

    println!("{}", completo);
    println!("{}", String::from("=").repeat(completo.len()));  



    exibir_itens(itens);

  println!("{}", if sair {"* - Sair" } else { "* - Voltar" });
  print! ("\nEscolha uma opcão: ");

  std::io::stdout().flush().unwrap();

 let mut linha: String = String::new();

  std::io::stdin().read_line(&mut linha).unwrap();  
    return 10;
}


fn exibir_itens(itens: &[&str]){
    for (i, item) in itens.iter().enumerate() {
    println!("{} - {}", i + 1, item);
}
}



 pub fn esperar_enter(){
    prompt_password("Pressione Enter para continuar").unwrap();
}


pub fn limpar_tela(){
    print!("{esc}c", esc =27 as char);
}