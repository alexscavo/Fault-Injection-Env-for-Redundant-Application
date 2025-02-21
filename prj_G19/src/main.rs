mod hardened;
mod fault_list_manager;
mod fault_env;
mod injector;
mod analyzer;
mod pdf_generator;

use fault_list_manager::static_analysis;
use std::io::{BufRead, Error};
use std::io;
use std::path::Path;
use std::fs::File;
use std::time::Instant;
use crate::fault_env::{Data, fault_injection_env};
use crate::fault_list_manager::DimData;
use crate::hardened::*;
use dialoguer::{Select, Input};
use rand::Rng;
use regex::Regex;


///Ambiente di Fault Injection per applicazione ridondata
pub static VERBOSE: bool = false; //Settare true per messaggi di iniezione
#[derive(Debug)]
pub struct InputData {
    pub vector: Vec<i32>,
    pub matrix_size: usize,
    pub matrix1: Vec<Vec<i32>>,
    pub matrix2: Vec<Vec<i32>>,
}
impl InputData {
    fn into_data(&self, ty:&str) -> Data<i32> {
        match ty {
            "vector" => Data::Vector(self.vector.clone()),
            "matrices" => Data::Matrices(self.matrix1.clone(), self.matrix2.clone()),
            _ => panic!("Unknown input type {}", ty),
        }
    }
    fn into_dimdata(&self, ty:&str) -> DimData {
        match ty {
            "vector" => DimData::Vector(self.vector.len()),
            "matrices" => DimData::Matrices((self.matrix1.len(), self.matrix2.len())),
            _ => panic!("Unknown input type {}", ty),
        }
    }
}

