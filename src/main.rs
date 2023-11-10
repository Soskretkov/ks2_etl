use console::{Style, Term}; // для очистки консоли перед выводом полезных сообщений
use std::env;
use std::path;
use std::thread; // для засыпания на секунду-две при печати сообщений
use std::time::Duration; // для засыпания на секунду-две при печати сообщений // имя ".exe" будет присвоено файлу Excel
mod config;
mod error;
mod extract;
mod load;
mod transform;
mod ui;
use crate::extract::Sheet;
use crate::load::Report;
use crate::transform::Act;

fn main() {
    Term::stdout().set_title("«Ks2 etl»,  v".to_string() + env!("CARGO_PKG_VERSION"));
    ui::show_first_lines();
    ui::show_help();
    'main_loop: loop {
        let _ = Term::stdout().clear_screen();

        let (path, user_entered_sh_name) = ui::user_input();
        // Debug
        // let user_entered_sh_name = "Лист1".to_owned();
        // let path = std::path::PathBuf::from(r"C:\Users\User\rust\ks2_etl".to_string());

        let string_report_path = env::args()
            .next()
            .unwrap()
            .trim_end_matches(".exe")
            .to_owned()
            + ".xlsx";

        let report_path = path::PathBuf::from(string_report_path);

        let cyan = Style::new().cyan();
        let red = Style::new().red();

        let books_vec = match extract::get_vector_of_books(path) {
            Ok(vec) => vec,
            Err(err) => {
                let _ = Term::stdout().clear_last_lines(1);
                println!(
                    "\n{}\n{}",
                    red.apply_to(" Возникла ошибка."),
                    err.to_string()
                );
                thread::sleep(Duration::from_secs(2));
                continue 'main_loop;
            }
        };

        let acts_vec = {
            let mut temp_acts_vec = Vec::new();
            for mut item in books_vec.into_iter() {
                let book = item.as_mut().unwrap();
                let wrapped_sheet = Sheet::new(
                    book,
                    &user_entered_sh_name,
                    29, //передается для расчета смещения столбцов. Это сумма номеров столбцов Y-типа в DESIRED_DATA_ARRAY: 0 + 0 + 3 + 5 + 9 + 9 + 3.
                );

                let sheet = match wrapped_sheet {
                    Ok(x) => x,
                    Err(err) => {
                        let _ = Term::stdout().clear_last_lines(1);
                        println!(
                            "\n{}\n{}",
                            red.apply_to(" Возникла ошибка."),
                            err.to_string()
                        );
                        thread::sleep(Duration::from_secs(3));
                        println!("\n\n\n\n");
                        continue 'main_loop;
                    }
                };

                let wrapped_act = Act::new(sheet);
                let act = match wrapped_act {
                    Ok(x) => x,
                    Err(err) => {
                        let _ = Term::stdout().clear_last_lines(1);
                        println!(
                            "\n{}\n{}",
                            red.apply_to(" Возникла ошибка."),
                            err.to_string()
                        );
                        thread::sleep(Duration::from_secs(3));
                        println!("\n\n\n\n");
                        continue 'main_loop;
                    }
                };
                temp_acts_vec.push(act);
            }
            temp_acts_vec
        };
        // "При создании Report требуется передать вектор актов. Это связанно с тем, что xlsxwriter
        // не умеет вставлять столбцы и не может переносить то, что им же записано (не умеет читать Excel),
        // что навязывает необходимость установить общее количество
        // столбцов, и их порядок до того как начнется запись актов. Получается, на протяжении работы программы в Report
        // акты передаются дважды: при создании формы отчета для создания выборки всех названий, что встречаются в итогах,
        // а второй раз акт в Report будет передан циклом записи."

        // очистка от временного сообщения
        let _ = Term::stdout().clear_last_lines(1);
        println!(
            " Идет построение структуры excel-отчета в зависимости от содержания итогов актов, ожидайте..."
        );

        let mut report = Report::new(&report_path, &acts_vec).unwrap();

        let _ = Term::stdout().clear_last_lines(1);
        println!(" Идет запись, ожидайте...");

        for act in acts_vec.iter() {
            match report.write(act) {
                Ok(updated_report) => report = updated_report,
                Err(err) => {
                    let _ = Term::stdout().clear_last_lines(1);
                    println!(
                        "\n{}\n{}",
                        red.apply_to(" Возникла ошибка."),
                        err.to_string()
                    );
                    thread::sleep(Duration::from_secs(3));
                    println!("\n\n\n\n");
                    continue 'main_loop;
                }
            }
        }

        let files_counter = report.body_syze_in_row;

        if let Err(err) = report.write_and_close_report(&report_path) {
            let _ = Term::stdout().clear_last_lines(3);
            println!(
                "\n{}\n{}",
                red.apply_to(" Возникла ошибка."),
                err.to_string()
            );
            thread::sleep(Duration::from_secs(3));
            println!("\n\n\n\n");
            continue 'main_loop;
        }

        let _ = Term::stdout().clear_last_lines(1);
        println!("{}", cyan.apply_to(" Успешно выполнено."));
        println!(" Собрано {} файла(ов).", files_counter);
        println!("\n Создан файл \"{}\"", report_path.display());
        thread::sleep(Duration::from_secs(1));
        println!("\n\n");
    }
}
