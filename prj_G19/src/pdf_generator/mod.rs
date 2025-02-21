
// SPDX-FileCopyrightText: 2020 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: CC0-1.0

//! This example generates a demo PDF document and writes it to the path that was passed as the
//! first command-line argument.  You may have to adapt the `FONT_DIRS`, `DEFAULT_FONT_NAME` and
//! `MONO_FONT_NAME` constants for your system so that these files exist:
//! - `{FONT_DIR}/{name}-Regular.ttf`
//! - `{FONT_DIR}/{name}-Bold.ttf`
//! - `{FONT_DIR}/{name}-Italic.ttf`
//! - `{FONT_DIR}/{name}-BoldItalic.ttf`
//! for `name` in {`DEFAULT_FONT_NAME`, `MONO_FONT_NAME`}.
//!
//! The generated document using the latest `genpdf-rs` release is available
//! [here](https://genpdf-rs.ireas.org/examples/demo.pdf).

mod encoder;
mod chart_generator;

use genpdf::{Alignment, Margins, Document};
use genpdf::Element as _;
use genpdf::{elements, fonts};
use genpdf::elements::{FrameCellDecorator, LinearLayout, PageBreak, Paragraph, TableLayout, UnorderedList};
use genpdf::style::{Color, Style};
use crate::analyzer::Analyzer;

