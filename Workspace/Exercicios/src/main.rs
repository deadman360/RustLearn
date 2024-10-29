use chrono::{Datelike, Local, NaiveDate};
use std::io;
fn main() {
    let idade: i32 = 0; //calcula_idade(pega_input());
    println!("{} anos", idade);
    println!("{}", is_even(890309489));
    println!("{}ºF", converte_celsius(0f32));
    println!("{}", fatorial(10, 1));
    println!("{}", fibonacci(9));
}

fn calcula_idade(data_informada: NaiveDate) -> i32 {
    let data_agora: NaiveDate = Local::now().date_naive();
    if data_informada.month() <= data_agora.month() {
        if data_informada.day() <= data_agora.day() {
            return data_agora.year() - data_informada.year();
        } else {
            return data_agora.year() - data_informada.year() - 1;
        }
    } else {
        return data_agora.year() - data_informada.year() - 1;
    }
}
fn pega_input() -> NaiveDate {
    /*Aqui eu Inicializo as variaveis do input */
    let mut input1: String = String::new();
    let mut input2: String = String::new();
    let mut input3: String = String::new();

    /*Aqui a func stdin pega o userinput e retorna um Stdin que tem o metodo
    read_line que retorna um result, no caso de ok ele passas a mensagem pro0 buffer mas
    se retorna um  erro quem resolve é o .expect que crasha o programa
    isso poderia ter sido feito com o match tambem */

    println!("Bom dia, Que dia voce nasceu?");
    io::stdin().read_line(&mut input1).expect("Falha");
    println!("em que mes ?");
    io::stdin().read_line(&mut input2).expect("Falha");
    println!(" e o ano??");
    io::stdin().read_line(&mut input3).expect("Falha");

    let dia: u32 = input1.trim().parse().unwrap();
    let mes: u32 = input2.trim().parse().unwrap();
    let ano: i32 = input3.trim().parse().unwrap();
    NaiveDate::from_ymd_opt(ano, mes, dia).unwrap()
}

fn is_even(num: i32) -> &'static str {
    match num % 2 {
        1 => "Impar",
        0 => "Par",
        _ => "Desconhecido",
    }
}

fn converte_celsius(celsius: f32) -> f32 {
    celsius * (9f32 / 5f32) + 32f32
}
fn fatorial(n: i64, counter: i64) -> i64 {
    match n {
        1 => counter,
        _ => fatorial(n - 1, counter  * n ),
    }
}
fn fibonacci(posicao_desejada: u32) -> i64{
        let (mut n1, mut n2) = (0i64, 1i64);
        for _ in 1..posicao_desejada{
            let novo_n: i64 = n1+n2;
            n1 = n2;
            n2 = novo_n;
        }
        n2
}
