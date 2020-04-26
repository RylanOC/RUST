use plotlib::page::Page;
use plotlib::repr::BarChart;
use plotlib::repr::{Histogram, HistogramBins};
use plotlib::style::BoxStyle;
use plotlib::view::CategoricalView;
use plotlib::view::ContinuousView;

use std::collections::VecDeque;

#[derive(Copy, Clone, Debug)]
pub struct BarchartDatum<'a> {
    name: &'a str,
    pub value: f64,
}

impl<'a> BarchartDatum<'a> {
    pub fn new(n: &'a str, v: f64) -> BarchartDatum {
        BarchartDatum { name: n, value: v }
    }
    pub fn add_value(mut self, v: f64) {
        self.value += v;
    }
}

//     let colors: [&str, 7] = ["white","#1DB954","hotpink","yellow","cornflowerblue","crimson","mediumorchid"];

pub fn make_barchart(
    data: Vec<&BarchartDatum>,
    col: &VecDeque<&str>,
    label: &str,
    ylabel: &str,
) -> std::string::String {
    let mut colors: VecDeque<&str> = col.clone();
    let mut v = CategoricalView::new()
        .x_label(label.to_string())
        .y_label(ylabel.to_string());

    for datum in &data {
        let curr_color: &str = colors.pop_front().unwrap();
        v = v.add(
            BarChart::new(datum.value)
                .label(datum.name.clone())
                .style(&BoxStyle::new().fill(curr_color.to_string())),
        );

        colors.push_back(curr_color);
    }

    Page::single(&v)
        .dimensions(1400, 400)
        .to_svg()
        .unwrap()
        .to_string()
}

pub fn make_histogram(
    data: Vec<f64>,
    label: &str,
    color: &str,
    ylabel: &str,
) -> std::string::String {
    let h = Histogram::from_slice(&data, HistogramBins::Count(20))
        .style(&BoxStyle::new().fill(color.to_string()));

    let v = ContinuousView::new().add(h).x_label(label).y_label(ylabel);

    Page::single(&v).to_svg().unwrap().to_string()
}