const FONT_DIRS: &[&str] = &[
    "src/pdf_generator/fonts/times_new_roman"
];
const DEFAULT_FONT_NAME: &'static str = "TimesNewRoman";
pub fn print_pdf_all(file_path: &String, data_list: Vec<Analyzer>){
    let mut doc = setup_document();
    let italic = Style::new().italic().with_font_size(10);
    let bold_italic = Style::new().bold().italic().with_font_size(10);
    let title_style = Style::new().bold().with_font_size(12).italic();
    let text_margins= Margins::trbl(0, 70,0,0);
    let mut side_headers: Vec<&str> = Vec::new();
    doc.push(elements::Break::new(0.5));
    for i in 0..data_list.len(){
        let mut list_input_output = get_list_input_output(&data_list[i], &mut side_headers);
        doc.push(Paragraph::default().styled_string(format!("Configurazione sperimentale {}",i+1), title_style).padded(text_margins).styled(Color::Rgb(255, 0, 0)));
        doc.push(elements::Break::new(0.3));
        doc.push(Paragraph::default().styled_string("Tipologia di esperimento: ",bold_italic).styled_string("MULTIPLE ANALYSIS ",italic).padded(text_margins));
        doc.push(elements::Break::new(0.3));
        doc.push(Paragraph::default().styled_string("Algoritmo eseguito: ", bold_italic).styled_string(side_headers[i].to_string(),italic).padded(text_margins));
        doc.push(elements::Break::new(0.3));
        doc.push(Paragraph::default().styled_string("Numero di faults: ",bold_italic).styled_string(data_list[i].faults.total_fault.to_string(),italic).padded(text_margins));
        doc.push(elements::Break::new(0.3));
        doc.push(list_input_output.pop().unwrap().padded(Margins::trbl(0, 0,0,-10)));
        doc.push(elements::Break::new(0.5));
        doc.push(Paragraph::default().styled_string("Output", title_style).padded(text_margins).styled(Color::Rgb(255, 0, 0)));
        doc.push(list_input_output.pop().unwrap().padded(Margins::trbl(0, 0,0,-10)));
        doc.push(elements::Break::new(0.5));
        doc.push(Paragraph::default().styled_string("Tempi di esecuzione", title_style).padded(text_margins).styled(Color::Rgb(255, 0, 0)));
        doc.push(Paragraph::default().styled_string("Durata dell'esperimento di Fault Injection:",bold_italic).styled_string(data_list[i].time_experiment.to_string(),italic).styled_string(" micro secondi",italic).padded(text_margins));
        doc.push(elements::Break::new(2));
    }
    doc.push(PageBreak::new());
    doc.push(Paragraph::default().styled_string("Overhead", title_style).padded(text_margins).styled(Color::Rgb(255, 0, 0)));
    doc.push(Paragraph::default().styled_string("Tabella di riepilogo che evidenzia gli effetti dell'irrobustimento del codice in termini di dimensione e tempi di esecuzione per ciascun algoritmo.",italic).padded(text_margins));
    doc.push(elements::Break::new(0.5));
    let top_headers =  vec!["NOT HARD(B)", "HARD(B)", "HARD/NOT HARD","NOT HARD (us)","HARD (us)","HARD/NOT HARD"];
    let dim_time_table = gen_table_dim_time(&data_list,&top_headers,&side_headers);
    doc.push(dim_time_table);
    doc.push(elements::Break::new(0.5));
    doc.push(Paragraph::default().styled_string("Risultati",title_style).padded(text_margins).styled(Color::Rgb(255, 0, 0)));
    doc.push(Paragraph::default().styled_string("Di seguito vengono riportatati i faults non rilevati e rilevati, specificando per quest'ultimi le diverse tipologie riconosciute.",italic).padded(text_margins));
    doc.push(elements::Break::new(0.5));
    let images_paths = gen_pie_chart(&data_list, &side_headers);
    add_image_to_pdf(images_paths,&mut doc);
    let top_headers =  vec!["SILENT","ASSIGN","INNER","SUB","MUL","ADD","INDEX","PAR_ORD"];
    let fault_table = gen_table_faults(&data_list,&top_headers,&side_headers);
    doc.push(fault_table);
    doc.push(elements::Break::new(0.5));
    doc.push(PageBreak::new());
    let path = gen_bar_chart(&data_list,&side_headers,"ALGORITMO");
    doc.push(elements::Image::from_path(path).expect("Unable to load image").with_alignment(Alignment::Center));
    doc.push(elements::Break::new(0.5));
    doc.push(Paragraph::default().styled_string("Fault fatatali (selection sort) : ",bold_italic).styled_string(format!("{} %",f64::trunc((data_list[0].faults.n_fatal_fault as f64/data_list[0].faults.total_fault as f64)*10000.0)/100.0),italic).styled_string(" (percentuale di fault iniettati che hanno provocato un output errato)",italic).padded(text_margins));
    doc.push(Paragraph::default().styled_string("Fault fatatali (bubble sort) : ",bold_italic).styled_string(format!("{} %",f64::trunc((data_list[1].faults.n_fatal_fault as f64/data_list[1].faults.total_fault as f64)*10000.0)/100.0),italic).styled_string(" (percentuale di fault iniettati che hanno provocato un output errato)",italic).padded(text_margins));
    doc.push(Paragraph::default().styled_string("Fault fatatali (matrix multiplication) : ",bold_italic).styled_string(format!("{} %",f64::trunc((data_list[2].faults.n_fatal_fault as f64/data_list[2].faults.total_fault as f64)*10000.0)/100.0),italic).styled_string(" (percentuale di fault iniettati che hanno provocato un output errato)",italic).padded(text_margins));

    doc.render_to_file(file_path)
        .expect("Failed to write output file");
}


