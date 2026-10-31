pub struct Permissao {
    pub leitura: bool,
    pub escrita: bool,
    pub execucao: bool,
}

pub struct Arquivo {
    pub nome: String,
    pub tamanho: u64,
    pub permissoes: (Permissao, Permissao, Permissao),
    pub uid_dono: u16,
    pub gid_grupo: u16,
}

pub struct Diretorio {
    pub nome: String,
    pub arquivos: Vec<Arquivo>,
    pub permissoes: (Permissao, Permissao, Permissao),
    pub uid_dono: u16,
}

pub struct Usuario {
    pub nome: String,
    pub uid: u16,
    pub grupos_ids: Vec<u16>,
}

pub struct Grupo {
    pub nome: String,
    pub gid: u16,
    pub membros_uids: Vec<u16>,
}

impl Permissao {
    pub fn new(leitura: bool, escrita: bool, execucao: bool) -> Permissao {
        Permissao { leitura, escrita, execucao }
    }

    pub fn octal(&self) -> u8 {
        let read = if self.leitura { 4 } else { 0 };
        let write = if self.escrita { 2 } else { 0 };
        let exe = if self.execucao { 1 } else { 0 };
        read + write + exe
    }

    pub fn formatar(&self) -> String {
        let read = if self.leitura { "r" } else { "-" };
        let write = if self.escrita { "w" } else { "-" };
        let exe = if self.execucao { "x" } else { "-" };
        format!("{read}{write}{exe}")
    }
}

impl Arquivo {
    pub fn new(nome: String, tamanho: u64, uid_dono: u16, gid_grupo: u16) -> Arquivo {

        let permissoes = (
            Permissao::new(true, true, false),
            Permissao::new(true, true, false),
            Permissao::new(true, false, false),
        );

        Arquivo { nome, tamanho, permissoes, uid_dono, gid_grupo }
    }

    pub fn alterar_permissao(&mut self, permissoes: (Permissao, Permissao, Permissao)) {
        self.permissoes = permissoes;
    }

    pub fn stat(&self) {
        println!("===================================");
        println!("Arquivo:       {}", self.nome);
        println!("Tamanho:       {} bytes", self.tamanho);
        println!(
            "Permissoes:    ({}{}{} / {}{}{})",
            self.permissoes.0.octal(), self.permissoes.1.octal(), self.permissoes.2.octal(),
            self.permissoes.0.formatar(), self.permissoes.1.formatar(), self.permissoes.2.formatar()
        );
        println!("Uid:           {}", self.uid_dono);
        println!("Gid:           {}", self.gid_grupo);
        println!("===================================");
    }
}

impl Diretorio {
    pub fn new(nome: String, permissoes: (Permissao, Permissao, Permissao), uid_dono: u16) -> Diretorio {
        Diretorio { nome, arquivos: Vec::new(), permissoes, uid_dono }
    }

    pub fn adiciona_arquivo(&mut self, arquivo: Arquivo) {
        self.arquivos.push(arquivo);
    }

    pub fn remove_arquivo(&mut self, nome_arquivo: String) {
        let mut no_such = true;
        
        for (posicao, arquivo) in self.arquivos.iter().enumerate() {
            if arquivo.nome == nome_arquivo {
                self.arquivos.remove(posicao);
                no_such = false;
                println!("Arquivo '{}' removido do diretório!", nome_arquivo);
                break;
            }
        }
        if no_such {
            println!("No such file or directory: {}", nome_arquivo);
        }
    }

    pub fn listar_conteudo(&self) {
        println!("===================================");
        println!("Diretório '{}':", self.nome);
        println!("Dono (UID):    {}", self.uid_dono);
        println!(
            "Permissoes:    ({}{}{} / {}{}{})",
            self.permissoes.0.octal(), self.permissoes.1.octal(), self.permissoes.2.octal(),
            self.permissoes.0.formatar(), self.permissoes.1.formatar(), self.permissoes.2.formatar()
        );
        for arquivo in self.arquivos.iter() {
            println!(" - {}", arquivo.nome);
        }
        println!("===================================");
    }
}

impl Usuario {
    pub fn new(nome: String, uid: u16) -> Usuario {
        Usuario {
            nome,
            uid,
            grupos_ids: Vec::new(),
        }
    }

    pub fn adiciona_grupo(&mut self, gid: u16) {
        self.grupos_ids.push(gid);
        println!("Grupo GID {} associado ao usuário '{}'!", gid, self.nome);
    }

    pub fn remove_grupo(&mut self, gid: u16) {
        let mut achou = false;
        for (i, &g) in self.grupos_ids.iter().enumerate() {
            if g == gid {
                self.grupos_ids.remove(i);
                achou = true;
                println!("Grupo GID {} removido do usuário!", gid);
                break;
            }
        }
        if !achou {
            println!("Grupo GID {} não encontrado no usuário!", gid);
        }
    }

    pub fn listar_grupos(&self) {
        println!("===================================");
        println!("Grupos do Usuário '{}' (UID: {}):", self.nome, self.uid);
        for gid in self.grupos_ids.iter() {
            println!(" - GID: {}", gid);
        }
        println!("===================================");
    }
}

impl Grupo {
    pub fn new(nome: String, gid: u16) -> Grupo {
        Grupo {
            nome,
            gid,
            membros_uids: Vec::new(),
        }
    }

    pub fn adiciona_membro(&mut self, uid: u16) {
        self.membros_uids.push(uid);
        println!("Membro UID {} adicionado ao grupo '{}'!", uid, self.nome);
    }

    pub fn remove_membro(&mut self, uid: u16) {
        let mut achou = false;
        for (i, &u) in self.membros_uids.iter().enumerate() {
            if u == uid {
                self.membros_uids.remove(i);
                achou = true;
                println!("Membro UID {} removido do grupo!", uid);
                break;
            }
        }
        if !achou {
            println!("Membro UID {} não encontrado no grupo!", uid);
        }
    }

    pub fn listar_membros(&self) {
        println!("===================================");
        println!("Membros do Grupo '{}' (GID: {}):", self.nome, self.gid);
        for uid in self.membros_uids.iter() {
            println!(" - UID: {}", uid);
        }
        println!("===================================");
    }
}