pub fn load_data_from_dataset()-> Result<InputData, Error> {
    // Apri il file
    let file = File::open("src/data/dataset/dataset_vector.txt")?;
    let reader = io::BufReader::new(file);

    // Leggi tutte le righe del file in un vettore
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    // Genera un indice casuale per selezionare una linea
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..lines.len());

    // Seleziona la linea e convertila in un vettore di i32
    let selected_line = &lines[random_index];
    let vector: Vec<i32> = selected_line
        .split(",") // Dividi la linea su ","
        .filter_map(|x| x.parse::<i32>().ok()) // Converte ogni elemento in i32
        .collect();



    // Apri il file matrici
    let file = File::open("src/data/dataset/dataset_matrix.txt")?;
    let reader2 = io::BufReader::new(file);

    // Leggi tutte le righe del file in un vettore
    let lines: Vec<String> = reader2.lines().filter_map(Result::ok).collect();

    // Genera un indice casuale per selezionare una linea
    let random_index = rand::thread_rng().gen_range(0..16) * 4;

    let matrix1: Vec<Vec<i32>> = (random_index..random_index + 3)
        .filter_map(|idx| lines.get(idx)) // Recupera la linea, se esiste
        .map(|line| {
            line.split(" ") // Dividi la linea su " "
                .filter_map(|x| x.parse::<i32>().ok()) // Converte ogni elemento in i32
                .collect()
        })
        .collect();

    let scale_factor: i32 = rng.gen_range(1..=10);

    // Crea la matrice identità 3x3
    let identity_matrix: Vec<Vec<i32>> = vec![
        vec![1, 0, 0], // prima riga
        vec![0, 1, 0], // seconda riga
        vec![0, 0, 1], // terza riga
    ];

    // Moltiplica ogni elemento della matrice per il fattore di scala
    // Matrix2 è la scaled matrix
    let matrix2: Vec<Vec<i32>> = identity_matrix
        .into_iter()
        .map(|row| row.into_iter().map(|value| value * scale_factor).collect())
        .collect();

    let matrix_size = 3;

    Ok(InputData {
        vector,
        matrix_size,
        matrix1,
        matrix2,
    })
}
pub fn load_data_from_file(file_path: &str) -> Result<InputData, Error> {
    let path = Path::new(file_path);
    let file = File::open(path)?;
    let mut lines = io::BufReader::new(file).lines();

    // Saltiamo il testo iniziale fino a quando non incontriamo una linea che inizia con un numero
    let mut current_line = String::new();
    while let Some(Ok(line)) = lines.next() {
        let trimmed_line = line.trim();
        if !trimmed_line.is_empty() && trimmed_line.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) {
            current_line = trimmed_line.to_string();
            break;
        }
    }
    // Leggiamo la dimensione del vettore dalla riga corrente
    let vector_size: usize = current_line
        .parse::<usize>()
        .map_err(|_| Error::new(io::ErrorKind::InvalidData, "Formato invalido per la dimensione del vettore"))?;


    // Trova la riga del vettore, saltando righe vuote
    let mut vector_line = String::new();
    while let Some(Ok(line)) = lines.next() {
        if !line.trim().is_empty() {
            vector_line = line;
            break;
        }
    }
    
    let vector: Vec<i32> = vector_line
        .trim()
        .split(',') 
        .map(|n| n.trim()) 
        .map(|n| n.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| Error::new(io::ErrorKind::InvalidData, "Formato invalido nel vettore"))?;

    // Verifica che la dimensione del vettore corrisponda a quella dichiarata
    if vector.len() != vector_size {
        return Err(Error::new(io::ErrorKind::InvalidData, format!("La dimensione del vettore ({}) non corrisponde ai dati forniti ({})", vector_size,vector.len())));
    }

    // Trova la dimensione della matrice, saltando righe vuote
    let mut matrix_size_line = String::new();
    while let Some(Ok(line)) = lines.next() {
        if !line.trim().is_empty() {
            matrix_size_line = line;
            break;
        }
    }

    // Leggi la dimensione delle matrici
    let matrix_size: usize = matrix_size_line
        .trim()
        .parse::<usize>()
        .map_err(|_| Error::new(io::ErrorKind::InvalidData, "Formato invalido per la dimensione della matrice"))?;


    // Leggi la prima matrice
    let mut matrix1 = Vec::new();
    for _ in 0..matrix_size {
        let row_line = loop {
            if let Some(Ok(line)) = lines.next() {
                if !line.trim().is_empty() {
                    break line;
                }
            }
        };

        let row: Vec<i32> = row_line
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<i32>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Formato invalido nelle righe della matrice 1"))?;

        if row.len() != matrix_size {
            return Err(Error::new(io::ErrorKind::InvalidData, "La dimensione della matrice 1 non corrisponde ai dati forniti"));
        }
        matrix1.push(row);
    }

    // Leggi la seconda matrice
    let mut matrix2 = Vec::new();
    for _ in 0..matrix_size {
        let row_line = loop {
            if let Some(Ok(line)) = lines.next() {
                if !line.trim().is_empty() {
                    break line;
                }
            } else {
                return Err(Error::new(
                    io::ErrorKind::InvalidData,
                    "Righe della matrice 2 mancanti",
                ));
            }
        };

        let row: Vec<i32> = row_line
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<i32>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Formato invalido nelle righe della matrice 2"))?;

        if row.len() != matrix_size {
            return Err(Error::new(
                io::ErrorKind::InvalidData,
                "La dimensione della matrice 2 non corrisponde ai dati forniti",
            ));
        }
        matrix2.push(row);
    }
    
    Ok(InputData {
        vector,
        matrix_size,
        matrix1,
        matrix2,
    })
}
fn main() {

    //IMPLEMENTAZIONE MENU UTENTE---------------------------

    // Descrizione iniziale
    println!();
    println!("----------------------------------------------------------------------------");
    println!(" Realizzazione di un ambiente di Fault Injection per applicazione ridondata ");
    println!("----------------------------------------------------------------------------");
    println!();

    let mut file_path: String = "results/".to_string();
    let input_path: String = "src/data/input.txt".to_string();

    let mut nome_file: String = Input::new()
        .with_prompt("Inserisci il nome del file per il report SENZA ESTENSIONE")
        .default("report".to_string())  // Imposta il percorso di default
        .interact_text()
        .unwrap();

    let regex = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();

    while !regex.is_match(&nome_file) {
        println!("Nome file invalido, per favore ritenta");
        nome_file = Input::new()
            .with_prompt("Inserisci il nome del file per il report SENZA ESTENSIONE")
            .default("report".to_string())  // Imposta il percorso di default
            .interact_text()
            .unwrap();
    }
    file_path.push_str(&nome_file);

    // Sorgente dei dati
    let data_sources = vec!["Data file", "Dataset"];
    let data_source_selection = Select::new()
        .with_prompt("Seleziona la sorgente dei dati")
        .default(0)
        .items(&data_sources)
        .interact()
        .unwrap();

    // Caricamento dati in base alla sorgente scelta
    let input_data: InputData = match data_source_selection {
        0 => match load_data_from_file(&input_path) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Errore: {}", e);
                std::process::exit(1);
            }
        },
        1 => match load_data_from_dataset() {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Errore: {}", e);
                std::process::exit(1);
            }
        },
        _ => unreachable!(),
    };

    // Scelta tra singolo algoritmo o tutti
    let operation_modes = vec!["Esegui un singolo algoritmo", "Esegui un'analisi comparativa tra tutti gli algoritmi"];
    let mode_selection = Select::new()
        .with_prompt("Seleziona il tipo di analisi")
        .items(&operation_modes)
        .default(0)
        .interact()
        .unwrap();

    match mode_selection {

        // Caso del singolo algoritmo
        0 => {
            // scelta algoritmo
            let options = vec![
                "Selection Sort",
                "Bubble Sort",
                "Matrix Multiplication"
            ];

            // Menu di selezione
            let algo_selection = Select::new()
                .with_prompt("Scegli un algoritmo da utilizzare")
                .default(0) // Selezione predefinita
                .items(&options)
                .interact()
                .unwrap();

            //--------------------------------------------------------------------------

            // scelta tra H/non H, variazione tra #fault list
            let options = vec![
                "Digita una cardinalità a piacere per la fault list entry",
                "Tre esecuzioni con cardinalità della fault list entry che varia [1000, 2000, 3000]",
            ];

            let single_algo_anlysis_selection = Select::new()
                .with_prompt("Scegli una modalità di single analysis")
                .default(0) // Selezione predefinita
                .items(&options)
                .interact()
                .unwrap();

            match single_algo_anlysis_selection {
                //single run su fault entries desiderate
                0 => {
                        let num_faults = Input::new()
                        .with_prompt("Inserisci il numero di fault entries desiderate")
                        .default(2000)
                        .interact_text()
                        .unwrap();

                        file_path.push_str(".pdf");

                    match algo_selection {
                        //single run selection sort
                        0 => {
                            run_case_study(
                                0,
                                num_faults,
                                "sel_sort",
                                &file_path,
                                input_data.into_data("vector"),
                                input_data.into_dimdata("vector"),
                                Instant::now(),
                                "src/fault_list_manager/file_fault_list/selection_sort/mod.rs",
                                "src/fault_list_manager/file_fault_list/selection_sort/sel_sort_ris.json",
                                "src/fault_list_manager/file_fault_list/selection_sort/sel_sort_FL.json",
                                |vettore| run_for_count_selection_sort(vettore));
                        }

                        //single run bubble sort
                        1 => {
                            run_case_study(
                                1,
                                num_faults,
                                "bubble_sort",
                                &file_path,
                                input_data.into_data("vector"),
                                input_data.into_dimdata("vector"),
                                Instant::now(),
                                "src/fault_list_manager/file_fault_list/bubble_sort/mod.rs",
                                "src/fault_list_manager/file_fault_list/bubble_sort/bubble_sort_ris.json",
                                "src/fault_list_manager/file_fault_list/bubble_sort/bubble_sort_FL.json",
                                |vettore| run_for_count_bubble_sort(vettore)
                            );
                        }

                        //single run matrix multiplication
                        2 => {
                            run_case_study(
                                2,
                                num_faults,
                                "matrix_multiplication",
                                &file_path,
                                input_data.into_data("matrices"),
                                input_data.into_dimdata("matrices"),
                                Instant::now(),
                                "src/fault_list_manager/file_fault_list/matrix_multiplication/mod.rs",
                                "src/fault_list_manager/file_fault_list/matrix_multiplication/matrix_mul_ris.json",
                                "src/fault_list_manager/file_fault_list/matrix_multiplication/matrix_mul_FL.json",
                                |matrici| run_for_count_matrix_mul(matrici,input_data.matrix_size)
                            );
                        }

                        _ => println!("Invalid selection."),
                    }
                }

                //tre run su 1000 2000 3000 fault entries
                1 => {
                    file_path.push_str("_diffcard.pdf");
                    let cardinalities: Vec<i32> = vec![1000, 2000, 3000];
                    match algo_selection {
                        0 => {
                            let mut esecuzione = 0;
                            // Caso studio 1: Selection Sort
                            for cardinality in cardinalities{
                                println!("Esecuzione Selection Sort con cardinalità:{}",cardinality);
                                run_case_study(
                                    esecuzione,
                                    cardinality,
                                    "sel_sort",
                                    &file_path,
                                    input_data.into_data("vector"),
                                    input_data.into_dimdata("vector"),
                                    Instant::now(),
                                    "src/fault_list_manager/file_fault_list/selection_sort/mod.rs",
                                    "src/fault_list_manager/file_fault_list/selection_sort/sel_sort_ris.json",
                                    "src/fault_list_manager/file_fault_list/selection_sort/sel_sort_FL.json",
                                    |vettore| run_for_count_selection_sort(vettore)
                                );
                                esecuzione += 1;
                            }
                        }
                        1 => {
                            // Caso studio 2: Bubble Sort
                            let mut esecuzione = 0;
                            for cardinality in cardinalities{
                                println!("Esecuzione Bubble Sort con cardinalità:{}",cardinality);
                                run_case_study(
                                    esecuzione,
                                    cardinality,
                                    "bubble_sort",
                                    &file_path,
                                    input_data.into_data("vector"),
                                    input_data.into_dimdata("vector"),
                                    Instant::now(),
                                    "src/fault_list_manager/file_fault_list/bubble_sort/mod.rs",
                                    "src/fault_list_manager/file_fault_list/bubble_sort/bubble_sort_ris.json",
                                    "src/fault_list_manager/file_fault_list/bubble_sort/bubble_sort_FL.json",
                                    |vettore| run_for_count_bubble_sort(vettore)
                                );
                                esecuzione += 1;
                            }
                        }
                        2 => {
                            // Caso studio 3: Matrix Multiplication
                            let mut esecuzione = 0;
                            for cardinality in cardinalities {
                                println!("Esecuzione Matrix Multiplication con cardinalità:{}",cardinality);
                                run_case_study(
                                    esecuzione,
                                    cardinality,
                                    "matrix_multiplication",
                                    &file_path,
                                    input_data.into_data("matrices"),
                                    input_data.into_dimdata("matrices"),
                                    Instant::now(),
                                    "src/fault_list_manager/file_fault_list/matrix_multiplication/mod.rs",
                                    "src/fault_list_manager/file_fault_list/matrix_multiplication/matrix_mul_ris.json",
                                    "src/fault_list_manager/file_fault_list/matrix_multiplication/matrix_mul_FL.json",
                                    |matrici| run_for_count_matrix_mul(matrici,input_data.matrix_size)
                                );
                                esecuzione += 1;
                            }
                        }
                        _ => println!("Invalid selection."),
                    }
                }

                _ => println!("Invalid selection."),
            }
        }

        //caso tutti gli algoritmi
        1 => {
            // Esegui tutti gli algoritmi
            let num_faults = Input::new()
                .with_prompt("Inserisci il numero di fault entries desiderate")
                .default(2000)
                .interact_text()
                .unwrap();
            file_path.push_str("_all.pdf");


            // Caso studio 1: Selection Sort
            let mut esecuzione = 0;
            println!("Esecuzione Selection Sort");
            run_case_study(
                esecuzione,
                num_faults,
                "sel_sort",
                &file_path,
                input_data.into_data("vector"),
                input_data.into_dimdata("vector"),
                Instant::now(),
                "src/fault_list_manager/file_fault_list/selection_sort/mod.rs",
                "src/fault_list_manager/file_fault_list/selection_sort/sel_sort_ris.json",
                "src/fault_list_manager/file_fault_list/selection_sort/sel_sort_FL.json",
                |vettore| run_for_count_selection_sort(vettore)
            );

            esecuzione += 1;
            println!("Esecuzione Bubble Sort");
            // Caso studio 2: Bubble Sort
            run_case_study(
                esecuzione,
                num_faults,
                "bubble_sort",
                &file_path,
                input_data.into_data("vector"),
                input_data.into_dimdata("vector"),
                Instant::now(),
                "src/fault_list_manager/file_fault_list/bubble_sort/mod.rs",
                "src/fault_list_manager/file_fault_list/bubble_sort/bubble_sort_ris.json",
                "src/fault_list_manager/file_fault_list/bubble_sort/bubble_sort_FL.json",
                |vettore| run_for_count_bubble_sort(vettore)
            );

            esecuzione += 1;
            println!("Esecuzione Matrix Multiplication");
            // Caso studio 3: Matrix Multiplication
            run_case_study(
                esecuzione,
                num_faults,
                "matrix_multiplication",
                &file_path,
                input_data.into_data("matrices"),
                input_data.into_dimdata("matrices"),
                Instant::now(),
                "src/fault_list_manager/file_fault_list/matrix_multiplication/mod.rs",
                "src/fault_list_manager/file_fault_list/matrix_multiplication/matrix_mul_ris.json",
                "src/fault_list_manager/file_fault_list/matrix_multiplication/matrix_mul_FL.json",
                |matrici| run_for_count_matrix_mul(matrici,input_data.matrix_size)
            );
        }

        _ => unreachable!(),
    }
    println!("Operazione completata. Report salvato in: {}", file_path);


    // Avvia la pipeline
    fn run_case_study(esecuzione:i8,
                      num_faults: i32,
                      case_name: &str,
                      file_path: &str,
                      input_data: Data<i32>,
                      dim_data: DimData,
                      start: Instant,
                      analysis_input_file: &str,
                      analysis_output_file: &str,
                      fault_list_file: &str,
                      fault_list_run: impl FnOnce(Data<i32>) -> usize){
        // 1. Analisi statica del codice

        static_analysis::generate_analysis_file(
            analysis_input_file.to_string(),
            analysis_output_file.to_string(),
        ).expect("Analisi statica del codice fallita");

        // 2. Generazione della fault list (FL)
        fault_list_manager::create_fault_list(
            num_faults,
            case_name.to_string(),
            analysis_output_file.to_string(),
            dim_data,
            fault_list_file.to_string(),
            fault_list_run(input_data.clone()),
        );

        // 3. Faccio partire l'ambiente di fault injection
        fault_injection_env(
            fault_list_file.to_string(),
            case_name.to_string(),
            file_path.to_string(),
            input_data.clone(),
            start,
            esecuzione
        );
    }
}