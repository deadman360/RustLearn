fn main() {
    let array: [f32; 4] = [1.2,1.3,1.4,1.5]; // com a sintaxe [valor; qunatidade] posso fazer arrays de forma dinamica
    let indice: usize = 3; //para acessar valores de arrays com variaveis tipadas ou qualquer tipo de dado tipado ele necessita ser usize pois para acessar armazenamos endereços de memoria
    println!("array = {:#?}\n elementos separados:\n{:#?}, \n{:#?}, \n{:#?}, \n{:#?}\n\nElemento numero {} = {}",
    array,
    array[0],
    array[1],
    array[2],
    array[3],
    indice,
    array[indice]);
    println!("\n\nElementos por for loop ");
    //com loop for
    for i in 0..array.len(){
        println!(" Elemento {} = {}", i, array[i])
    }

    matriz();
    println!("É fim de semana ? {}", eh_fim_de_semana(DiaDaSemana::Domingo));
    println!("{}",cores());
    opcional();
    vectors();
    structs();

}

fn matriz(){
    let matriz:[[f32; 3]; 2] = [
        [0.1,0.2,0.3],
        [1.2,1.3,1.4]
    ];
    println!("matriz = {:#?}", matriz);
    for indice in 0..matriz.len() {
        for item in matriz[indice]{
            println!("item do array {} = {}",indice + 1 , item)
        }
    }
}

#[allow(dead_code)]
enum DiaDaSemana {
    Domingo,
    Segunda,
    Terca,
    Quarta,
    Quinta,
    Sexta,
    Sabado
}
#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    RGB(u8,u8,u8),
    Cymk{cyan: u8, yellow: u8, magenta: u8, black: u8}
}
fn eh_fim_de_semana(dia_da_semana: DiaDaSemana) -> bool{
    match dia_da_semana {
        DiaDaSemana::Sabado| DiaDaSemana::Domingo => true,
        _ => false
    }
}
fn cores() -> &'static str{
    let cor: Color = Color::Cymk { cyan: 40, yellow: 234, magenta: 43, black: 255};
    match cor {
        Color::Blue => "Azul",
        Color::Green => "Verde",
        Color::Red => "Vermelho",
        Color::RGB(0, 0, 0) | Color::Cymk{cyan: _, yellow: _, magenta: _, black: 255}=> "Preto",
        Color::RGB(_,_,_) => "RGB DESCONHECIDO",
        Color::Cymk{cyan: _, yellow: _, magenta: _, black: _} => "Cymk desconhecido"

    }
}

