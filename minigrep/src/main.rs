use std::env;
use std::process;


use minigrep::Config;

fn main() {


    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}"); //eprintln! permet de d'afficher des messages d'erreur à l'écran 
        process::exit(1);                               //et avec cargo run > output.txt, on affiche la sortie(notamment tous les println!) dans un fichier output.txt
    });

    /* 
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    */
    
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }


}