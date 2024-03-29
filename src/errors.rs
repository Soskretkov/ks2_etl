use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Error<'a> {
    InternalLogic {
        tech_descr: String,
        err: Option<Box<dyn std::error::Error>>,
    },

    InvalidDashInUserPath {
        entered_path: PathBuf,
    },
    NumericConversion {
        tech_descr: String,
        err: Box<dyn std::error::Error>,
    },
    NumericOverflow {
        tech_descr: String,
    },

    CalamineFileOpen {
        file_path: PathBuf,
        err: calamine::XlsxError,
    },

    CalamineSheetOfTheBookIsUndetectable {
        file_path: PathBuf,
        sh_name_for_search: &'a str,
        sh_names: Vec<String>,
    },
    CalamineSheetOfTheBookIsUnreadable {
        file_path: PathBuf,
        sh_name: String, // нельзя ссылкой - имя листа с учетом регистра определяется внутри функции, где возможна ошибка
        err: calamine::XlsxError,
    },
    EmptySheetRange {
        file_path: PathBuf,
        sh_name: String, // нельзя ссылкой - имя листа с учетом регистра определяется внутри функции, где возможна ошибка
    },

    SheetNotContainAllNecessaryData {
        file_path: PathBuf,
    },

    SheetMisalignment {
        is_row_algmnt_check: bool,
        fst_tag_str: &'static str,
        fst_tag_index_on_sheet: String,
        snd_tag_str: &'static str,
        snd_tag_index_on_sheet: String,
        file_path: PathBuf,
    },

    XlsxwriterWorkbookCreation {
        wb_name: &'a str,
        err: xlsxwriter::XlsxError,
    },
    XlsxwriterSheetCreation,
    XlsxwriterCellWrite(xlsxwriter::XlsxError),
    XlsxwriterFormatting(xlsxwriter::XlsxError),
    XlsxwriterWorkbookClose {
        wb_name: &'a str,
        err: xlsxwriter::XlsxError,
    },
}

impl<'a> std::error::Error for Error<'a> {}

