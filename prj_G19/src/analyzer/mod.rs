use std::fs;
use std::sync::mpsc::{Receiver};
use std::time::Instant;
use serde::{Deserialize, Serialize};
use crate::fault_env::Data;
use crate::fault_list_manager::file_fault_list::{bubble_sort, matrix_multiplication, selection_sort};
use crate::hardened::{bubble_sort_hardened, matrix_multiplication_hardened, selection_sort_hardened, Hardened, IncoherenceError, IntoNestedVec};
use crate::injector::TestResult;
use crate::{pdf_generator, VERBOSE};

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Faults{
    pub(crate) n_silent_fault: usize,
    pub(crate) n_assign_fault: usize,
    pub(crate) n_inner_fault: usize,
    pub(crate) n_sub_fault: usize,
    pub(crate) n_mul_fault: usize,
    pub(crate) n_add_fault: usize,
    pub(crate) n_indexmut_fault: usize,
    pub(crate) n_index_fault: usize,
    pub(crate) n_ord_fault: usize,
    pub(crate) n_partialord_fault: usize,
    pub(crate) n_partialeq_fault: usize,
    pub(crate) n_fatal_fault: usize,
    pub(crate) total_fault: usize,
}
pub struct FaultsIter<'a> {
    faults: &'a Faults,
    index: usize,
}

impl Faults {
    // Metodo per creare l'iteratore
    pub fn iter(&self) -> FaultsIter {
        FaultsIter {
            faults: self,
            index: 0,
        }
    }
}

impl<'a> Iterator for FaultsIter<'a> {
    type Item = (&'static str, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.index {
            0 => Some(("n_silent_fault", self.faults.n_silent_fault)),
            1 => Some(("n_assign_fault", self.faults.n_assign_fault)),
            2 => Some(("n_inner_fault", self.faults.n_inner_fault)),
            3 => Some(("n_sub_fail",self.faults.n_sub_fault)),
            4 => Some(("n_mul_fault", self.faults.n_mul_fault)),
            5 => Some(("n_add_fault", self.faults.n_add_fault)),
            6 => Some(("n_index_fault", self.faults.n_index_fault)),
            7 => Some(("n_partialord_fault", self.faults.n_partialord_fault)),
            _ => None,
        };
        self.index += 1;
        result
    }
}
#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Analyzer{
    pub(crate) n_esecuzione: i8,
    pub(crate) faults: Faults,
    pub(crate) input: Data<i32>,
    pub(crate) output: Data<i32>,
    pub(crate) time_experiment: f64,
    pub(crate) time_alg_hardened: f64,
    pub(crate) time_alg_not_hardened: f64,
    pub(crate) byte_hardened: f64,
    pub(crate) byte_not_hardened: f64,
    pub(crate) target_program: String,
}

impl Analyzer{
    pub(crate) fn new(faults: Faults, time_exp:f64, n_esecuzione:i8, target: String) -> Self{
        let input:Data<i32> = match target.as_str() {
            "matrix_multiplication" => Data::Matrices(Vec::new(),Vec::new()),
            _ => Data::Vector(Vec::new())
        };
        let output:Data<i32> = match target.as_str() {
            "matrix_multiplication" => Data::Matrices(Vec::new(),Vec::new()),
            _ => Data::Vector(Vec::new())
        };

        Analyzer{
            n_esecuzione,
            faults,
            time_experiment: time_exp,
            input,
            output,
            time_alg_hardened: 0.0,
            time_alg_not_hardened: 0.0,
            byte_hardened: 0.0,
            byte_not_hardened: 0.0,
            target_program: target
        }
    }

}

impl Faults{
    fn new() -> Faults {
        Faults {
            n_silent_fault: 0,
            n_assign_fault: 0,
            n_inner_fault: 0,
            n_sub_fault:0,
            n_mul_fault: 0,
            n_add_fault: 0,
            n_indexmut_fault: 0,
            n_index_fault: 0,
            n_ord_fault: 0,
            n_partialord_fault: 0,
            n_partialeq_fault: 0,
            n_fatal_fault:0,
            total_fault: 0,
        }
    }
}