pub fn print_pdf_diffcard(file_path: &String, data_list: Vec<Analyzer>){
    let mut doc = setup_document();
    let italic = Style::new().italic().with_font_size(10);
    let bold_italic = Style::new().bold().italic().with_font_size(10);
    let title_style = Style::new().bold().with_font_size(12).italic();
    let text_margins= Margins::trbl(0, 70,0,0);
    let mut side_headers:Vec<&str> = Vec::new();
    doc.push(elements::Break::new(0.5));

    let mut chart_headers:Vec<&str> = Vec::new();
    match data_list[0].target_program.as_str() {
        "sel_sort"=> for _ in 0..3 {chart_headers.push("SELECTION SORT")} ,
        "bubble_sort"=> for _ in 0..3 {chart_headers.push("BUBBLE SORT")},
        "matrix_multiplication"=> for _ in 0..3 {chart_headers.push("MATRIX MULTIPLICATION")},
        _ => {}
    }

    let mut list_input_output = get_list_input_output(&data_list[0], &mut side_headers);
    doc.push(Paragraph::default().styled_string("Configurazione sperimentale", title_style).padded(text_margins).styled(Color::Rgb(255, 0, 0)));
    doc.push(elements::Break::new(0.3));
    doc.push(Paragraph::default().styled_string("Tipologia di esperimento: ",bold_italic).styled_string("MULTIPLE ANALYSIS",italic).padded(text_margins));
    doc.push(elements::Break::new(0.3));
    doc.push(Paragraph::default().styled_string("Algoritmo eseguito: ", bold_italic).styled_string(side_headers[0].to_string(),italic).padded(text_margins));
    doc.push(elements::Break::new(0.3));
    doc.push(Paragraph::default().styled_string("Numero di faults: ",bold_italic).styled_string(format!("[{},{},{}]",data_list[0].faults.total_fault,data_list[1].faults.total_fault,data_list[2].faults.total_fault),italic).padded(text_margins));
    doc.push(elements::Break::new(0.3));
    doc.push(list_input_output.pop().unwrap().padded(Margins::trbl(0, 0,0,-10)));
    doc.push(elements::Break::new(0.5));
    doc.push(Paragraph::default().styled_string("Output", title_style).padded(text_margins).styled(Color::Rgb(255, 0, 0)));
    doc.push(list_input_output.pop().unwrap().padded(Margins::trbl(0, 0,0,-10)));
    doc.push(elements::Break::new(0.5));
    doc.push(Paragraph::default().styled_string("Tempi di esecuzione", title_style).padded(text_margins).styled(Color::Rgb(255, 0, 0)));
    for i in 0..data_list.len(){
        doc.push(Paragraph::default().styled_string(format!("Durata dell'esperimento di Fault Injection per {} faults: ",data_list[i].faults.total_fault),bold_italic).styled_string(data_list[i].time_experiment.to_string(),italic).styled_string(" micro secondi",italic).padded(text_margins));
        doc.push(elements::Break::new(0.3));
    }
    doc.push(elements::Break::new(0.5));
    doc.push(Paragraph::default().styled_string("Overhead", title_style).padded(text_margins).styled(Color::Rgb(255, 0, 0)));
    doc.push(Paragraph::default().styled_string("Tabella di riepilogo che evidenzia gli effetti dell'irrobustimento del codice in termini di dimensione e tempi di esecuzione per ciascuna cardinalità della fault list.",italic).padded(text_margins));
    doc.push(elements::Break::new(0.5));
    let top_headers =  vec!["NOT HARD(B)", "HARD(B)", "HARD/NOT HARD","NOT HARD (us)","HARD (us)","HARD/NOT HARD"];
    //let side_headers = vec![format!("{} FAULTS",data_list[0].faults.total_fault),format!("{} FAULTS",data_list[1].faults.total_fault),format!("{} FAULTS",data_list[2].faults.total_fault)];
    let side_headers = vec!["1000 FAULTS","2000 FAULTS","3000 FAULTS"];
    let dim_time_table = gen_table_dim_time(&data_list,&top_headers,&side_headers);
    doc.push(dim_time_table);
    doc.push(elements::Break::new(0.5));

    doc.push(PageBreak::new());
    doc.push(Paragraph::default().styled_string("Risultati",title_style).padded(text_margins).styled(Color::Rgb(255, 0, 0)));
    doc.push(Paragraph::default().styled_string("Di seguito vengono riportatati i faults non rilevati e rilevati, specificando per quest'ultimi le diverse tipologie riconosciute.",italic).padded(text_margins));
    doc.push(elements::Break::new(0.5));
    let images_paths = gen_pie_chart(&data_list, &chart_headers);
    add_image_to_pdf(images_paths,&mut doc);
    let top_headers =  vec!["SILENT","ASSIGN","INNER","SUB","MUL","ADD","INDEX","PAR_ORD"];
    let fault_table = gen_table_faults(&data_list,&top_headers,&side_headers);
    doc.push(fault_table);
    doc.push(elements::Break::new(0.5));
    let path = gen_bar_chart(&data_list,&side_headers,"CARDINALITA'");
    doc.push(elements::Image::from_path(path).expect("Unable to load image").with_alignment(Alignment::Center));
    doc.push(elements::Break::new(0.5));
    doc.push(Paragraph::default().styled_string("Fault fatatali su 1000 iniezioni: ",bold_italic).styled_string(format!("{} %",f64::trunc((data_list[0].faults.n_fatal_fault as f64/data_list[0].faults.total_fault as f64)*10000.0)/100.0),italic).styled_string(" (percentuale di fault iniettati che hanno provocato un output errato)",italic).padded(text_margins));
    doc.push(Paragraph::default().styled_string("Fault fatatali su 2000 iniezioni: ",bold_italic).styled_string(format!("{} %",f64::trunc((data_list[1].faults.n_fatal_fault as f64/data_list[1].faults.total_fault as f64)*10000.0)/100.0),italic).styled_string(" (percentuale di fault iniettati che hanno provocato un output errato)",italic).padded(text_margins));
    doc.push(Paragraph::default().styled_string("Fault fatatali su 3000 iniezioni: ",bold_italic).styled_string(format!("{} %",f64::trunc((data_list[2].faults.n_fatal_fault as f64/data_list[2].faults.total_fault as f64)*10000.0)/100.0),italic).styled_string(" (percentuale di fault iniettati che hanno provocato un output errato)",italic).padded(text_margins));

    doc.render_to_file(file_path)
        .expect("Failed to write output file");
}
pub fn print_pdf_singolo(file_path: &String, analyzer: Analyzer) {
    let mut doc = setup_document();
    let mut data_list:Vec<Analyzer> = Vec::new();
    let mut side_headers:Vec<&str> = Vec::new();
    let mut list_input_output = get_list_input_output(&analyzer, &mut side_headers);
    let title_style = Style::new().bold().with_font_size(12).italic();
    let italic = Style::new().italic().with_font_size(10);
    let bold_italic = Style::new().bold().italic().with_font_size(10);
    let text_margins= Margins::trbl(0, 70,0,0);

    data_list.push(analyzer.clone());
    doc.push(elements::Break::new(0.5));
    doc.push(Paragraph::default().styled_string("Configurazione sperimentale", title_style).padded(text_margins).styled(Color::Rgb(255, 0, 0)));
    doc.push(elements::Break::new(0.3));
    doc.push(Paragraph::default().styled_string("Tipologia di esperimento: ",bold_italic).styled_string("SINGLE ANALYSIS ",italic).padded(text_margins));
    doc.push(elements::Break::new(0.3));
    doc.push(Paragraph::default().styled_string("Algoritmo scelto: ", bold_italic).styled_string(side_headers[0].to_string(),italic).padded(text_margins));
    doc.push(elements::Break::new(0.3));
    doc.push(Paragraph::default().styled_string("Numero di faults: ",bold_italic).styled_string(analyzer.faults.total_fault.to_string(),italic).padded(text_margins));
    doc.push(elements::Break::new(0.3));
    doc.push(list_input_output.pop().unwrap().padded(Margins::trbl(0, 0,0,-10)));
    doc.push(elements::Break::new(0.5));
    doc.push(Paragraph::default().styled_string("Output", title_style).padded(text_margins).styled(Color::Rgb(255, 0, 0)));
    doc.push(list_input_output.pop().unwrap().padded(Margins::trbl(0, 0,0,-10)));
    doc.push(elements::Break::new(0.5));
    doc.push(Paragraph::default().styled_string("Tempi di esecuzione", title_style).padded(text_margins).styled(Color::Rgb(255, 0, 0)));
    doc.push(Paragraph::default().styled_string("Durata dell'esperimento di Fault Injection:",bold_italic).styled_string(analyzer.time_experiment.to_string(),italic).styled_string(" micro secondi",italic).padded(text_margins));
    doc.push(elements::Break::new(0.5));
    doc.push(Paragraph::default().styled_string("Overhead", title_style).padded(text_margins).styled(Color::Rgb(255, 0, 0)));
    doc.push(Paragraph::default().styled_string("Tabella di riepilogo che evidenzia gli effetti dell'irrobustimento del codice in termini di dimensione e tempi di esecuzione.",italic).padded(text_margins));
    doc.push(elements::Break::new(0.5));
    let top_headers =  vec!["NOT HARD(B)", "HARD(B)", "HARD/NOT HARD","NOT HARD (us)","HARD (us)","HARD/NOT HARD"];
    let dim_time_table = gen_table_dim_time(&data_list,&top_headers,&side_headers);
    doc.push(dim_time_table);
    doc.push(elements::Break::new(0.5));
    doc.push(Paragraph::default().styled_string("Risultati",title_style).padded(text_margins).styled(Color::Rgb(255, 0, 0)));
    doc.push(Paragraph::default().styled_string("Di seguito viene illustrata la suddivisione dei faults non rilevati e rilevati, specificando per quest'ultimi le diverse tipologie riconosciute.",italic).padded(text_margins));
    doc.push(elements::Break::new(0.5));
    let images_paths = gen_pie_chart(&data_list, &side_headers);
    doc.push(elements::Image::from_path(images_paths[0]).expect("Unable to load image").with_alignment(Alignment::Center));
    let top_headers =  vec!["SILENT","ASSIGN","INNER","SUB","MUL","ADD","INDEX","PAR_ORD"];
    let fault_table = gen_table_faults(&data_list,&top_headers,&side_headers);
    doc.push(elements::Break::new(0.5));
    doc.push(fault_table);

    doc.push(elements::Break::new(0.5));
    doc.push(Paragraph::default().styled_string("Percentuale di detected:",bold_italic).styled_string(format!("{}",f64::trunc(((data_list[0].faults.total_fault as f64 - data_list[0].faults.n_silent_fault as f64)/data_list[0].faults.total_fault as f64)*10000.0)/100.0),italic).styled_string(" %",italic).padded(text_margins));
    doc.push(elements::Break::new(0.5));
    doc.push(Paragraph::default().styled_string("Fault fatatali: ",bold_italic).styled_string(format!("{} %",f64::trunc((data_list[0].faults.n_fatal_fault as f64/data_list[0].faults.total_fault as f64)*10000.0)/100.0),italic).styled_string(" (percentuale di fault iniettati che hanno provocato un output errato)",italic).padded(text_margins));

    doc.render_to_file(file_path)
        .expect("Failed to write output file");
}


