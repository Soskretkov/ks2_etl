use console::Term;
use dialoguer::Input;
use std::io;
use std::path::PathBuf;
use std::thread; // для засыпания на секунду-две
use std::time::Duration; // для засыпания на секунду-две // для очистки консоли перед выводом полезных сообщений

pub fn user_input() -> (PathBuf, String) {
    loop {
        let entered_text = inputting_path();
        let path = PathBuf::from(&entered_text);

        if path.exists() {
            break (path, entered_sheet_name());
        }

        //filter нужен на случай ввода "details"  в кавычках (@ - на случай русской раскладки)
        let keyword = entered_text
            .chars()
            .filter(|ch| *ch != '"' && *ch != '@')
            .collect::<String>()
            .to_lowercase();
        let len_text = keyword.chars().count();

        match keyword {
            x if len_text < 9
                && x.matches([
                    'd', 'e', 't', 'a', 'i', 'l', 's', 'в', 'у', 'е', 'ф', 'ш', 'д', 'ы',
                ])
                .count()
                    > 4 =>
            {
                show_details();
                thread::sleep(Duration::from_secs(2));
                continue;
            }
            _ => continue,
        }
    }
}

fn inputting_path() -> String {
    println!(" Введите путь:");
    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect(" Ошибка чтения ввода");

    text = text.trim().to_string();
    text
}

fn entered_sheet_name() -> String {
    let _ = Term::stdout().clear_screen();
    let temp_sh_name: String = Input::new()
        .with_prompt(
            r#" Подтвердите лист или укажите другой.
 Не имеет значения, используете ли вы прописные или строчные буквы при указании листа.
 Имя листа"#,
        )
        .with_initial_text("Лист1")
        .interact()
        .expect("Ошибка чтения ввода");
    temp_sh_name
}

pub fn show_first_lines() {
    println!("        Введите  \"details\"  для получения подробностей о программе.\n");
}
#[rustfmt::skip]
pub fn show_help() {
    println!("------------------------------------------------------------------------------------------------------------\n");
    println!(" ● Используйте CTRL + C, чтобы вставить скопированный путь к папке или файлу с данными, которые вы хотите собрать.");
    println!(" ● Программа будет собирать данные из файлов Excel по указанному пути, включая вложенные папки.");
    println!(" ● Собираются только файлы с расширением «.xlsm».");
    println!(" ● Полезный совет:\n     - переименуйте файл Excel, добавив символ «@», и программа не будет собирать его данные;");
    println!("     - переименуйте папку, добавив символ «@», и программа проигнорирует ее содержимое.");
    println!("\n------------------------------------------------------------------------------------------------------------\n");
}
#[rustfmt::skip]
fn show_details() {
    // Очистка прошлых сообщений
    let _ = Term::stdout().clear_screen();
    println!("\n");
    show_help();

    //println!("------------------------------------------------------------------------------------------------------------\n");
    println!("            Наименование продукта:        «Сборщик данных из актов формы \"КС-2\"»");
    println!("            Версия продукта:              {}", env!("CARGO_PKG_VERSION"));
    println!("            Дата основания проекта:       02.06.2022");
    println!("            Адрес на GitHub.com:          https://github.com/Soskretkov/ks2_etl");
    println!("            Автор:                        Оскретков Сергей Юрьевич\n");
    println!("            Специально для: ООО «Трест Росспецэнергомонтаж»,");
    println!("            Альтуфьевское шоссе, д. 43, стр. 1, Москва, 127410,");
    println!("            Cметно-договорное управление.");
    println!(
        "\n------------------------------------------------------------------------------------------------------------\n"
    );
}
