use zoon::eprintln;
use zoon::named_color::{BLUE_0, BLUE_8, GREEN_0, GREEN_1, GREEN_5, GREEN_8};
use zoon::routing::url;
use zoon::Text;
use zoon::*;

#[static_ref]
fn selection() -> &'static Mutable<String> {
    Mutable::new(String::new())
}

fn root() -> impl Element {
    Column::new()
        .items(vec![
            Row::new().item("Select your training today"),
            Row::new().item(Column::new().items(vec![
                Row::new().items(vec![update_button("Leg day"), update_button("Chest day")]),
                Row::new().items(vec![
                    update_button("Let's do pull up and shoulders"),
                    update_button("I am resting"),
                ]),
            ])),
        ])
        .item(Row::new().item(Text::with_signal(selection().signal())))
}

fn update_button(name: &str) -> impl Element {
    let selected = Mutable::new(false);
    let query = format!("selected={}", name);
    let task = Task::start_droppable(selected.signal().for_each_sync(move |sel| {
        eprintln!("is selected ? {}", sel);
        if sel {
            selection().set(selection().take() + &query.to_string());
        } else {
            // Todo find the string and remove it problably
        }
        eprintln!(" Query is {:?}", &selection())
    }));

    Button::new()
        .s(Width::exact(150))
        .s(Height::exact(50))
        .s(Borders::all_signal(selected.signal().map_bool(
            || Border::new().color(BLUE_8).dashed(),
            || Border::new().color(BLUE_0).dotted(),
        )))
        .s(Background::new().color_signal(selected.signal().map_bool(|| GREEN_5, || GREEN_8)))
        .on_press(move || selected.update(|select| !select))
        .after_remove(move |_| drop(task))
        .label(name)
}

// ---------- // -----------

fn main() {
    start_app("app", root);
}