pub fn gen_bar_chart(data_list: &Vec<Analyzer>, side_headers:&Vec<&str>, x_axis_label: &str)-> &'static str {
    let mut percentages = Vec::new();
    for anl in data_list{
        percentages.push(f64::trunc(((anl.faults.total_fault as f64 - anl.faults.n_silent_fault as f64)/anl.faults.total_fault as f64)*10000.0)/100.0);

    }
    chart_generator::bar_chart(percentages,side_headers,x_axis_label);
    "src/pdf_generator/images/percentage_detected.png"
}
pub fn gen_table_dim_time(data_list: &Vec<Analyzer> , top_headers: &Vec<&str>, side_headers: &Vec<&str>)-> TableLayout{
    let mut column_weights = vec![12; top_headers.len()+1];
    column_weights[0] = 17;
    let header_style = Style::new().with_font_size(7).bold();
    let mut table = TableLayout::new(column_weights);
    table.set_cell_decorator(FrameCellDecorator::new(false, true, false));

    let mut row = table.row().element(
        Paragraph::new("")
    );

    //Costruisco l'header
    for header in top_headers{
        row = row.element(
            Paragraph::default()
                .styled_string(*header, header_style)
                .aligned(Alignment::Center)
                .padded( Margins::trbl(0,4.5,2,0)),
        );
    }

    row.push().expect("Invalid table row");
    for i in 0..data_list.len(){
        let anl = &data_list[i];
        let info_vec = vec![  anl.byte_not_hardened,
                                        anl.byte_hardened,
                                        f64::trunc((anl.byte_hardened/anl.byte_not_hardened)*100.0)/100.0,
                                        anl.time_alg_not_hardened,
                                        anl.time_alg_hardened,
                                        f64::trunc((anl.time_alg_hardened/anl.time_alg_not_hardened)*100.0)/100.0
                                    ];
        let mut row = table.row().element(
            Paragraph::new(side_headers[i])
                .styled(header_style).padded(Margins::trbl(4,4.5,2,0)),
        );

        for info in info_vec{
            row = row.element(
                Paragraph::default()
                    .styled_string(&info.to_string(), Style::new().with_font_size(7).italic())
                    .aligned(Alignment::Center).padded(Margins::trbl(2,2,0,2)),
            );
        }
        row.push().expect("Invalid table row");
    }
    table


}

