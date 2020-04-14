use crate plotlib::src::page::Page;
use crate plotlib::src::repr::BarChart;
use crate plotlib::src::style::BoxStyle;
use crate plotlib::src::view::CategoricalView;

use std::collections::VecDeque;

struct BarchartDatum {
    name: String,
    value: i32
}

impl BarchartDatum {
    fn new(n: &str, v: i32) -> BarchartDatum {
        BarchartDatum {
            name: n.to_string(),
            value: v
        }
    }
}

fn make_barchart(data: Vec<BarchartDatum>, col: Vec<&str>, label: &str, filename: &str, ) -> Result<(), &'static str> {
    let mut bars = Vec::new();
    let colors: VecDeque<&str> = VecDeque::from_iter(&col);
    for datum in &data {
        let curr_color: &str = colors.pop_front().unwrap();
        bars.push(BarChart::new(datum.value)
            .label(datum.name)
            .style(&BoxStyle::new().fill(curr_color).to_string()));
        colors.push_back(curr_color);
    }

    let v = CategoricalView::new().x_label(label.to_string());
    for b in &bars { v.add(b); }

    Page::single(&v).save(filename.to_string())
}

// fn main() {
//     let colors: [&str, 7] = ["white","#1DB954","hotpink","yellow","cornflowerblue","crimson","mediumorchid"];

// }