fn opcional(){
    let opcao: Option<&'static str> = opcao("QQRCOIDSA");
    if let Some(valor) = opcao  {           // if let é um eslito de match statment que podemos passar um padrao, nesse caso Some(valor) e um valor para match
        println!("{}", valor);
    } 
}
fn opcao(init: &'static str) -> Option<&'static str> {      //Option é um estilo de monada mas não é uma propriamente dita mas é um tipo em rust completamente similar a uma monada
    match init{
        "200:OK" => Some("Deu Certo"),
        "404:NOT_FOUND" => Some("Vish, Não encontramos sua Pagina "),
        _ => None
    }
}
// Options são enums gericos em rust Option<T> essa sintaxe <T> representa um tipo qualquer para fazer um template para quando informado o tipo
// como em Option<&'static str> o compilador faça uma enum com o tipo informado 
fn vectors(){
    //vetores já são alocados com certa quantidade de memória
    let mut novo_vetor: Vec<u8> = Vec::new();//vetores sempre devem ter o tipo especificado na sintaxe Vec<u8>
    novo_vetor.push(10);
    println!("{:?}", novo_vetor);
    novo_vetor.push(11);
    println!("{:?}", novo_vetor);

    let mut vetor_com_valores: Vec<u8> = vec![1,2,3,4,5,6,7,8,9,10];
    println!("{:?}",vetor_com_valores);
    println!("Valor 1 = {}", vetor_com_valores[0]); // para acessar valores em vectores pode ser usado indices mas como os valores são dinamicos um end de memoria de um vec pode estar na frente de um outro e isso pode dar problema 
    //podemos usar outros metodos para acessar valores, pois alem desse problema acima se eu tentar pegar um indice onde não existe um valor por exemplo 50 ele compila e da panic
    println!("Valor 50 = {}", match vetor_com_valores.get(50){      // nesse caso esse metodo .get retorna um option em vez de retornar diretamente um valor
        Some(valor) => *valor, //porem como o tipo option é um generic a variavel valor é uma referencia para o tipo do vetor entã o rust não copia o valor do vetor para a variavel e somente faz com que a variaavel aponte para o valor no vetor 
        None => 0 
    });
    println!("{:?}", vetor_com_valores.len());
    //metodo pop ele remove retorna o ultimo valor do vetor e remove ele do vetor, lembrando que ele retorna um opton entao use if let para acessar o valor
    /*
    while let Some(ultimo) = vetor_com_valores.pop(){
        println!("Valor removido {}", ultimo);
    }
    */
    // se for usado com metodo for ele pega o valor pois ele transforma o vec em um iterador, então é necessario usar a referencia para somente emprestar o valor do vetor para ele 
    for item in &vetor_com_valores{
        println!("{}", item)
    }
    println!("{:?}", vetor_com_valores);
    /*vetores tem uma capacidade ja determinada na heap quando criados, mesmo que sem nenhum valor, toda vez que a capacidade é excedida
    ele dobra a capacidade realocando a memoria, pois fazendo um por um ele teria que realocar a memoria a cada adição e alocar memoria na heap émuito caro */
    println!("{}", vetor_com_valores.capacity());
    vetor_com_valores.push(11);
    println!("{}", vetor_com_valores.capacity());
    // para criar um veto com capacidade especifica para evitar uso excessivo de memoria se usa a sintaxe Vec::with_capacity()
    let vetor_capacitado: Vec<i32> = Vec::with_capacity(11);
    println!("{}", vetor_capacitado.capacity());
}
struct Registrado {
    nome: &'static str,
    sobrenome: &'static str,
    idade: u8
}
struct Cliente {
    pessoa: Registrado,
    id: i32
}
impl Cliente{                   // impl Cliente diz que todas funções dentro desse mesmo bloco são métodos
    fn aniversario(&mut self, desconto: i32){
        println!("Ola CLiente {}, Voce recebeu um desconto de {}% na sua proxima compra, Feliz {} anos!", self.id, desconto, self.pessoa.idade+1);
        self.pessoa.idade += 1
    }        
    /*self é uma referencia ao valor que chamou esse metodo
    como em rust referencias são por padrão imutaveis se eu declarar
    o metodo com self sendo uma referencia mutavel como no caso acima
    nao poderei usar esse metodo para variaveis imutaveis */
}
fn structs(){
    let eu: Registrado = Registrado{
        nome: "Victor",
        sobrenome: "Morto",
        idade:254
    };
    let mut eu_cliente: Cliente = Cliente { pessoa: eu, id: 3452 };
    println!("Nome: {} {}, idade: {},ID: {} ", eu_cliente.pessoa.nome, eu_cliente.pessoa.sobrenome, eu_cliente.pessoa.idade, eu_cliente.id);
    eu_cliente.aniversario(20);
    println!("\n\n\n Cliente Idade == {} anos", eu_cliente.pessoa.idade);
    /*println!("\n\n\n Pessoa Idade == {}", eu.idade); esse codigo simplesmente nao roda
    porque ele por ownership o dono do valor e "eu" virou o eu_cliente, se eu tento implementar um
    ponteiro dentro da struct, ele nao entende pois a struct deixa implicito que recebe uma variavel e nao um ponteiro
    por isso tambem nao preciso declarar a varaivelk "eu" como mutavel*/
}

