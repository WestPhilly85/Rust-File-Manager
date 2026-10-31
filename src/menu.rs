use crate::entradas_e_buscas::{ ler_u16, ler_u64, ler_string, ler_bool, busca_arquivo, busca_dir, busca_usuario, busca_grupo };

use crate::estruturas::{ Arquivo, Diretorio, Grupo, Permissao, Usuario };

fn percorre_dir(nome: &str, diretorios: &Vec<Diretorio>) -> (bool, usize, usize) {
    for (i, dir) in diretorios.iter().enumerate() {

        for (j, arq) in dir.arquivos.iter().enumerate() {
            if arq.nome == nome {
                return (true, i, j);
            }
        }
    } return (false, 0, 0)
}

fn interface_permissao() -> (Permissao, Permissao, Permissao) {

    println!("Permissões Dono - Leitura (s/n):");
    let r1 = ler_bool();
    println!("Permissões Dono - Escrita (s/n):");
    let w1 = ler_bool();
    println!("Permissões Dono - Execução (s/n):");
    let x1 = ler_bool();

    println!("Permissões Grupo - Leitura (s/n):");
    let r2 = ler_bool();
    println!("Permissões Grupo - Escrita (s/n):");
    let w2 = ler_bool();
    println!("Permissões Grupo - Execução (s/n):");
    let x2 = ler_bool();

    println!("Permissões Outros - Leitura (s/n):");
    let r3 = ler_bool();
    println!("Permissões Outros - Escrita (s/n):");
    let w3 = ler_bool();
    println!("Permissões Outros - Execução (s/n):");
    let x3 = ler_bool();

    ( Permissao::new(r1, w1, x1), Permissao::new(r2, w2, x2), Permissao::new(r3, w3, x3) )
}