pub fn gen_pie_chart(data: &Vec<Analyzer>, target: &Vec <&str>)->Vec<&'static str> {
    for i in 0..data.len(){
        let anl = &data[i];
        let mut file_name = "pie_chart".to_string();
        file_name.push_str(&i.to_string());
        file_name.push_str(".png");
        chart_generator::not_rose_pie_chart(&anl.faults, file_name.as_str(),target[i]);
    }
    let mut image_paths = Vec::new();
    if data.len()>1 {
        image_paths.push("src/pdf_generator/images/pie_chart0.png");
        image_paths.push("src/pdf_generator/images/pie_chart1.png");
        image_paths.push("src/pdf_generator/images/pie_chart2.png");
    } else {
        image_paths.push("src/pdf_generator/images/pie_chart0.png");
    }
    image_paths
}

pub fn add_image_to_pdf(images_paths: Vec<&str>,doc: &mut Document){
    doc.push(elements::Image::from_path(images_paths[0]).expect("Unable to load image").
        with_alignment(Alignment::Center));

    //Metto due immagini una di fianco all'altra
    let mut table = TableLayout::new(vec![5, 5]);
    table.set_cell_decorator(FrameCellDecorator::new(false, false, false));
    table
        .row()
        .element(
            elements::PaddedElement::new(
                elements::Image::from_path(images_paths[1])
                    .expect("Unable to load image")
                    .with_alignment(Alignment::Center), Margins::trbl(0,40,0,0)),
        )
        .element(
            elements::PaddedElement::new(
                elements::Image::from_path(images_paths[2])
                    .expect("Unable to load image")
                    .with_alignment(Alignment::Center), Margins::trbl(0,0,0,0)),
        )
        .push()
        .expect("Invalid table row");
    doc.push(table.padded(Margins::trbl(5,0,0,20)));
}

