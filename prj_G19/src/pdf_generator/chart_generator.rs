use charts_rs::{BarChart, PieChart};
use crate::analyzer::Faults;
use crate::pdf_generator::encoder::svg_to_png;
pub fn not_rose_pie_chart(faults: &Faults, file_name: &str,target: &str) {
    let pie_chart_json = format!(
        r###"{{
            "title_text": "Faults",
            "sub_title_text": "Risultato iniezione {} errori su {}",
            "legend_show": false,
            "radius": 130,
            "inner_radius": 30,
            "rose_type": false,
            "series_list": [
                {{
                    "name": "Silent",
                    "data": [{}]
                }},
                {{
                    "name": "Assignment",
                    "data": [{}]
                }},
                {{
                    "name": "Inner",
                    "data": [{}]
                }},
                {{
                    "name": "Subtraction",
                    "data": [{}]
                }},
                {{
                    "name": "Multiplication",
                    "data": [{}]
                }},
                {{
                    "name": "Addition",
                    "data": [{}]
                }}
                ,
                {{
                    "name": "Index",
                    "data": [{}]
                }},
                {{
                    "name": "PartialOrd",
                    "data": [{}]
                }}
            ]
        }}"###,
        faults.total_fault,
        target,
        faults.n_silent_fault,
        faults.n_assign_fault,
        faults.n_inner_fault,
        faults.n_sub_fault,
        faults.n_mul_fault,
        faults.n_add_fault,
        faults.n_index_fault,
        faults.n_partialord_fault,
    );
    let pie_chart = PieChart::from_json(&pie_chart_json).unwrap();
    let res = pie_chart.svg().unwrap();
    let dest_path = "src/pdf_generator/images/";
    svg_to_png(&res, dest_path, file_name).expect("Impossibile convertire SVG in PNG");
}
pub fn bar_chart(data: Vec<f64>, x_axis_data: &Vec<&str>,x_axis_label: &str ){
    let bar_chart_json = format!(r###"{{
            "width": 630,
            "height": 410,
            "margin": {{
                "left": 10,
                "top": 5,
                "right": 10
            }},
            "sub_title_text": "Percentuale di fault detected rispetto al totale, confronto tra diverse esecuzioni",
            "sub_title_align": "right",
            "sub_title_font_weight": "bold",
            "legend_align": "center",
            "legend_font_weight": "bold",
            "y_axis_configs": [
                {{
                    "axis_font_weight": "bold"
                }}
            ],
            "series_label_font_weight": "bold",
            "series_list": [
                {{  "name":"{}",
                    "label_show": true,
                    "data": [{}]
                }},
                {{  "name":"{}",
                    "label_show": true,
                    "data": [{}]
                }},
                {{  "name":"{}",
                    "label_show": true,
                    "data": [{}]
                }}
            ],
            "x_axis_data": [
                "{}"
            ],
            "x_axis_margin": {{
                "left": 1,
                "top": 0,
                "right": 0,
                "bottom": 0
            }},
            "x_axis_font_weight": "bold"
        }}"###, x_axis_data[0], data[0],  x_axis_data[1], data[1],  x_axis_data[2] ,data[2], x_axis_label);

    let mut bar_chart = BarChart::from_json(&bar_chart_json).unwrap();
    bar_chart.y_axis_configs[0].axis_width = Some(100.0);
    bar_chart.y_axis_configs[0].axis_max = Some(100.0);
    bar_chart.y_axis_configs[0].axis_formatter = Some("{c} %".to_string());
    let res = bar_chart.svg().unwrap();
    let dest_path = "src/pdf_generator/images/";
    let file_name = "percentage_detected.png";
    svg_to_png(&res, dest_path, file_name).expect("Impossibile convertire SVG in PNG");
}

#[cfg(test)]
mod tests {
    use charts_rs::{BarChart, PieChart};
    use crate::pdf_generator::encoder::svg_to_png;

    #[test]
    fn bar_chart() {
        let bar_chart_json = format!(r###"{{
            "width": 630,
            "height": 410,
            "margin": {{
                "left": 10,
                "top": 5,
                "right": 10
            }},
            "title_text": "Bar Chart",
            "title_font_color": "#345",
            "title_align": "right",
            "sub_title_text": "demo",
            "sub_title_align": "right",
            "sub_title_font_weight": "bold",
            "legend_align": "left",
            "legend_font_weight": "bold",
            "y_axis_configs": [
                {{
                    "axis_font_weight": "bold"
                }}
            ],
            "series_label_font_weight": "bold",
            "series_list": [
                {{
                    "name": "Email",
                    "label_show": true,
                    "data": [120.0, 132.0, 101.0, 134.0, 90.0, 230.0, 210.0]
                }},
                {{
                    "name": "Union Ads",
                    "data": [220.0, 182.0, 191.0, 234.0, 290.0, 330.0, 310.0]
                }},
                {{
                    "name": "Direct",
                    "data": [320.0, 332.0, 301.0, 334.0, 390.0, 330.0, 320.0],
                    "colors": [null, "#a90000"]
                }},
                {{
                    "name": "Search Engine",
                    "data": [{},{},{}, 934.0, 1290.0, 1330.0, 1320.0]
                }}
            ],
            "x_axis_data": [
                "{}",
                "{}",
                "{}",
                "Thu",
                "Fri",
                "Sat",
                "Sun"
            ],
            "x_axis_margin": {{
                "left": 1,
                "top": 0,
                "right": 0,
                "bottom": 0
            }},
            "x_axis_font_weight": "bold"
        }}"###,
                                         3,
                                         3,
                                         2,
                "SELECTION","BUBBLE","MATRIX"
        );
        println!("{}", bar_chart_json);
        let bar_chart = BarChart::from_json(&bar_chart_json).unwrap();
        println!("{:#?}", bar_chart);
        let res = bar_chart.svg().unwrap();
        let dest_path = "src/pdf_generator/images/";
        let file_name = "test_bar_chart.png";
        svg_to_png(&res, dest_path, file_name).expect("Impossibile convertire SVG in PNG");
    }
}