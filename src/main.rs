use std::io;
use rand::{thread_rng, Rng};

fn main() {
    let answers: [&str; 20] = ["Бесспорно","Предрешено",
        "Никаких сомнений", "Определённо да",
        "Можешь быть уверен в этом", "Мне кажется — «да»",
        "Вероятнее всего", "Хорошие перспективы",
        "Знаки говорят — «да»", "Пока не ясно, попробуй снова",
        "Да",
        "Спроси позже", "Лучше не рассказывать",
        "Сейчас нельзя предсказать", "Сконцентрируйся и спроси опять",
        "Даже не думай", "Мой ответ — «нет»",
        "По моим данным — «нет»", "Перспективы не очень хорошие",
        "Весьма сомнительно"];

    println!("---Магический шар с ответами---");
    
    println!("Введи свой вопрос:");
    let mut ask = String::new();
    io::stdin().read_line(&mut ask).expect("Вы ввели пустату!");
    println!("{}", answers[thread_rng().gen_range(0..21)]);
}
