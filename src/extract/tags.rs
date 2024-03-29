use crate::errors::Error;
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub enum Column {
    Initial,
    Contract,
}

#[derive(Clone, Copy)]
pub enum Row {
    TableHeader,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq, Copy)]
pub enum TagID {
    Генподрядчик,
    Субподрядчик,
    Исполнитель,
    Стройка,
    Объект,
    ДоговорПодряда,
    ДопСоглашение,
    НомерДокумента,
    ОПриемкеВыполненныхРабот,
    НаименованиеРаботИЗатрат,
    СтоимостьВЦенах2001,
    СтоимостьВТекущихЦенах,
    ЗтрВсего,
    ИтогоПоАкту,
    СтоимостьМатериальныхРесурсовВсего,
}

#[rustfmt::skip]
impl TagID {
    pub fn as_str(&self) -> &'static str {
        match self {
            TagID::Генподрядчик => "Генподрядчик",
            TagID::Субподрядчик => "Субподрядчик",
            TagID::Исполнитель => "Исполнитель",
            TagID::Стройка => "Стройка",
            TagID::Объект => "Объект",
            TagID::ДоговорПодряда => "Договор подряда",
            TagID::ДопСоглашение => "Доп. соглашение", // треб. страховать случай "Доп. Соглашение" - Excel автоматически передводит символ после точки в верх. регистр
            TagID::НомерДокумента => "Номер документа",
            TagID::ОПриемкеВыполненныхРабот => "О ПРИЕМКЕ ВЫПОЛНЕННЫХ РАБОТ",
            TagID::НаименованиеРаботИЗатрат => "Наименование работ и затрат",
            TagID::СтоимостьВЦенах2001 => "Стоимость в ценах 2001",
            TagID::СтоимостьВТекущихЦенах => "Стоимость в текущих ценах",
            TagID::ЗтрВсего => "ЗТР всего чел",
            TagID::ИтогоПоАкту => "Итого по акту:",
            TagID::СтоимостьМатериальныхРесурсовВсего => "Стоимость материальных ресурсов (всего)",
        }
    }
}

// режим сравнения двух текстов: частичное или полное совпадение
pub enum TextCmp {
    Part,
    Whole,
    StartsWith,
    EndsWith,
}

pub struct TagInfo {
    pub id: TagID,
    pub is_required: bool,
    pub group_by_row: Option<Row>,
    pub group_by_col: Option<Column>,
    pub look_at: TextCmp,
    pub match_case: bool,
}

// Перечислены в порядке вхождения на листе Excel при чтении ячеек слева направо и сверху вниз  (важно для валидации)
// При изменении поправить display для типа Error::SheetNotContainAllNecessaryData
#[rustfmt::skip]
pub const TAG_INFO_ARRAY: [TagInfo; 15] = [
    TagInfo { id: TagID::Генподрядчик,                       is_required: false, group_by_row: None,                   group_by_col: Some(Column::Initial),  look_at: TextCmp::Whole,      match_case: true },
    TagInfo { id: TagID::Субподрядчик,                       is_required: false, group_by_row: None,                   group_by_col: Some(Column::Initial),  look_at: TextCmp::Whole,      match_case: true },
    TagInfo { id: TagID::Исполнитель,                        is_required: false, group_by_row: None,                   group_by_col: Some(Column::Initial),  look_at: TextCmp::Whole,      match_case: true },
    TagInfo { id: TagID::Стройка,                            is_required: true,  group_by_row: None,                   group_by_col: Some(Column::Initial),  look_at: TextCmp::Whole,      match_case: true },
    TagInfo { id: TagID::Объект,                             is_required: true,  group_by_row: None,                   group_by_col: Some(Column::Initial),  look_at: TextCmp::Whole,      match_case: true },
    TagInfo { id: TagID::ДоговорПодряда,                     is_required: true,  group_by_row: None,                   group_by_col: Some(Column::Contract), look_at: TextCmp::Whole,      match_case: true },
    TagInfo { id: TagID::ДопСоглашение,                      is_required: true,  group_by_row: None,                   group_by_col: Some(Column::Contract), look_at: TextCmp::Whole,      match_case: false },
    TagInfo { id: TagID::НомерДокумента,                     is_required: true,  group_by_row: None,                   group_by_col: None,                   look_at: TextCmp::Whole,      match_case: true },
    TagInfo { id: TagID::ОПриемкеВыполненныхРабот,           is_required: true,  group_by_row: None,                   group_by_col: Some(Column::Initial),  look_at: TextCmp::Whole,      match_case: true },
    TagInfo { id: TagID::НаименованиеРаботИЗатрат,           is_required: true,  group_by_row: Some(Row::TableHeader), group_by_col: None,                   look_at: TextCmp::Whole,      match_case: true },
    TagInfo { id: TagID::СтоимостьВЦенах2001,                is_required: true,  group_by_row: Some(Row::TableHeader), group_by_col: None,                   look_at: TextCmp::StartsWith, match_case: true },
    TagInfo { id: TagID::СтоимостьВТекущихЦенах,             is_required: true,  group_by_row: Some(Row::TableHeader), group_by_col: None,                   look_at: TextCmp::Whole,      match_case: true },
    TagInfo { id: TagID::ЗтрВсего,                           is_required: false, group_by_row: Some(Row::TableHeader), group_by_col: None,                   look_at: TextCmp::StartsWith, match_case: true },
    TagInfo { id: TagID::ИтогоПоАкту,                        is_required: false, group_by_row: None,                   group_by_col: Some(Column::Initial),  look_at: TextCmp::Whole,      match_case: true },
    TagInfo { id: TagID::СтоимостьМатериальныхРесурсовВсего, is_required: true,  group_by_row: None,                   group_by_col: None,                   look_at: TextCmp::Whole,      match_case: true },
];

// (не удалять)
// pub struct TagArrayTools;

// impl TagArrayTools {
//     pub fn _get_tags() -> &'static [TagInfo] {
//         &TAG_INFO_ARRAY
//     }
//     pub fn get_tag_info_by_id(id: TagID) -> Result<TagInfo, Error<'static>> {
//         TAG_INFO_ARRAY
//             .into_iter()
//             .find(|tag_info| tag_info.id == id)
//             .ok_or_else(|| Error::InternalLogic {
//                 tech_descr: format!(r#"Массив тегов не содержит тег "{}""#, id.as_str()),
//                 err: None,
//             })
//     }
// }

// Это обертка над хешкартой, нужна чтобы централизовать обработку ошибок.
// В противном случае каждая попытка прочитать данные из Hmap потребует индивидуальный unwrap с конвертацией в ошибку
#[derive(Debug)]
pub struct TagAddressMap {
    data: HashMap<TagID, (usize, usize)>,
}

impl TagAddressMap {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
    pub fn get(&self, key: &TagID) -> Result<&(usize, usize), Error<'static>> {
        self.data.get(key).ok_or_else(|| Error::InternalLogic {
            tech_descr: format!(r#"Хешкарта не содержит ключ "{}""#, key.as_str()),
            err: None,
        })
    }
    pub fn insert(&mut self, key: TagID, data: (usize, usize)) {
        self.data.insert(key, data);
    }
}
