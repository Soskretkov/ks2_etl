// use xlsxwriter::*;
mod extract;
mod load;
mod transform;
mod ui;
use crate::extract::{Book, Sheet, SEARCH_REFERENCE_POINTS};
// use crate::load::{DataSource, PART_1_REPORT};
use crate::transform::Act;

fn main() {
    let (_path, _sh_name) = ui::session();
    let path = String::from("f");

    let path = path + ".xlsm";
    let mut wb = Book::new(&path).expect(&("Не удалось считать файл".to_owned() + &path));

    let sheet = Sheet::new(
        &mut wb,
        "Лист1",
        &SEARCH_REFERENCE_POINTS,
        0 + 0 + 3 + 5 + 9 + 3,
    )
    .unwrap();

    let act = Act::new(sheet).unwrap();

    let act1 = act.clone();
    let act2 = act.clone();
    let act3 = act.clone();
    let vector_of_acts: Vec<Act> = vec![act1, act2, act3];

    let (part_2_fst_fls_base, part_4_fst_fls_curr) = load::first_file_data_names(&vector_of_acts[0].data_of_totals);

     println!("{:#?}", part_2_fst_fls_base);
    //  println!("{:#?}", part_4_fst_fls_curr);

    // Печать шапки
    let mut header = act
        .names_of_header
        .iter()
        .zip(act.data_of_header.iter().map(|x| x.as_ref().unwrap()));

    // for print in header {
    //     println!("{}:  {:?}", print.0 .0, print.1);
    // }
    let print = header.nth(2).unwrap();
    println!("{}:  {:?}", print.0 .0, print.1);

    // Печать итогов
    let summary = act.data_of_totals;
    let last_row = summary.iter().last().unwrap();
    println!(
        "\n{}: базовая - {}; текущая - {}",
        last_row.name,
        last_row.base_price.unwrap_or(0.),
        last_row.current_price.unwrap_or(0.)
    );

    //Печать rows Excel
    // let sheet = Sheet::new(
    //     &mut wb,
    //     "Лист1",
    //     &SEARCH_REFERENCE_POINTS,
    //     0 + 0 + 3 + 5 + 9 + 3,
    // )
    // .unwrap();

    // let start_of_range = sheet.search_points
    //     .get("Стоимость материальных ресурсов (всего)")
    //     .unwrap(); //unwrap не требует обработки
    // for row in sheet.data.rows().skip(start_of_range.0) {
    //     println!("{:?}", row);

    // // Пробная запись
    // let wb = Workbook::new("Test.xlsx");
    // let mut sh1 = wb.add_worksheet(Some("Лист1")).unwrap();
    // sh1.write_string(0, 0, "Red text", None).unwrap();
    // wb.close().unwrap();
    // }
}
