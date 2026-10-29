struct Arquivo {
    nome: String,
    tamanho: u64,
    permissoes: (Permissao, Permissao, Permissao),
    uid_dono: u16,
    gid_grupo: u16
}

struct Diretorio {
    nome: String,
    arquivos: Vec<Arquivo>,
    permissoes: (Permissao, Permissao, Permissao),
    uid_dono: u16
}

struct Permissao {
    leitura: bool,
    escrita: bool,
    execucao: bool
}

impl Diretorio {
    pub fn new(nome: String, permissoes: (Permissao, Permissao, Permissao), uid_dono: u16) -> Diretorio {

        let arquivos: Vec<Arquivo> = Vec::new();

        Diretorio {nome, arquivos, permissoes, uid_dono}
    }

    pub fn adiciona_arquivo(&mut self, arquivo: Arquivo) {
        self.arquivos.push(arquivo);
    }

    pub fn remove_arquivo(&mut self, nome_arquivo: String) {

        let mut no_such: bool = true;

        for (posicao, arquivo) in self.arquivos.iter().enumerate() {
            if arquivo.nome == nome_arquivo {
                self.arquivos.remove(posicao);
                no_such = false;
                break
            }
        }

        if no_such {
            println!("No such file or directory: {}", nome_arquivo);
        }
    }

    pub fn listar_conteudo(&self) {
        println!("===================================");

        for arquivo in self.arquivos.iter() {
            println!("{}", arquivo.nome);
        }

        println!("===================================");
    }
}

impl Arquivo{
    pub fn new(nome: String, tamanho: u64, uid_dono: u16, gid_grupo: u16) -> Arquivo {

        let permissoes: (Permissao, Permissao, Permissao) = (
            Permissao::new(true, true, false), Permissao::new(true, true, false), Permissao::new(true, false, false)
        );

        Arquivo { nome, tamanho, permissoes, uid_dono, gid_grupo}
    }

    pub fn alterar_permissao(&mut self, permissoes: (Permissao, Permissao, Permissao)) {
        self.permissoes = permissoes;
    }

    pub fn stat(&self) {

        println!("===================================");
        println!("Arquivo:      {}", self.nome);
        println!("Tamanho:      {} bytes", self.tamanho);

        println!("Permissões:   ({}{}{} / {}{}{})",
            self.permissoes.0.octal(), self.permissoes.1.octal(), self.permissoes.2.octal(),
            self.permissoes.0.formatar(), self.permissoes.1.formatar(), self.permissoes.2.formatar() 
        );

        println!("Uid:          {}", self.uid_dono);
        println!("Gid:          {}", self.gid_grupo);
        println!("===================================");
    }
}

impl Permissao {
    pub fn new(leitura: bool, escrita: bool, execucao: bool) -> Permissao {
        Permissao { leitura, escrita, execucao}
    }

    pub fn octal(&self) -> u8 {

        let read = if self.leitura { 4 } else { 0 };
        let write = if self.escrita { 2 } else { 0 };
        let exe = if self.execucao { 1 } else { 0 };

        read + write + exe
    }

    pub fn formatar(&self) -> String {

        let read = if self.leitura {"r"} else {"-"};
        let write = if self.escrita {"w"} else {"-"};
        let exe = if self.execucao {"x"} else {"-"};

        format!("{read}{write}{exe}")
    }
}

fn main() {
    let mut teste = Arquivo::new("Teste".to_string(), 12, 1600, 3400);
    teste.stat();
    teste.alterar_permissao((Permissao::new(true, true, true), Permissao::new(true, true, false), Permissao::new(true, false, false)));
    teste.stat();
}
