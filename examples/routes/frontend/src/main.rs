use zoon::*;
use zoon::named_color::{BLUE_0, BLUE_8, GREEN_0, GREEN_1, GREEN_5, GREEN_8};


fn root() -> impl Element {
    Column::new().items(vec![
   Row::new().item("Select your training today"),
   Row::new()
       .item(Column::new() .items(vec![
           Row::new().items(vec![cell("Leg day"),cell("Chest day")]),
           Row::new().items(vec![cell("Let's do pull up and shoulders"),cell("I am resting")])
       ])
    )])
}

fn cell(name : &str) -> impl Element {
    let selected= Mutable::new(false);
        Button::new()
            .s(Width::exact(150)).s(Height::exact(50))
        .s(Borders::all_signal(selected.signal().map_bool(
            || Border::new().color(BLUE_8).dashed(),
            || Border::new().color(BLUE_0).dotted(),
        )))
            .s(Background::new().color_signal(selected.signal().map_bool(
                || GREEN_5,
                || GREEN_8
            )))
        .on_press(move || selected.set_neq(!selected.take()))
        .label(name)
}



// ---------- // -----------

fn main() {
    start_app("app", root);
}