impl fmt::Display for Error<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InternalLogic { tech_descr, err } => {
                let base_msg = format!(
                    "Во внутренней логике программы произошла ошибка.
{tech_descr}"
                );
                let footer_msg = match err {
                    Some(err) => format!("Подробности об ошибке:\n{}", err),
                    None => "".to_string(),
                };
                let full_msg = format!("{base_msg}\n\n{footer_msg}");
                write!(f, "{full_msg}")
            }

            Self::InvalidDashInUserPath { entered_path } => {
                let base_msg = format!("Обнаружена ошибка в пути:\n{}", entered_path.display());
                let footer_msg ="Похоже, в пути присутствует длинное тире («— копия»).
Терминал windows (окно которое вы наблюдаете) иногда заменяет его на короткое, что приводит к ошибке.
Пожалуйста, переименуйте файл или папку, удалив или заменив длинное тире. После повторите попытку.

P.s. Не беспокойтесь о вложенных папках, в которых может встречаться длинное тире, программе
их наличие не мешает, проблема возникает на этапе когда вы вставляете путь в окно,
важно чтобы этот путь не исказился и длинное тире автоматически не подменилось на короткое
(попробуйте вставить в это окно любой текст с длинным тире, и вы увидите, что оно автоматически
меняется, что создает проблему программе принять от вас неискаженный терминалом windows путь).";
                let full_msg = format!("{base_msg}\n\n{footer_msg}");
                write!(f, "{full_msg}")
            }

            Self::NumericConversion { tech_descr, err } => {
                let base_msg = tech_descr;
                let footer_msg = format!("Подробности об ошибке:\n{}", err);
                let full_msg = format!("{base_msg}\n\n{footer_msg}");
                write!(f, "{full_msg}")
            }

            Self::NumericOverflow { tech_descr } => {
                let base_msg = "Переполнение при операции с числами.";
                let footer_msg = format!("Подробности об ошибке:\n{}", tech_descr);
                let full_msg = format!("{base_msg}\n\n{footer_msg}");
                write!(f, "{full_msg}")
            }

            Self::CalamineFileOpen { file_path, err } => {
                let base_msg = "Возникла проблема с открытием книги Excel для чтения.";
                let footer_msg = format!("Подробности об ошибке:\n{err}");
                let path_msg = format!("Файл, вызывающий ошибку:\n{}", file_path.display());
                let full_msg = format!("{base_msg}\n\n{footer_msg}\n\n{path_msg}");
                write!(f, "{full_msg}")
            }

            Self::CalamineSheetOfTheBookIsUndetectable {
                file_path,
                sh_name_for_search,
                sh_names,
            } => {
                let string_sh_names = format!("{:?}", sh_names);

                // базовое сообщение
                let base_msg = format!(
                    "Встретился файл, который не содержит запрашиваемого вами листа '{sh_name_for_search}',
так как файл имеет только следующие листы:
    
{string_sh_names}"
                );

                // опциональное сообщение про кавычки как возможную причину ошибки
                let optional_msg =
                    if sh_name_for_search.starts_with('"') && sh_name_for_search.ends_with('"') {
                        format!(
                            "
                        
Обратите внимание: вы ввели имя листа, заключённое в кавычки ('{sh_name_for_search}'), эти кавычки,
могут являться причиной ошибки, так как обычно имена листов в книгах Excel не заключают в кавычки.
Попробуйте повторить процедуру и ввести имя листа таким, каким вы его видите в самом файле Excel."
                        )
                    } else {
                        "".to_string()
                    };

                // заключительная часть сообщения
                let footer_msg = "Чтобы успешно выполнить процедуру сбора файлов, выполните одно из перечисленных действий:
- откройте файл, вызывающий ошибку, и присвойте листу с актом имя, которое затем укажете программе;
- если не хотите собирать этот файл, переименуйте файл, добавив к существующему имени символ '@',
  или удалите файл из папки;
- если не хотите собирать папку, где находится файл, добавьте к существующему имени папки символ '@'.";

                let path_msg = format!("Файл, вызывающий ошибку:\n{}", file_path.display());

                // объединение всех частей в одно сообщение
                let full_msg = format!("{base_msg}{optional_msg}\n\n{footer_msg}\n\n{path_msg}");
                write!(f, "{full_msg}")
            }

            Self::CalamineSheetOfTheBookIsUnreadable {
                file_path,
                sh_name,
                err,
            } => {
                let base_msg = format!("Возникла проблема с чтением листа '{sh_name}'.");
                let footer_msg = format!("Подробности об ошибке:\n{err}");
                let path_msg = format!("Файл, вызывающий ошибку:\n{}", file_path.display());
                let full_msg = format!("{base_msg}\n\n{footer_msg}\n\n{path_msg}");
                write!(f, "{full_msg}")
            }

            Self::EmptySheetRange { file_path, sh_name } => {
                let base_msg = format!("Лист '{sh_name}' не содержит данных (пуст).");
                let path_msg = format!("Файл, вызывающий ошибку:\n{}", file_path.display());
                let full_msg = format!("{base_msg}\n\n{path_msg}");
                write!(f, "{full_msg}")
            }

            Self::SheetNotContainAllNecessaryData { file_path } => {
                let base_msg = "Акт не содержит полных данных.
От собираемого файла требуется следующий набор ключевых слов:
    'Стройка',
    'Объект',
    'Договор подряда',
    'Доп. соглашение',
    'Номер документа',
    'О ПРИЕМКЕ ВЫПОЛЕНЕННЫХ РАБОТ',
    'Наименование работ и затрат',
    'Стоимость в ценах 2001',
    'Стоимость в текущих ценах',
    'Стоимость материальных ресурсов (всего)'.
    
Если чего-то из перечисленного в акте не обнаружено, такой акт не может быть собран.
Проверьте документ на наличие перечисленных ключевых слов. Если ошибка происходит при наличии всех
ключевых слов - проверьте строковый порядок: вхождение слов по строкам должно быть в порядке
перечисленом выше (т.е. в файле строка 'Стройка' должна быть выше строки с 'Объект', а 'Объект',
в свою очередь, расположен выше (или левее) строки с текстом 'Договор подряда' и так далее).";
                let path_msg = format!("Файл, вызывающий ошибку:\n{}", file_path.display());
                let full_msg = format!("{base_msg}\n\n{path_msg}");
                write!(f, "{full_msg}")
            }

            Self::SheetMisalignment {
                is_row_algmnt_check,
                fst_tag_str,
                fst_tag_index_on_sheet,
                snd_tag_str,
                snd_tag_index_on_sheet,
                file_path,
            } => {
                let alignment_type = if *is_row_algmnt_check {
                    "строке"
                } else {
                    "колонке"
                };

                let base_msg = format!(
"Ожидалось, что ключевые слова '{fst_tag_str}' и '{snd_tag_str}'
будут оба находиться в одной и той же {alignment_type}.
Однако, '{fst_tag_str}' найдено в {alignment_type} '{fst_tag_index_on_sheet}', а '{snd_tag_str}' — в {alignment_type} '{snd_tag_index_on_sheet}'.

Для исправления ошибки разместите оба ключевых слова в одной {alignment_type}.");

                let path_msg = format!("Файл, вызывающий ошибку:\n{}", file_path.display());
                let full_msg = format!("{base_msg}\n\n{path_msg}");
                write!(f, "{full_msg}")
            }

            Self::XlsxwriterWorkbookCreation { wb_name, err } => {
                let base_msg = format!(
                    "Не удалась попытка создания файла Excel с именем '{wb_name}'."
                );
                let footer_msg = format!("Подробности об ошибке:\n{err}");

                let full_msg = format!("{base_msg}\n\n{footer_msg}");
                write!(f, "{full_msg}")
            }

            Self::XlsxwriterSheetCreation => {
                let msg = "Не удалась попытка создание листа результата внутри отчетного файла Excel, речь о листе Excel на котором
должен был быть записан результат работы программы.";
                write!(f, "{msg}")
            }

            Self::XlsxwriterCellWrite(err) => {
                let base_msg =
                    "Не удалась попытка записи данных в ячейку отчетного файла Excel.";

                let footer_msg = format!("Подробности об ошибке:\n{err}");
                let full_msg = format!("{base_msg}\n\n{footer_msg}");
                write!(f, "{full_msg}")
            }

            Self::XlsxwriterFormatting(err) => {
                let base_msg =
                    "Не удалась попытка форматирования отчетного файла Excel";

                let footer_msg = format!("Подробности об ошибке:\n{err}");
                let full_msg = format!("{base_msg}\n\n{footer_msg}");
                write!(f, "{full_msg}")
            }

            Self::XlsxwriterWorkbookClose { wb_name, .. } => {
                let msg = format!(
                    "Не удалось сохранение на диск файла Excel с именем '{wb_name}', который содержит
результат работы программы.
                    
Вероятная причина ошибки: не закрыт файл Excel с результатами прошлого сбора."
                );
                write!(f, "{msg}")
            }
        }
    }
}
