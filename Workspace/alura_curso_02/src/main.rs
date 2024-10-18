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