pub fn gen_table_faults(data: &Vec<Analyzer>, top_headers: &Vec<&str>, side_headers: &Vec<&str>)-> TableLayout {
    let mut column_weights = vec![11; top_headers.len()+1];
    column_weights[0] = 18;
    let header_style = Style::new().with_font_size(7).bold();
    let mut table = TableLayout::new(column_weights);
    table.set_cell_decorator(FrameCellDecorator::new(false, true, false));
    //Costruisco l'header
    let mut row = table.row().element(
        Paragraph::new("")
    );

    for header in top_headers{
        row = row.element(
            Paragraph::default()
                .styled_string(*header, header_style)
                .aligned(Alignment::Center)
                .padded( Margins::trbl(0,4.5,2,0)),
        );
    }
    row.push().expect("Invalid table row");

    for i in 0..data.len(){
        let anl = &data[i];
        let f = &anl.faults;
        let mut row = table.row().element(
                    Paragraph::new(side_headers[i])
                    .styled(header_style).padded(Margins::trbl(4,4.5,2,0)),
                    );

            for info in f.iter(){
                row = row.element(
                    Paragraph::default()
                        .styled_string(&info.1.to_string(), Style::new().with_font_size(7).italic())
                        .aligned(Alignment::Center).padded(Margins::trbl(2,2,0,2)),
                );
            }
            row.push().expect("Invalid table row");

    }
    table

 }