pub fn run_analyzer(rx_chan_inj_anl: Receiver<TestResult>, file_path:String, data: Data<i32>,
                target:String, n_esecuzione:i8, time_experiment:f64) {
    let mut vec_result = Vec::new();
    let mut faults = Faults::new();
    while let Ok(test_result) = rx_chan_inj_anl.recv() {
        vec_result.push(test_result);
    }


    let mut v_ok = Vec::new();
    let mut fault_list_ok = Vec::new();
    for test_result in &vec_result {

        let res = test_result.get_result();


        if res.is_ok() {
            faults.n_silent_fault += 1;
            v_ok.push(res.unwrap());
            fault_list_ok.push(test_result.get_fault_list_entry());
        } else {
            match res.err().unwrap() {
                IncoherenceError::AssignFail => faults.n_assign_fault += 1,
                IncoherenceError::AddFail => faults.n_add_fault += 1,
                IncoherenceError::MulFail => faults.n_mul_fault += 1,
                IncoherenceError::InnerFail => faults.n_inner_fault += 1,
                IncoherenceError::SubFail => faults.n_sub_fault += 1,
                IncoherenceError::IndexMutFail => faults.n_indexmut_fault += 1,
                IncoherenceError::IndexFail => faults.n_index_fault += 1,
                IncoherenceError::OrdFail => faults.n_ord_fault += 1,
                IncoherenceError::PartialOrdFail => faults.n_partialord_fault += 1,
                IncoherenceError::PartialEqFail => faults.n_partialeq_fault += 1
            }
        }
    }
    faults.total_fault =  faults.n_silent_fault + faults.n_assign_fault + faults.n_add_fault +
                            faults.n_mul_fault + faults.n_inner_fault + faults.n_sub_fault +
                            faults.n_indexmut_fault + faults.n_index_fault + faults.n_ord_fault +
                            faults.n_partialord_fault + faults.n_partialeq_fault;


    let mut analyzer = Analyzer::new(faults,time_experiment, n_esecuzione,target);
    analyzer.input = data;
    get_data_for_dimension_table(&mut analyzer).unwrap();
    get_data_for_time_table(&mut analyzer).unwrap();

    let correct_ouput = match analyzer.target_program.as_str() {
        "matrix_multiplication" => {analyzer.output.clone().into_matrices().0.into_iter().flatten().collect::<Vec<i32>>()}
        _ => {analyzer.output.clone().into_vector()}
    };
    let mut i = 0;
    if VERBOSE {
        println!("##########################################################################");
        println!("-----INIEZIONI CHE HANNO PORTATO AD UN FAULT SILENT CON OUTPUT ERRATO-----");
        println!("##########################################################################");
    }
    for v in v_ok{
        if correct_ouput != v.into_nested_vec() {
            if VERBOSE {
                println!("Fault #{} {:?}", analyzer.faults.n_fatal_fault, fault_list_ok[i]);
            }
            analyzer.faults.n_fatal_fault += 1;
        }
        i=i+1;
    }

    let json_path = "results/tmp.json";
    // 1. Leggi il contenuto esistente del file (o array vuoto se è stato appena creato)
    let mut data_list: Vec<Analyzer> = match fs::read_to_string(json_path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => Vec::new(),
    };
    if file_path.contains("_all") || file_path.contains("_diffcard") {
        if n_esecuzione == 0{
            let empty_json = "[]";
            fs::write(json_path, empty_json).expect("Failed to create the JSON file");
        }

        // 2. Aggiungi nuovi dati all'array
        data_list.push(analyzer.clone());

        // 3. Riscrivi il file JSON con l'array aggiornato
        let json_string = serde_json::to_string_pretty(&data_list).expect("Serialization failed");
        fs::write(json_path, json_string).expect("Unable to write to file");

        if n_esecuzione == 2 {
           fs::remove_file(json_path).expect("Failed to delete the JSON file");

           if file_path.contains("_all"){
               pdf_generator::print_pdf_all(&file_path,data_list);
           }else{
               pdf_generator::print_pdf_diffcard(&file_path,data_list);
           }
        }
    }else{
        pdf_generator::print_pdf_singolo(&file_path,analyzer);
    }

}
fn get_data_for_dimension_table(analyzer: &mut Analyzer) -> Result<(),String>{
    let file_path_nothardened = match analyzer.target_program.as_str() {
        "sel_sort" => "src/fault_list_manager/file_fault_list/selection_sort/mod.rs",
        "bubble_sort" => "src/fault_list_manager/file_fault_list/bubble_sort/mod.rs",
        "matrix_multiplication" => "src/fault_list_manager/file_fault_list/matrix_multiplication/mod.rs",
        _ => "",
    };
    let metadata_not_hard = fs::metadata(file_path_nothardened);

    let file_path_hardened = match analyzer.target_program.as_str() {
        "sel_sort" => "src/hardened/selection_sort_hardened/mod.rs",
        "bubble_sort" => "src/hardened/bubble_sort_hardened/mod.rs",
        "matrix_multiplication" => "src/hardened/matrix_multiplication_hardened/mod.rs",
        _ => "",
    };
    let metadata_hard = fs::metadata(file_path_hardened);
    if metadata_not_hard.is_ok() && metadata_hard.is_ok() {
        analyzer.byte_not_hardened = metadata_not_hard.unwrap().len() as f64;
        analyzer.byte_hardened = metadata_hard.unwrap().len() as f64;
    }else{
        return Err(format!("il path del file: {} non è valido",file_path_nothardened));
    }
    Ok(())
}

