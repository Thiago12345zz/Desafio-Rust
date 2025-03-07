/*Imagine que você está desenvolvendo um sistema de atendimento para um banco. Nesse sistema, os clientes chegam e aguardam na fila para serem atendidos na ordem de chegada (modelo FIFO – First In, First Out). Como parte desse sistema, você precisa implementar a fila do zero. Essa implementação será utilizada para controlar o fluxo de atendimento dos clientes, garantindo que o primeiro a chegar seja o primeiro a ser atendido.*/

use std::collections::LinkedList;
use std::io::{self, Write};

fn main(){
    let mut fila : LinkedList<String> = LinkedList::new();
    let mut c = 1;
    



    while true {
        print!("Digite seu nome: ");

            io::stdout().flush().unwrap();
    
            let mut nome = String::new();
    
            io::stdin().read_line(&mut nome).unwrap();
    
            let nome = nome.trim().to_string();
    
            fila.push_back(nome);

            print!("Voce deseja continuar inserindo pessoas s/n: ");

            io::stdout().flush().unwrap();

            let mut pergunta = String::new();

            io::stdin().read_line(&mut pergunta).unwrap();

            let pergunta = pergunta.trim().to_string();
        
        if pergunta == "n" {
            break;
        }
    }

    println!("\n");
    println!("Resultado da fila de clientes seguindo o modelo (Fist iN, First out)(Primeiro a entar é o primeiro a sair): {:?}",fila);
    println!("\n");
    
    while let Some(p) = fila.pop_front() {
        print!("{:?}: ",c);
        println!("Chamando {:?}", p);
        c = c+1;
    }
}