pub fn menu() {
    // cada estrutura tem o seu vetor aqui no main, e cada funcao recebe so a parte que precisa usar
    let mut usuarios: Vec<Usuario> = Vec::new();
    let mut grupos: Vec<Grupo> = Vec::new();
    let mut arquivos: Vec<Arquivo> = Vec::new();
    let mut diretorios: Vec<Diretorio> = Vec::new();

    loop {
        println!("\n=== Gerenciador de Arquivo ===");
        println!("1. Criar Usuário");
        println!("2. Criar Grupo");
        println!("3. Criar Arquivo");
        println!("4. Criar Diretório");
        println!("5. Gerenciar Usuário (Adicionar/Remover/Listar Grupos)");
        println!("6. Gerenciar Grupo (Adicionar/Remover/Listar Membros)");
        println!("7. Gerenciar Diretório (Adicionar/Remover/Listar Arquivos)");
        println!("8. Alterar Permissão de Arquivo");
        println!("9. Exibir Status do Arquivo (Stat)");
        println!("0. Sair");
        println!("\nEscolha uma opção:");

        let opcao = ler_string();

        match opcao.as_str() {

            "1" => {
                println!("--- Novo Usuário ---");
                println!("Digite o nome:");
                let nome = ler_string();

                println!("Digite o UID:");
                let mut uid = ler_u16();

                let mut found = busca_usuario(uid, &usuarios).0;

                while found {
                    println!("\nAVISO: Usuário já existente!");
                    println!("Digite o UID:");
                    uid = ler_u16();
                    found = busca_usuario(uid, &usuarios).0;
                }

                usuarios.push(Usuario::new(nome.clone(), uid));
                println!("Usuário '{}' salvo com sucesso!", nome);
            }

            "2" => {
                println!("--- Novo Grupo ---");
                println!("Digite o nome do grupo:");
                let nome_gp = ler_string();

                println!("Digite o GID:");
                let mut gid = ler_u16();

                let mut found = busca_grupo(gid, &grupos).0;

                while found {
                    println!("\nAVISO: Grupo já existente!");
                    println!("Digite o GID:");
                    gid = ler_u16();
                    found = busca_grupo(gid, &grupos).0;
                }

                grupos.push(Grupo::new(nome_gp.clone(), gid));
                println!("Grupo '{}' salvo com sucesso!", nome_gp);
            }
            
            "3" => {
                println!("--- Novo Arquivo ---");
                println!("Digite o nome do arquivo:");
                let mut nome_arq = ler_string();

                let mut found = busca_arquivo(&nome_arq, &arquivos).0;

                if !found {
                    found = percorre_dir(&nome_arq, &diretorios).0;
                }

                while found {
                    println!("\nAVISO: Arquivo já existente com esse nome!");
                    println!("Digite o Nome:");
                    nome_arq = ler_string();
                    found = busca_arquivo(&nome_arq, &arquivos).0;

                    if !found {
                        found = percorre_dir(&nome_arq, &diretorios).0;
                    }
                }

                println!("Digite o tamanho:");
                let tamanho = ler_u64();

                println!("Digite o UID do dono:");
                let uid = ler_u16();

                println!("Digite o GID do grupo:");
                let gid_gp = ler_u16();

                let novo_arq = Arquivo::new(nome_arq.clone(), tamanho, uid, gid_gp);
                arquivos.push(novo_arq);
                println!("Arquivo '{}' salvo com sucesso!", nome_arq);
            }

            "4" => {
                println!("--- Novo Diretório ---");
                println!("Digite o nome do diretório:");
                let mut nome_dir = ler_string();

                let mut found = busca_dir(&nome_dir, &diretorios).0;

                while found {
                    println!("\nAVISO: Diretório já existente com esse nome!");
                    println!("Digite o nome:");
                    nome_dir = ler_string();
                    found = busca_dir(&nome_dir, &diretorios).0;
                }

                println!("Digite o UID do dono:");
                let uid = ler_u16();

                let perms = interface_permissao();

                let novo_dir = Diretorio::new(nome_dir.clone(), perms, uid);
                diretorios.push(novo_dir);
                println!("Diretório '{}' salvo com sucesso!", nome_dir);
            }

            "5" => {
                println!("--- Gerenciar Grupos do Usuário ---");
                println!("Digite o UID do usuário:");
                let uid = ler_u16();

                let (found, posicao) = busca_usuario(uid, &usuarios);

                if found {
                    let user = &mut usuarios[posicao];

                    println!("1. Adicionar GID | 2. Remover GID | 3. Listar Grupos");
                    let sub = ler_string();

                    match sub.as_str() {
                        
                        "1" => {
                            println!("Digite o GID para adicionar:");
                            user.adiciona_grupo(ler_u16());
                        }

                        "2" => {
                            println!("Digite o GID para remover:");
                            user.remove_grupo(ler_u16());
                        }

                        "3" => user.listar_grupos(),
                        _ => println!("Opção inválida!"),
                    }

                } else {
                    println!("Usuário com UID {} não encontrado!", uid);
                }
            }

            "6" => {
                println!("--- Gerenciar Membros do Grupo ---");
                println!("Digite o GID do grupo:");
                let gid = ler_u16();

                let (found, posicao) = busca_grupo(gid, &grupos);

                if found {
                    let grupo = &mut grupos[posicao];

                    println!("1. Adicionar UID | 2. Remover UID | 3. Listar Membros");
                    let sub = ler_string();

                    match sub.as_str() {
                        "1" => {
                            println!("Digite o UID para adicionar:");
                            grupo.adiciona_membro(ler_u16());
                        }
                        "2" => {
                            println!("Digite o UID para remover:");
                            grupo.remove_membro(ler_u16());
                        }
                        "3" => grupo.listar_membros(),
                        _ => println!("Opção inválida!"),
                    }

                } else {
                    println!("Grupo com GID {} não encontrado!", gid);
                }
            }

            "7" => {
                println!("--- Gerenciar Conteúdo do Diretório ---");
                println!("Digite o nome do diretório:");
                let nome_dir = ler_string();

                let (found, posicao) = busca_dir(&nome_dir, &diretorios);

                if found {
                    let dir = &mut diretorios[posicao];
                    
                    println!("1. Adicionar Arquivo Existente | 2. Remover Arquivo | 3. Listar Conteúdo");

                    let sub = ler_string();

                    match sub.as_str() {

                        "1" => {
                            println!("Digite o nome do arquivo a adicionar:");
                            let nome_arq = ler_string();

                            let (found, pos_arq) = busca_arquivo(&nome_arq, &arquivos);

                            if found {
                                let arq = arquivos.remove(pos_arq);

                                diretorios[posicao].adiciona_arquivo(arq);
                                println!("Arquivo adicionado ao diretório!");

                            } else {
                                let (achou, pos_dir, pos_arq) = percorre_dir(&nome_arq, &diretorios);

                                if achou {
                                    let arq = diretorios[pos_dir].arquivos.remove(pos_arq);

                                    diretorios[posicao].adiciona_arquivo(arq);
                                    println!("Arquivo adicionado ao diretório!");

                                } else {
                                    println!("Arquivo '{}' não encontrado!", nome_arq);
                                }
                            }
                        }

                        "2" => {
                            println!("Digite o nome do arquivo a remover:");
                            dir.remove_arquivo(ler_string());
                        }

                        "3" => dir.listar_conteudo(),
                        _ => println!("Opção inválida!"),
                    }

                } else {
                    println!("Diretório não encontrado!");
                }
            }

            "8" => {
                println!("--- Alterar Permissão de Arquivo ---");
                println!("Digite o nome do arquivo:");

                let nome_arq = ler_string();

                let (found, posicao) = busca_arquivo(&nome_arq, &arquivos);

                if found {
                    let arq = &mut arquivos[posicao];
                    
                    let perms = interface_permissao();

                    arq.alterar_permissao(perms);
                    println!("Permissões de '{}' alteradas!", nome_arq);

                } else {
                    let (achou, pos_dir, pos_arq) = percorre_dir(&nome_arq, &diretorios);

                    if achou {
                        let arq = &mut diretorios[pos_dir].arquivos[pos_arq];

                        let perms = interface_permissao();

                        arq.alterar_permissao(perms);
                        println!("Permissões de '{}' alteradas!", nome_arq);

                    } else {
                        println!("Arquivo '{}' não encontrado!", nome_arq);
                    }
                }
            }

            "9" => {
                println!("--- Consultar Stat ---");
                println!("Qual arquivo deseja consultar?");

                let stat_busca = ler_string();

                let (found, posicao) = busca_arquivo(&stat_busca, &arquivos);

                if found {
                    let arq = &arquivos[posicao];

                    arq.stat();

                } else {
                    let (achou, pos_dir, pos_arq) = percorre_dir(&stat_busca, &diretorios);

                    if achou {
                        diretorios[pos_dir].arquivos[pos_arq].stat();
                    } else {
                        println!("Arquivo '{}' não encontrado!", stat_busca);
                    }
                }
            }

            "0" => {
                println!("Saindo do sistema...");
                break;
            }

            _ => println!("Opção inválida! Tente novamente."),
        }
    }
}