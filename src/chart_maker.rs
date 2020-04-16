
use plotlib::page::Page;
use plotlib::repr::BarChart;
use plotlib::style::BoxStyle;
use plotlib::view::CategoricalView;
use plotlib::repr::{Histogram, HistogramBins};
use plotlib::view::ContinuousView;

use std::collections::VecDeque;
use std::iter::FromIterator;

pub struct BarchartDatum {
    name: String,
    value: f64
}

impl BarchartDatum {
    pub fn new(n: &str, v: f64) -> BarchartDatum {
        BarchartDatum {
            name: n.to_string(),
            value: v
        }
    }
}

pub fn make_barchart(data: Vec<BarchartDatum>, col: Vec<&str>, label: &str, filename: &str, ) -> Result<String> {

    let mut colors: VecDeque<&str> = VecDeque::from_iter(col);
    let mut v = CategoricalView::new().x_label(label.to_string());
    for datum in &data {
        let curr_color: &str = colors.pop_front().unwrap();

        v = v.add(BarChart::new(datum.value)
                .label(datum.name.clone())
                .style(&BoxStyle::new().fill(curr_color.to_string())));

        colors.push_back(curr_color);
    }

    

    //Page::single(&v).save(filename.to_string()).expect("saving svg");
    Page::single(&v).to_svg()
}

// fn main() {
//     let colors: [&str, 7] = ["white","#1DB954","hotpink","yellow","cornflowerblue","crimson","mediumorchid"];

// }

pub fn make_histogram(data: Vec<f64>, label: &str, filename: &str) {
    let h = Histogram::from_slice(&data, HistogramBins::Count(10))
        .style(&BoxStyle::new().fill("#cfe8fa"));

    let v = ContinuousView::new().add(h).x_label(label);

    Page::single(&v).save(filename).expect("saving svg");
}