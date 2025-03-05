use gloo_file::{callbacks::read_as_text, Blob, File};
use gloo_utils::document;
use web_sys::{
    wasm_bindgen::JsCast, window, DragEvent, HtmlAnchorElement, HtmlElement, HtmlInputElement, Url,
};
use yew::prelude::*;

mod parse;
use parse::{parse, Scen, ScenItem};

fn input_string(state: yew::UseStateHandle<String>) -> Callback<InputEvent> {
    Callback::from(move |e: InputEvent| {
        if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
            state.set(input.value());
        }
    })
}

#[function_component]
fn App() -> Html {
    let file_content = use_state(|| None::<String>);
    let file_text = use_state(|| String::new());
    let file_name = use_state(|| String::new());
    let scens: UseStateHandle<Vec<ScenItem>> = use_state(|| vec![]);
    let focus = use_state(|| -1);

    let regex_input = use_state(|| String::new());
    let names_input = use_state(|| String::new());
    let table_input = use_state(|| false);
    let file_name_bool_input = use_state(|| false);
    let position_input = use_state(|| 0 as usize);
    let table_mask_input = use_state(|| String::new());

    let on_regex_input = input_string(regex_input.clone());
    let on_names_input = input_string(names_input.clone());
    let on_table_mask_input = input_string(table_mask_input.clone());

    let on_table_input = {
        let table_input = table_input.clone();
        Callback::from(move |event: Event| {
            let input = event.target_unchecked_into::<HtmlInputElement>();
            table_input.set(input.checked());
        })
    };

    let on_file_name_bool_input = {
        let file_name_bool_input = file_name_bool_input.clone();
        Callback::from(move |event: Event| {
            let input = event.target_unchecked_into::<HtmlInputElement>();
                file_name_bool_input.set(input.checked());
        })
    };

    let on_position_input = {
        let position_input = position_input.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                if let Ok(u) = input.value().trim().parse::<usize>() {
                    position_input.set(u);
                } else {
                    position_input.set(0);
                };
            }
        })
    };

    let ondragover = Callback::from(|event: DragEvent| {
        event.prevent_default();
    });

    let read_file = {
        let file_content = file_content.clone();
        let file_text = file_text.clone();
        let file_name = file_name.clone();
        let scens = scens.clone();
        move |file: File| {
            let file_reader = read_as_text(&file, {
                let file_content = file_content.clone();
                let file_name = file_name.clone();
                let file_text = file_text.clone();
                let scens = scens.clone();
                move |result| {
                    if let Ok(text) = result {
                        file_content.set(Some(text.clone()));
                        match serde_json::from_str::<Scen>(&text) {
                            Ok(v) => {
                                file_name.set(v.file_name);
                                file_text.set(v.text);
                                if let Some(scens_l) = v.scen {
                                    scens.set(scens_l);
                                };
                            }
                            Err(_) => {}
                        }
                        // let jsv: serde_json::Value = serde_json::from_str(&text);
                        // file_text.set(text.clone());
                    }
                }
            });

            std::mem::forget(file_reader);
        }
    };

    let ondrop = {
        let read_file = read_file.clone();
        Callback::from(move |event: DragEvent| {
            event.prevent_default();

            if let Some(files) = event.data_transfer().and_then(|dt| dt.files()) {
                if let Some(file) = files.get(0) {
                    read_file(File::from(file));
                }
            };
        })
    };

    let file_input_ref = use_node_ref();

    let on_file_change = {
        let file_input_ref = file_input_ref.clone();
        let read_file = read_file.clone();
        Callback::from(move |_| {
            if let Some(input) = file_input_ref.cast::<HtmlInputElement>() {
                if let Some(file) = input.files().and_then(|files| files.get(0)) {
                    read_file(File::from(file));
                }
            }
        })
    };

    let on_click = {
        let file_input_ref = file_input_ref.clone();
        Callback::from(move |_| {
            if let Some(input) = file_input_ref.cast::<HtmlInputElement>() {
                input.click();
            }
        })
    };
    let on_click_save = {
        let regex_input = regex_input.clone();
        let names_input = names_input.clone();
        let file_name_bool_input = file_name_bool_input.clone();
        let table_input = table_input.clone();
        let position_input = position_input.clone();
        let table_mask_input = table_mask_input.clone();
        let scens = scens.clone();
        let focus = focus.clone();
        let mut scen_vec: Vec<ScenItem> = (*scens).clone();

        let names = (*names_input)
            .trim()
            .split(',')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let mut tm: Option<String> = None;

        if *table_input {
            tm = Some((*table_mask_input).clone());
        }
        let item = ScenItem {
            regex: (*regex_input).clone(),
            names: names,
            file_name_bool: *file_name_bool_input,
            table: *table_input,
            position: *position_input,
            table_mask: tm,
        };
        if *focus < 0 {
            scen_vec.push(item);
        } else {
            scen_vec[*focus as usize] = item.clone();
        }

        Callback::from(move |_| {
            scens.set(scen_vec.clone());
        })
    };

    let on_click_test = {
        let regex_input = regex_input.clone();
        let names_input = names_input.clone();
        let file_name_bool_input = file_name_bool_input.clone();
        let table_input = table_input.clone();
        let position_input = position_input.clone();
        let table_mask_input = table_mask_input.clone();
        let file_text = file_text.clone();
        let file_name = file_name.clone();

        let names = (*names_input)
            .trim()
            .split(',')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let mut tm: Option<String> = None;

        if *table_input {
            tm = Some((*table_mask_input).clone());
        }
        let item = ScenItem {
            regex: (*regex_input).clone(),
            names: names,
            file_name_bool: *file_name_bool_input,
            table: *table_input,
            position: *position_input,
            table_mask: tm,
        };

        let out = parse((*file_name).clone(), (*file_text).clone(), vec![item]);

        
        let js = serde_json::to_string(&{out}).unwrap();

        Callback::from(move |_| {
            if let Some(win) = window() {
                win.alert_with_message(format!("{}", js).as_str())
                    .expect("");
            }
        })
    };

    let on_click_test_all = {
        let scens = scens.clone();
        let file_text = file_text.clone();
        let file_name = file_name.clone();

        let out = parse((*file_name).clone(), (*file_text).clone(), (*scens).clone());

        
        let js = serde_json::to_string(&{out}).unwrap();

        Callback::from(move |_| {
            if let Some(win) = window() {
                win.alert_with_message(format!("{}", js).as_str())
                    .expect("");
            }
        })
    };

    let on_click_clear = {
        let regex_input = regex_input.clone();
        let names_input = names_input.clone();
        let file_name_bool_input = file_name_bool_input.clone();
        let table_input = table_input.clone();
        let position_input = position_input.clone();
        let table_mask_input = table_mask_input.clone();
        let focus = focus.clone();

        Callback::from(move |_| {
            regex_input.set(String::new());
            names_input.set(String::new());
            file_name_bool_input.set(false);
            table_input.set(false);
            position_input.set(0 as usize);
            table_mask_input.set(String::new());
            focus.set(-1);
        })
    };

    let on_click_edit = {
        let scens = scens.clone();
        let regex_input = regex_input.clone();
        let names_input = names_input.clone();
        let file_name_bool_input = file_name_bool_input.clone();
        let table_input = table_input.clone();
        let position_input = position_input.clone();
        let table_mask_input = table_mask_input.clone();
        let focus = focus.clone();
        Callback::from(move |e: MouseEvent| {
            if let Some(target) = e.target_dyn_into::<HtmlElement>() {
                if let Ok(step) = target
                    .dataset()
                    .get("step")
                    .unwrap_or_default()
                    .parse::<usize>()
                {
                    if let Some(scen_item) = (*scens).get(step) {
                        regex_input.set(scen_item.regex.clone());
                        names_input.set(scen_item.names.join(","));
                        file_name_bool_input.set(scen_item.file_name_bool);
                        table_input.set(scen_item.table);
                        position_input.set(scen_item.position);
                        if let Some(tm) = scen_item.table_mask.clone() {
                            table_mask_input.set(tm);
                        };
                        focus.set(step as i32);
                    };
                }
            }
        })
    };

    let on_click_remove = {
        let scens = scens.clone();
        Callback::from(move |e: MouseEvent| {
            if let Some(target) = e.target_dyn_into::<HtmlElement>() {
                if let Ok(step) = target
                    .dataset()
                    .get("step")
                    .unwrap_or_default()
                    .parse::<usize>()
                {
                    let mut l_scens = (*scens).clone();
                    l_scens.remove(step);
                    scens.set(l_scens);
                }
            }
        })
    };

    let on_save_file = {
        let scens = scens.clone();
        move |_| {
            if let Ok(data) = serde_json::to_string_pretty(&(*scens)) {
                let blob = Blob::new_with_options(data.as_str(), Some("application/json"));
                let url = gloo_file::ObjectUrl::from(blob);
                let document = document();
                if let Ok(anchor) = document.create_element("a") {
                    if let Ok(anchor_d) = anchor.dyn_into::<HtmlAnchorElement>() {
                        anchor_d.set_href(&url);
                        anchor_d.set_download("name.json");
                        document.body().unwrap().append_child(&anchor_d).unwrap();
                        anchor_d.click();
                        document.body().unwrap().remove_child(&anchor_d).unwrap();

                        Url::revoke_object_url(&url).unwrap();
                    };
                };
            };
        }
    };

    html! {
    <div class="container">
        <div class="left">
            <h2>{"Json редактор"}</h2>
            <div class="upload-area" ondragover={ondragover} ondrop={ondrop} onclick={on_click}>
                {"Перетащите файл cюда или нажмите для выбора"}
            </div>
            <input type="file" accept=".json" ref={file_input_ref} onchange={on_file_change} style="display: none;"/>
            <div class="output"><h4>{"Имя файла:"}</h4>{(*file_name).clone()}</div>
            <div class="output"><h4>{"Текст:"}</h4>{(*file_text).clone()}</div>
        </div>
        <div class="right">
            <div class="form-group">
                <label>{"Регулярное выражения:"}</label>
                <input type="text" value={(*regex_input).clone()} oninput={on_regex_input}/>
            </div>
            <div class="form-group">
                <label>{"Имена (через запятую):"}</label>
                <input type="text" value={(*names_input).clone()} oninput={on_names_input}/>
            </div>
            <div class="form-group">
                <label>{"Поиск в имени файла:"}</label>
                <input type="checkbox" checked={*file_name_bool_input} onchange={on_file_name_bool_input}/>
            </div>
            <div class="form-group">
                <label>{"Таблица:"}</label>
                <input type="checkbox" checked={*table_input} onchange={on_table_input}/>
            </div>
            <div class="form-group">
                <label>{"Позиция:"}</label>
                <input type="number" value={format!("{}", *position_input)} oninput={on_position_input}/>
            </div>
            <div class="form-group">
                <label>{"Регулярное выражение для поиска таблици:"}</label>
                <input type="text" value={(*table_mask_input).clone()} oninput={on_table_mask_input}/>
            </div>
            <div class="buttons">
                <button class="save-btn" onclick={on_click_save}>{"Сохранить"}</button>
                <button class="reset-btn" onclick={on_click_clear}>{"Сбросить"}</button>
                <button class="test-btn" onclick={on_click_test}>{"Тест"}</button>
                <button class="test-btn" onclick={on_click_test_all}>{"Тест всех сценариев"}</button>
                <button class="test-btn" onclick={on_save_file}>{"Сохранить файл"}</button>
            </div>
            <div class="saved-list">{
                (*scens).clone().into_iter().enumerate().map(|(index, item)| {
                    html!{
                        <div>
                            <ul>
                                <li>{format!("Регулярное выражения: \"{}\"", item.regex)}</li>
                                <li>{format!("Имена: [{}]", item.names.join(","))}</li>
                                <li>{format!("Поиск в имени файла: {}", item.file_name_bool)}</li>
                                <li>{format!("Таблица: {}", item.table)}</li>
                                if item.table {
                                    <li>{format!("Позиция: {}", item.position)}</li>
                                    if item.table_mask.is_some() {
                                        <li>{format!("Регулярное выражения для поиска таблицы: \"{}\"", item.table_mask.expect("Не может быть пустым"))}</li>
                                    }

                                }
                            </ul>

                            <span class="entry-buttons">
                                <button class="edit-btn" onclick={on_click_edit.clone()} data-step={format!("{}", index)}>{"Редактировать"}</button>
                                <button class="delete-btn" onclick={on_click_remove.clone()} data-step={format!("{}", index)}>{"Удалить"}</button>
                            </span>
                        </div>
                    }
                }).collect::<Html>()
            }</div>
        </div>
    </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