fn get_list_input_output(analyzer: &Analyzer, side_headers: &mut Vec<&str>) -> Vec<UnorderedList> {
    let data_input = analyzer.input.clone();
    let data_output = analyzer.output.clone();
    let mut list_input = UnorderedList::with_bullet("");
    let mut list_output = UnorderedList::with_bullet("");
    let italic = Style::new().italic().with_font_size(10);
    let bold_italic = Style::new().bold().italic().with_font_size(10);
    let text_margins= Margins::trbl(0, 70,0,0);
    match analyzer.target_program.as_str() {
        "sel_sort"=> {
            side_headers.push("SELECTION SORT");
            let p_input = Paragraph::default().styled_string("Vettore di input: ", bold_italic)
                .styled_string(format!("{:?}",data_input.into_vector()),italic).padded(text_margins);
            let p_output = Paragraph::default().styled_string("Vettore ordinato: ", bold_italic)
                .styled_string(format!("{:?}",data_output.into_vector()),italic).padded(text_margins);
            list_input.push(p_input);
            list_output.push(p_output);
        },
        "bubble_sort"=> {
            side_headers.push("BUBBLE SORT");
            let p_input = Paragraph::default().styled_string("Vettore di input: ", bold_italic)
                .styled_string(format!("{:?}",data_input.into_vector()),italic).padded(text_margins);
            let  p_output = Paragraph::default().styled_string("Vettore ordinato: ", bold_italic)
                .styled_string(format!("{:?}",data_output.into_vector()),italic).padded(text_margins);
            list_input.push(p_input);
            list_output.push(p_output);
        },
        "matrix_multiplication"=> {
            let (a,b) = data_input.clone().into_matrices();
            let (output,_) = data_output.clone().into_matrices();
            let matrix_len = data_input.into_matrices().0.len();
            side_headers.push("MATRIX MULTIPLICATION");

            for i in 0..matrix_len {
                let mut p_input = Paragraph::default();
                let mut p_output = Paragraph::default();
                if i==0{
                    list_input.push(Paragraph::new("Matrici di input: ").styled(bold_italic).padded(text_margins));
                    list_output.push(Paragraph::new("Prodotto tra matrici: ").styled(bold_italic).padded(text_margins));
                }
                p_input = p_input.styled_string(format!(" {} ",a[i].iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ")),italic).styled_string(format!("    {} ",b[i].iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ")),italic);
                p_output = p_output.styled_string(format!(" {} ",output[i].iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ")),italic);
                list_input.push(p_input.padded(text_margins));
                list_output.push(p_output.padded(text_margins));

            }
        },
        _ => {}
    }
    vec![list_output,list_input]
}

fn setup_document()->Document{
    let title_style =  Style::new().bold().with_font_size(20);
    let title_margins= Margins::trbl(0, 10,0,0);
    let red = Color::Rgb(255, 0, 0);

    let font_dir = FONT_DIRS
        .iter()
        .filter(|path| std::path::Path::new(path).exists())
        .next()
        .expect("Could not find font directory");

    let default_font =
        fonts::from_files(font_dir, DEFAULT_FONT_NAME, Some(fonts::Builtin::Times))
            .expect("Failed to load the default font family");

    let mut doc = Document::new(default_font);
    doc.set_minimal_conformance();
    doc.set_line_spacing(0.85);


    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    decorator.set_header(|page| {
        let mut layout = LinearLayout::vertical();
        if page > 1 {
            layout.push(
                Paragraph::new(format!("Page {}", page)).aligned(Alignment::Right),
            );
            layout.push(elements::Break::new(1));
        }
        layout.styled(Style::new().with_font_size(12))
    });
    doc.set_page_decorator(decorator);
    doc.push(
       Paragraph::default().styled_string("Report",title_style).aligned(Alignment::Center)
                        .padded(title_margins).styled(red),
    );
     doc

}
#[cfg(test)]
mod tests {
    use crate::analyzer::Faults;
    use super::*;


    #[test]
    fn try_gen_table_faults(){
        let file_path = "results/prova_table.pdf";
        let anl = Analyzer::new(Faults {
            n_silent_fault: 10,
            n_assign_fault: 20,
            n_inner_fault: 60,
            n_sub_fault: 35,
            n_mul_fault: 30,
            n_add_fault: 50,
            n_indexmut_fault: 60,
            n_index_fault: 70,
            n_ord_fault: 80,
            n_partialord_fault: 90,
            n_partialeq_fault: 100,
            n_fatal_fault: 22,
            total_fault: 550,
        }, 324.98, 1,  "sel_sort".to_string());
        let mut anl2 = anl.clone();
        anl2.n_esecuzione=1;
        anl2.faults.n_add_fault=1;
        let mut anl3 = anl.clone();
        anl3.n_esecuzione = 2;
        anl3.faults.n_assign_fault=2;
        let data = vec![anl,anl2,anl3];
        let top_headers =  vec!["SILENT","ASSIGN","INNER","SUB","MUL","ADD","INDEX","PAR_ORD"];
        let side_headers = vec!["SELECTION SORT","BUBBLE SORT","MATRIX MULTIPLICATION"];
        let table = gen_table_faults(&data,&top_headers,&side_headers);
        let mut doc = setup_document();
        doc.push(table);
        doc.render_to_file(file_path)
            .expect("Failed to write output file");

    }
    #[test]
    fn try_gen_dim_time_table(){
        let file_path = "results/prova_table.pdf";
        let top_headers =  vec!["HARD(B)", "NOT HARD(B)", "HARD / NOT HARD","HARD(uS)","NOT HARD(uS)","HARD / NOT HARD"];
        let side_headers = vec!["SELECTION SORT","BUBBLE SORT","MATRIX MULTIPLICATION"];
        let anl = Analyzer::new(Faults {
            n_silent_fault: 10,
            n_assign_fault: 20,
            n_inner_fault: 35,
            n_sub_fault: 30,
            n_mul_fault: 30,
            n_add_fault: 50,
            n_indexmut_fault: 60,
            n_index_fault: 70,
            n_ord_fault: 80,
            n_partialord_fault: 90,
            n_partialeq_fault: 100,
            n_fatal_fault: 22,
            total_fault: 550,
        }, 324.98, 1,  "sel_sort".to_string());
        let mut anl2 = anl.clone();
        anl2.n_esecuzione=1;
        anl2.faults.n_add_fault=1;
        let mut anl3 = anl.clone();
        anl3.n_esecuzione = 2;
        anl3.faults.n_assign_fault=2;
        let data = vec![anl,anl2,anl3];
        let table = gen_table_dim_time(&data,&top_headers,&side_headers);
        let mut doc = setup_document();
        doc.push(table);
        doc.render_to_file(file_path)
            .expect("Failed to write output file");

    }

    #[test]
    fn try_gen_pie_chart(){
        let mut table = TableLayout::new(vec![1, 5]);
        let image_path1: &'static str = "src/pdf_generator/images/pie_chart1.png";
        let image_path2: &'static str = "src/pdf_generator/images/pie_chart2.png";
        let mut doc = setup_document();
        // Metto una sola immagine
        doc.push(elements::Image::from_path(image_path2).expect("Unable to load image").
            with_alignment(Alignment::Center));

        //Metto due immagini una di fianco all'altra
        let mut table = elements::TableLayout::new(vec![5, 5]);
        table.set_cell_decorator(elements::FrameCellDecorator::new(false, false, false));
        let mut linear_layout = elements::LinearLayout::vertical();

        table
            .row()
            .element(
                elements::PaddedElement::new(
                    elements::Image::from_path(image_path1)
                        .expect("Unable to load image")
                        .with_alignment(Alignment::Center), Margins::trbl(0,40,0,0)),
            )
            .element(
                elements::PaddedElement::new(
                    elements::Image::from_path(image_path2)
                        .expect("Unable to load image")
                        .with_alignment(Alignment::Center), Margins::trbl(0,0,0,0)),
            )
            .push()
            .expect("Invalid table row");
        doc.push(table.padded(Margins::trbl(5,0,0,20)));
        let file_path = "results/prova_pie_chart.pdf";
        doc.render_to_file(file_path)
            .expect("Failed to write output file");
    }
}
