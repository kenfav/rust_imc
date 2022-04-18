use std::io;

struct Cliente {
    nome: String,
    idade: u8,
    peso: f32,
    altura: f32,
    imc: f32
}

impl Cliente {
    pub fn new(nome: String, idade: u8, peso: f32, altura: f32) -> Self {
        let imc_calculado: f32 = peso/(altura * altura);
        Self {
            nome,
            idade,
            peso,
            altura,
            imc: imc_calculado           
        }       
    }
}

fn main() {

menu_principal();
    
}

fn input_usuario() -> (String, u8, f32, f32) {
    //nome do cliente
    println!("Digite o nome: ");
    let mut nome = String::new();
    io::stdin().read_line(&mut nome).expect("Digite o nome no formato correto");

    //idade do cliente
    println!("Digite a idade: ");
    let mut idade = String::new();
    io::stdin().read_line(&mut idade).expect("Digite a idade no formato correto");

    //altura do cliente
    println!("Digite a altura: ");
    let mut altura = String::new();
    io::stdin().read_line(&mut altura).expect("Digite a altura no formato correto");

    //peso do cliente
    println!("Digite o peso: ");
    let mut peso = String::new();
    io::stdin().read_line(&mut peso).expect("Digite o peso no formato correto");

    //retorno da funcao
    (nome, 
    idade.trim().parse::<u8>().unwrap(),
    peso.trim().parse::<f32>().unwrap(),
    altura.trim().parse::<f32>().unwrap())
}

fn tabela_imc(imc: f32) -> String {
    if imc < 18.0 {
        return "Abaixo do peso".to_string()
    } else if imc < 25.0 {
        return "Peso ideal".to_string()
    } else if imc < 30.0 {
        return "Sobrepeso".to_string()
    } else if imc < 35.0 {
        "Obesidade 1".to_string()
    } else if imc < 40.0 {
        "Obesidade 2".to_string()
    } else {
        "Obesidade morbida".to_string()
    }

}

fn menu_principal() {
    loop {
        let mut opcao_selecionada = String::new();
        mensagem_menu_principal();
        io::stdin().read_line(&mut opcao_selecionada).expect("Digita a opcao desejada de forma correta!");

        if opcao_selecionada.trim() == "2" {
            mensagem_despedida();
            break
        } else if opcao_selecionada.trim() == "1" {
            let cliente1 = cadastro_novo_paciente();
            mostrar_dados_do_paciente(cliente1);
        }
        else {
            mensagem_opcao_invalida();
        }
    }
}

fn mensagem_menu_principal() {
    println!("==================================================");
    println!("Bem vindo a clinica de nutricao Saude e Bem Estar!");
    println!("Digite a sua opcao: ");
    println!("(1) Nova consulta de IMC");
    println!("(2) Sair");
    println!("==================================================");
}

fn mostrar_dados_do_paciente(paciente: Cliente) {
    println!("=====================================");
    println!("\n\nNome do paciente: {}\nIdade: {} anos.\nAltura: {} mts.\nPeso: {} kgs.\nIMC: {:?}.",
    paciente.nome, paciente.idade, paciente.altura, paciente.peso, paciente.imc);
    println!("Segundo a tabela de IMC a sua situacao e de: {:?}",tabela_imc(paciente.imc));
    println!("Pressione ENTER para voltar ao menu principal");
    io::stdin().read_line(&mut String::new());
}

fn mensagem_opcao_invalida() {
    println!("\n\n\n\n\n");
    println!("*************************************");
    println!("----------OPCAO INVALIDA-------------");
    println!("Digite somente as opcoes disponiveis!");
    println!("\n\n\n\n\n");
}

fn mensagem_despedida() {
    println!("Clinica Saude e Bem estar agradece a sua visita. Volte sempre!");
}

fn cadastro_novo_paciente() -> Cliente {
    let dados: (String, u8, f32, f32) = input_usuario();
    let cliente1: Cliente = Cliente::new(dados.0, dados.1, dados.2, dados.3);
    cliente1
}