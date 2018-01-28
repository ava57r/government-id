extern crate government_id;

use government_id::*;

fn check_kpp(s: &str) {
    let value = Kpp::new(s);
    match value.is_valid() {
        Ok(res) => println!("{}", res),
        Err(msg) => println!("Error: {}", msg),
    }
}

fn main() {
    const HELP: &'static str = "Использование: government_id команда [аргументы]...
    Команды:
        check kpp KPP - проверить КПП. Параметр KPP - содержит Код причины постановки на учет (КПП)
        help  - показать это сообщение.";

    let args: Vec<String> = std::env::args().collect();
    match args.get(1) {
        Some(text) => match text.as_ref() {
            "check" => match args.get(2) {
                Some(flag) => match flag.as_ref() {
                    "kpp" => {
                        if args.len() != 4 {
                            panic!("Использование: government_id check kpp KPP");
                        }
                        check_kpp(&args[3])
                    }
                    param @ _ => panic!(format!(
                        "Неправильный параметр команды check: {}",
                        param
                    )),
                },
                None => panic!("Отсутствует обязательный параметр"),
            },
            "help" => {
                println!("{}", HELP);
            }
            command @ _ => panic!(format!(
                "Неправильная команда: {}",
                command
            )),
        },
        None => println!("{}", HELP),
    }
}
