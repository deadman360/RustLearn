static IDADE:u8 = 24; //static sao como constantes mas são de escopo IDADE e podem ter endereço de memoria acessado
                        // statics podem ser mutaveis porem precisa ser em unsafe block pois não é seguro
fn main(){
    const PI:f32 = 3.14;  //constantes em rust sao processadas em tempo de copilação
    // unsafe {
    //     IDADE = 10;
    //     println!("{}", IDADE);
    // }
    println!("Pi ={}", PI);
    let num:i32 = 128;
    println!("Num = {}, tamanho = {} byte", num, std::mem::size_of_val(&num));
    let decimal:f32 = 0.1;
    println!("decimal = {}", decimal);
    let mut booleana:bool = true;
    println!("Booleana {} de tamanho {} byte", booleana, std::mem::size_of_val(&booleana));
    booleana = false;
    println!("Agora a booleana é {}", booleana);
    shadowing();
    println!("soma = {}", soma(1,2));
    println!("Maior ? = {}", maior_de_idade(IDADE));
    loops(5);
    loop_for(4);
    match_stat();
}
fn shadowing(){
    let a:i32 = 100;

    {
        let b:i32 = 400;
        println!("antes da nova a ser atribuida a dentro = {}", a);
        let a:i32 = 99; //cria uma nova variavel de escopo, nao podendo ser acessada de fora
        println!("b dentro = {}", b);
        println!("a dentro = {}", a);
    }
    println!("a fora = {}", a)
}
fn soma(a:i32, b:i32) -> i32{
   //return 
   a+b //em rust o ; representa a supressão do retorno de uma função ou macro
       // se eu quiser somente retornar o valor de uma expressão posso somente nao colocar o ponto e virgula
       // porem posso usar o return normalmente tambem mas tudo em rust possui um valor. 
}

fn maior_de_idade(a:u8) -> bool{
    let eh_maior = a >= 18;
    let cond = if eh_maior { "maior" } else{"menor"}; //como tudo em rust é uma expressão uma condicional poide ser atribuido a uma variavel
    println!("Ele é {} e pode entrar", cond);
    if eh_maior{
        true
    }else {
        false
    }
    
}

fn loops(multiplicador:u8){
    let mut contador:u8 = 0;
    
    loop{
        contador += 1;
        println!("{} x {} = {}", multiplicador, contador, multiplicador * contador);
        if contador == 10{
            contador = 0;
            break; //tambem pode ser usado o continue para continuar o loop parando a execução de apenas uma tarefa do loop
        }
    }
    println!("Com while temos:");
    while contador < 10{
        contador += 1;
        println!("{} x {} = {}", multiplicador, contador, multiplicador * contador);
    }
    
}

fn loop_for(multiplicador:u8){
    for i in 1..=10{ // 1..11
        println!("{} x {} = {}", multiplicador, i, multiplicador * i);
    };
}
fn match_stat(){
    let linguagem = "";
    let proposito = match linguagem {
        "PHP" => "WEB",
        "PYTHON" =>"AUTOMAÇÃO E I.A",
        "GO" => "CONCORRENCIA",
        _ => "Desconhecido",
    };
    println!("O proposito da {} é = {}", linguagem, proposito);
}