fn get_data_for_time_table(analyzer: &mut Analyzer) -> Result<(),String>{
    let data = analyzer.input.clone();
    let data_hard = analyzer.input.clone();
    analyzer.time_alg_not_hardened = match analyzer.target_program.as_str() {
        "sel_sort" => {
            let start_sel_sort = Instant::now();
            analyzer.output = selection_sort::selection_sort(data.into_vector()).into();
            (start_sel_sort.elapsed().as_nanos() as f64)/1000.0
        },
        "bubble_sort" => {
            let start_bb_sort = Instant::now();
            analyzer.output = bubble_sort::bubble_sort(data.into_vector()).into();
            (start_bb_sort.elapsed().as_nanos() as f64)/1000.0
        },
        "matrix_multiplication" => {
            let start_mat_multiplication =  Instant::now();
            let matrices=  data.into_matrices();
            analyzer.output = matrix_multiplication::matrix_multiplication(matrices.0,matrices.1).into();
            (start_mat_multiplication.elapsed().as_nanos() as f64)/1000.0
        },
        _ => return Err("Indice non valido".to_string()),
    };
    analyzer.time_alg_hardened= match analyzer.target_program.as_str() {
        "sel_sort" => {
            let start_sel_sort = Instant::now();
            selection_sort_hardened::selection_sort(&mut Hardened::from_vec(data_hard.into_vector())).unwrap();
            (start_sel_sort.elapsed().as_nanos() as f64)/1000.0
        },
        "bubble_sort" => {
            let start_bb_sort = Instant::now();
            bubble_sort_hardened::bubble_sort(&mut Hardened::from_vec(data_hard.into_vector())).unwrap();
            (start_bb_sort.elapsed().as_nanos() as f64)/1000.0
        },
        "matrix_multiplication" => {
            let start_mat_multiplication =  Instant::now();
            let matrices=  data_hard.into_matrices();
            matrix_multiplication_hardened::matrix_multiplication(&mut Hardened::from_mat(matrices.0),&mut Hardened::from_mat(matrices.1)).unwrap();
            (start_mat_multiplication.elapsed().as_nanos() as f64)/1000.0
        },
        _ => return Err("Indice non valido".to_string()),
    };
    Ok(())
}
#[cfg(test)]
mod tests{
    use rand::Rng;
    use crate::analyzer::{get_data_for_dimension_table, get_data_for_time_table, Analyzer, Faults};
    #[test]
    fn try_get_execution_times(){
        let faults = Faults {
            n_silent_fault: 1,
            n_assign_fault: 2,
            n_mul_fault: 3,
            n_inner_fault: 3,
            n_sub_fault: 8,
            n_add_fault: 5,
            n_indexmut_fault: 6,
            n_index_fault: 7,
            n_ord_fault: 8,
            n_partialord_fault: 9,
            n_partialeq_fault: 10,
            n_fatal_fault: 22,
            total_fault: 55,
        };
        let mut analyzer = Analyzer::new(faults,389.0,1,"sel_sort".to_string());
        let mut rng = rand::thread_rng();
        let vec: Vec<i32> = (0..3000).map(|_| rng.gen_range(0..20)).collect();
        println!("{:?}", vec);
        let tim = get_data_for_time_table(&mut analyzer);
        if tim.is_ok(){
           assert_eq!(tim.is_ok(), true);
        }else{
            println!("{}",tim.unwrap_err());
        }
    }
    #[test]
    fn try_get_files_dimensions(){
        let faults = Faults {
            n_silent_fault: 1,
            n_assign_fault: 2,
            n_mul_fault: 3,
            n_inner_fault: 3,
            n_sub_fault: 8,
            n_add_fault: 5,
            n_indexmut_fault: 6,
            n_index_fault: 7,
            n_ord_fault: 8,
            n_partialord_fault: 9,
            n_partialeq_fault: 10,
            n_fatal_fault: 22,
            total_fault: 55,
        };
        let mut analyzer = Analyzer::new(faults,389.0,1,"sel_sort".to_string());
        let dim = get_data_for_dimension_table(&mut analyzer);
        if dim.is_ok(){
            assert_eq!(dim.is_ok(), true);
        }else{
            println!("{}",dim.unwrap_err());
        }
    }

    #[test]
    fn try_iterator_faults(){
        let faults = Faults {
            n_silent_fault: 1,
            n_assign_fault: 2,
            n_mul_fault: 3,
            n_inner_fault: 3,
            n_sub_fault: 8,
            n_add_fault: 5,
            n_indexmut_fault: 6,
            n_index_fault: 7,
            n_ord_fault: 8,
            n_partialord_fault: 9,
            n_partialeq_fault: 10,
            n_fatal_fault: 22,
            total_fault: 55,
        };
        let ref_iter = &faults;
        for (name, value) in ref_iter.iter() {
            println!("{}: {}", name, value.to_string());
        }
    }
}

