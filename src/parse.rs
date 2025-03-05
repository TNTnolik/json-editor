//version 0.0.2
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ErrorEnum {
    Warning,
    Error,
    Info,
}

#[derive(Serialize, Deserialize)]
pub struct Scen {
    pub file_name: String,
    pub text: String,
    pub scen: Option<Vec<ScenItem>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ScenItem {
    pub regex: String,
    pub names: Vec<String>,
    pub file_name_bool: bool,
    pub table: bool,
    pub position: usize,
    pub table_mask: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OutputItem {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OutputItemTabls {
    pub name: String,
    pub value: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorItem {
    pub message: String,
    pub type_error: ErrorEnum,
}

pub fn parse(
    file_name: String,
    text: String,
    scens: Vec<ScenItem>,
) -> (Vec<OutputItem>, Vec<OutputItemTabls>, Vec<ErrorItem>) {
    let mut out_singl_vec: Vec<OutputItem> = vec![];
    let mut out_tabls_vec: Vec<OutputItemTabls> = vec![];
    let mut out_error_vec: Vec<ErrorItem> = vec![];

    for r in scens {
        if !r.table {
            let Ok(re) = Regex::new(&r.regex) else {
                out_error_vec.push(ErrorItem {
                    message: format!("Есть ошибки в регулярном выражении: {}", &r.regex),
                    type_error: ErrorEnum::Error,
                });
                continue;
            };
            let local_text = if !r.file_name_bool {&text} else {&file_name};
            let Some(caps) = re.captures(local_text) else {
                out_error_vec.push(ErrorItem {
                    message: format!("Не найдено совпадений по выражению: {}", &r.regex),
                    type_error: ErrorEnum::Warning,
                });
                continue;
            };
            for name in r.names {
                match caps.name(name.as_str()) {
                    Some(v) => out_singl_vec.push(OutputItem {
                        name: name,
                        value: v.as_str().to_string(),
                    }),
                    None => {
                        out_error_vec.push(ErrorItem {
                            message: format!("Не найдено совпадений по имени: {}", &name),
                            type_error: ErrorEnum::Warning,
                        });
                    }
                }
            }
        } else {
            // если есть выражение для Таблицы
            let Some(tm) = &r.table_mask else {
                out_error_vec.push(ErrorItem {
                    message: format!("Отсутствует регулярное вырожение для поиска таблицы"),
                    type_error: ErrorEnum::Error,
                });
                continue;
            };

            let Ok(reg_table) = Regex::new(tm) else {
                out_error_vec.push(ErrorItem {
                    message: format!("Есть ошибки в регулярном выражении: {}", tm),
                    type_error: ErrorEnum::Error,
                });
                continue;
            };

            let Some(table) = reg_table.captures_iter(&text).nth(r.position) else {
                out_error_vec.push(ErrorItem {
                    message: format!(
                        "Не найдено совпадений по выражению и позиции: {}, {}",
                        tm, r.position
                    ),
                    type_error: ErrorEnum::Warning,
                });
                continue;
            };

            let Some(text_table) = table.get(0) else {
                continue;
            };

            let Ok(reg) = Regex::new(&r.regex) else {
                out_error_vec.push(ErrorItem {
                    message: format!("Есть ошибки в регулярном выражении: {}", &r.regex),
                    type_error: ErrorEnum::Error,
                });
                continue;
            };

            for item in reg.captures_iter(text_table.as_str()) {
                for name in &r.names {
                    match item.name(name.as_str()) {
                        Some(v) => {
                            match out_tabls_vec
                                .iter()
                                .position(|v| v.name.as_str() == name.as_str())
                            {
                                Some(position) => {
                                    out_tabls_vec[position].value.push(v.as_str().to_string());
                                }
                                None => out_tabls_vec.push(OutputItemTabls {
                                    name: name.clone(),
                                    value: vec![v.as_str().to_string()],
                                }),
                            }
                        }
                        None => {
                            out_error_vec.push(ErrorItem {
                                message: format!("Не найдено совпадений по имени: {}", &name),
                                type_error: ErrorEnum::Warning,
                            });
                        }
                    }
                }
            }
        }
    }
    (out_singl_vec, out_tabls_vec, out_error_vec)
}
