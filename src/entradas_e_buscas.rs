use std::io;
use crate::estruturas::{ Arquivo, Diretorio, Grupo, Usuario };

// ENTRADAS

pub fn ler_u16() -> u16 {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Erro ao ler");
    buffer.trim().parse().expect("Digite um número válido")
}

pub fn ler_string() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Erro ao ler");
    buffer.trim().to_string()
}

pub fn ler_u64() -> u64 {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Erro ao ler");
    buffer.trim().parse().expect("Digite um número válido")
}

// aceita s ou 1 como sim, qualquer outra resposta conta como nao
pub fn ler_bool() -> bool {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Erro ao ler");
    let entrada = buffer.trim().to_lowercase();

    if entrada == "s" || entrada == "1" {
        return true;
    } else {
        return false;
    }
}

// BUSCA

pub fn busca_usuario(uid: u16, usuarios: &Vec<Usuario>) -> (bool, usize) {
    for (i, usuario) in usuarios.iter().enumerate() {
        if usuario.uid == uid {
            return (true, i);
        }
    }  (false, 0)
}

pub fn busca_grupo(gid: u16, grupos: &Vec<Grupo>) -> (bool, usize) {
    for (i, grupo) in grupos.iter().enumerate() {
        if grupo.gid == gid {
            return (true, i);
        }
    }  (false, 0)
}

pub fn busca_arquivo(nome: &str, arquivos: &Vec<Arquivo>) -> (bool, usize) {
    for (i, arquivo) in arquivos.iter().enumerate() {
        if arquivo.nome == nome {
            return (true, i);
        }
    } (false, 0)
}

pub fn busca_dir(nome: &str, diretorios: &Vec<Diretorio>) -> (bool, usize) {
    for (i, dir) in diretorios.iter().enumerate() {
        if dir.nome == nome {
            return (true, i);
        }
    } (false, 0)
}