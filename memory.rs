// direitos autorais (Rust) DCrust 16/04/2026
// é resilientemente de todos mas seria em conceitos tecnicos
// qualquer um pode auditar o codigo pode pegar analisar mas baixar faça o que quiser mas use o kernel para fazer distribuições 
// como no linux  mas não é permitido baixar os repositorios para usar nos seus se for apenas de estudo

// Copyright (Rust) DCrust // 16/04/2026
// it is resilient for everyone but it would be in technical concepts anyone can audit the code can 
// get analyse but download do what you 
// want but use the kernel to make distributions like in linux but it is not allowed to 
// download the repositories to use in yours if it is only for study


use std::fs;
use tokio;

// Simulação de alocação de buffers na memória
fn main() {
    // RAM: 16 GB alocada como um grande vetor
    let ram: Vec<u8> = vec![0; 16 * 1024 * 1024 * 1024]; // 16 GB - cuidado, pode crashar

    // SSD: representado como um arquivo
    fs::write("temp_data.bin", [0u8; 32 * 1024 * 1024 * 1024]).expect("Falha ao escrever no SSD");

    // Cloud: representado como uma chamada assíncrona
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            println!("Enviando dados para a nuvem...");
            // Simulação de upload
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            println!("Dados enviados com segurança!");
        });

    // Empréstimo (borrow) de parte da RAM
    let chunk = &ram[0..100]; // empresta 100 bytes
    println!("Processando {} bytes da RAM", chunk.len());

    // Limpeza
    drop(ram);
    let _ = fs::remove_file("temp_data.bin");
}   
