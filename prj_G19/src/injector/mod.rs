mod algorithms;

use std::sync::{Arc, RwLock};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::{panic, thread, vec};
use crate::fault_list_manager::FaultListEntry;
use crate::hardened::{Hardened, IncoherenceError};
use algorithms::{runner_selection_sort};
use crate::fault_env::Data;
use crate::injector::algorithms::{runner_bubble_sort, runner_matrix_multiplication};
use crate::VERBOSE;

#[allow(dead_code)]
#[derive(Debug)]
pub struct TestResult {
    fault_list_entry: FaultListEntry,
    result: Result<Vec<Hardened<i32>>, IncoherenceError>
}
impl TestResult {
    pub fn get_result(&self) -> Result<Vec<Hardened<i32>>, IncoherenceError> {
        self.result.clone()
    }

    pub fn get_fault_list_entry(&self) -> FaultListEntry {
        self.fault_list_entry.clone()
    }
}
enum AlgorithmVariables {
    SelectionSort(SelectionSortVariables),
    BubbleSort(BubbleSortVariables),
    MatrixMultiplication(MatrixMultiplicationVariables),
}

struct SelectionSortVariables {
    i: RwLock<Hardened<usize>>,
    j: RwLock<Hardened<usize>>,
    n: RwLock<Hardened<usize>>,
    min: RwLock<Hardened<usize>>,
    vec: RwLock<Vec<Hardened<i32>>>,
}

struct BubbleSortVariables {
    i: RwLock<Hardened<usize>>,
    j: RwLock<Hardened<usize>>,
    n: RwLock<Hardened<usize>>,
    swapped: RwLock<Hardened<bool>>,
    vet: RwLock<Vec<Hardened<i32>>>,
}

struct MatrixMultiplicationVariables {
    size: RwLock<Hardened<usize>>,
    i: RwLock<Hardened<usize>>,
    j: RwLock<Hardened<usize>>,
    k: RwLock<Hardened<usize>>,
    acc: RwLock<Hardened<i32>>,
    a: RwLock<Vec<Vec<Hardened<i32>>>>,
    b: RwLock<Vec<Vec<Hardened<i32>>>>,
    result: RwLock<Vec<Vec<Hardened<i32>>>>
}

// Common initialization trait
trait VariableSet {
    type Input;
    fn new(input: Self::Input) -> Self;
}

impl VariableSet for SelectionSortVariables {
    type Input = Vec<i32>;
    fn new(vec: Vec<i32>) -> Self {
        SelectionSortVariables {
            i: RwLock::new(Hardened::from(0)),
            j: RwLock::new(Hardened::from(0)),
            min: RwLock::new(Hardened::from(0)),
            n: RwLock::new(Hardened::from(0)),
            vec: RwLock::new(Hardened::from_vec(vec))
        }
    }
}

impl VariableSet for BubbleSortVariables {
    type Input = Vec<i32>;
    fn new(vet: Vec<i32>) -> Self {
        BubbleSortVariables {
            i: RwLock::new(Hardened::from(0)),
            j: RwLock::new(Hardened::from(0)),
            swapped: RwLock::new(Hardened::from(false)),
            n: RwLock::new(Hardened::from(0)),
            vet: RwLock::new(Hardened::from_vec(vet))
        }
    }
}

impl VariableSet for MatrixMultiplicationVariables {
    type Input = (Vec<Vec<i32>>, Vec<Vec<i32>>);
    fn new((a, b): (Vec<Vec<i32>>, Vec<Vec<i32>>)) -> Self {
        MatrixMultiplicationVariables {
            size: RwLock::new(Hardened::from(0)),
            i: RwLock::new(Hardened::from(0)),
            j: RwLock::new(Hardened::from(0)),
            k: RwLock::new(Hardened::from(0)),
            acc: RwLock::new(Hardened::from(0)),
            a: RwLock::new(Hardened::from_mat(a)),
            b: RwLock::new(Hardened::from_mat(b)),
            result: RwLock::new(Hardened::from_mat(Vec::new()))
        }
    }
}


impl AlgorithmVariables {
    fn from_target(target: &str, data: Data<i32>) -> Arc<AlgorithmVariables> {
        match target {
            "sel_sort" => Arc::new(AlgorithmVariables::SelectionSort(SelectionSortVariables::new(data.into_vector()))),
            "bubble_sort" => Arc::new(AlgorithmVariables::BubbleSort(BubbleSortVariables::new(data.into_vector()))),
            "matrix_multiplication" => Arc::new(AlgorithmVariables::MatrixMultiplication(MatrixMultiplicationVariables::new(data.into_matrices()))),
            _ => panic!("Unknown target algorithm"),
        }
    }
}

fn runner(variables: Arc<AlgorithmVariables>, fault_list_entry: FaultListEntry, tx_runner: Sender<&str>, rx_runner: Receiver<&str>) -> TestResult {

    let result = panic::catch_unwind(|| {
        match &*variables {
            AlgorithmVariables::SelectionSort(var) => {
                runner_selection_sort(var, tx_runner, rx_runner)
            }
            AlgorithmVariables::BubbleSort(var) => {
                runner_bubble_sort(var, tx_runner, rx_runner)
            }
            AlgorithmVariables::MatrixMultiplication(var) => {
                runner_matrix_multiplication(var, tx_runner, rx_runner)
            }
        }
    });



    match result {
        Ok(Ok(v)) => TestResult {result: Ok(v), fault_list_entry},
        Ok(Err(err)) => {
            if VERBOSE {
                println!("Error found - {:?}", err);
            }
            TestResult {result: Err(err), fault_list_entry}
        },
        Err(panic) => {
            let msg = panic_message::panic_message(&panic);

            return match msg {
                m if m.contains("IndexMut") => TestResult { result: Err(IncoherenceError::IndexMutFail), fault_list_entry },
                m if m.contains("Index") => TestResult { result: Err(IncoherenceError::IndexFail), fault_list_entry },
                m if m.contains("PartialOrd") => TestResult { result: Err(IncoherenceError::PartialOrdFail), fault_list_entry },
                m if m.contains("Ord") => TestResult { result: Err(IncoherenceError::OrdFail), fault_list_entry },
                _ => TestResult { result: Err(IncoherenceError::PartialEqFail), fault_list_entry },
            }
        }
    }
}



fn injector(variables: Arc<AlgorithmVariables>, fault_list_entry: FaultListEntry, tx_injector: Sender<&str>, rx_runner: Receiver<&str>) {

    let mut counter = 0usize;

    // dato che fault_mask mi dice la posizione del bit da modificare, per ottenere la maschera devo calcolare 2^fault_mask
    let mask = 1 << (fault_list_entry.flipped_bit);

    //println!("mask: {}", 1 << (fault_list_entry.fault_mask));       // ottengo la maschera

    while let Ok(_) = rx_runner.recv() {
        counter += 1;
        if counter == fault_list_entry.time {
            match &*variables {
                AlgorithmVariables::SelectionSort(var) => {
                    match fault_list_entry.var.as_str() {
                        "i" => {
                            let val = var.i.read().unwrap().inner().unwrap().clone();     // leggo il valore della variabile
                            let new_val = val ^ mask;                                           // nuovo valore da salvare (XOR per il bitflip)
                            var.i.write().unwrap()["cp1"] = new_val;                            // inietto l'errore
                        },
                        "j" => {
                            let val = var.j.read().unwrap().inner().unwrap().clone();     // leggo il valore della variabile
                            let new_val = val ^ mask;                                           // nuovo valore da salvare (XOR per il bitflip)
                            var.j.write().unwrap()["cp1"] = new_val;                            // inietto l'errore
                        },
                        "n" => {
                            let val = var.n.read().unwrap().inner().unwrap().clone();     // leggo il valore della variabile
                            let new_val = val ^ mask;                                           // nuovo valore da salvare (XOR per il bitflip)
                            var.n.write().unwrap()["cp1"] = new_val;                            // inietto l'errore
                        },
                        "min" => {
                            let val = var.min.read().unwrap().inner().unwrap().clone();     // leggo il valore della variabile
                            let new_val = val ^ mask;                                             // nuovo valore da salvare (XOR per il bitflip)
                            var.min.write().unwrap()["cp1"] = new_val;                            // inietto l'errore
                        },
                        _ => {
                            let index = fault_list_entry.var
                                .split(|c| c == '[' || c == ']')
                                .collect::<Vec<_>>()[1]
                                .parse::<usize>().unwrap(); // ottengo l'indice dell'elemento nel vttore in cui iniettare l'errore

                            let val = var.vec.read().unwrap()[index].inner().unwrap().clone();
                            let new_val = val ^ (mask as i32);
                            var.vec.write().unwrap()[index]["cp1"] = new_val;
                        }
                    };
                }
                AlgorithmVariables::BubbleSort(var) => {
                    match fault_list_entry.var.as_str() {
                        "i" => {
                            let val = var.i.read().unwrap().inner().unwrap().clone();     // leggo il valore della variabile
                            let new_val = val ^ mask;                                           // nuovo valore da salvare (XOR per il bitflip)
                            var.i.write().unwrap()["cp1"] = new_val;                            // inietto l'errore
                        },
                        "j" => {
                            let val = var.j.read().unwrap().inner().unwrap().clone();     // leggo il valore della variabile
                            let new_val = val ^ mask;                                           // nuovo valore da salvare (XOR per il bitflip)
                            var.j.write().unwrap()["cp1"] = new_val;                            // inietto l'errore
                        },
                        "n" => {
                            let val = var.n.read().unwrap().inner().unwrap().clone();     // leggo il valore della variabile
                            let new_val = val ^ mask;                                           // nuovo valore da salvare (XOR per il bitflip)
                            var.n.write().unwrap()["cp1"] = new_val;                            // inietto l'errore
                        },
                        "swapped" => {
                            let val = var.swapped.read().unwrap().inner().unwrap().clone();     // leggo il valore della variabile
                            let new_val = !val;                                             // nuovo valore da salvare (XOR per il bitflip)
                            var.swapped.write().unwrap()["cp1"] = new_val;                            // inietto l'errore
                        },
                        _ => {
                            let index = fault_list_entry.var
                                .split(|c| c == '[' || c == ']')
                                .collect::<Vec<_>>()[1]
                                .parse::<usize>().unwrap(); // ottengo l'indice dell'elemento nel vttore in cui iniettare l'errore

                            let val = var.vet.read().unwrap()[index].inner().unwrap().clone();
                            let new_val = val ^ (mask as i32);
                            var.vet.write().unwrap()[index]["cp1"] = new_val;
                        }
                    };
                }
                AlgorithmVariables::MatrixMultiplication(var) => {
                    match fault_list_entry.var.as_str() {
                        "i" => {
                            let val = var.i.read().unwrap().inner().unwrap().clone();     // leggo il valore della variabile
                            let new_val = val ^ mask;                                           // nuovo valore da salvare (XOR per il bitflip)
                            var.i.write().unwrap()["cp1"] = new_val;                            // inietto l'errore
                        },
                        "j" => {
                            let val = var.j.read().unwrap().inner().unwrap().clone();     // leggo il valore della variabile
                            let new_val = val ^ mask;                                           // nuovo valore da salvare (XOR per il bitflip)
                            var.j.write().unwrap()["cp1"] = new_val;                            // inietto l'errore
                        },
                        "k" => {
                            let val = var.k.read().unwrap().inner().unwrap().clone();     // leggo il valore della variabile
                            let new_val = val ^ mask;                                           // nuovo valore da salvare (XOR per il bitflip)
                            var.k.write().unwrap()["cp1"] = new_val;                            // inietto l'errore
                        },
                        "size" => {
                            let val = var.size.read().unwrap().inner().unwrap().clone();     // leggo il valore della variabile
                            let new_val = !val;                                             // nuovo valore da salvare (XOR per il bitflip)
                            var.size.write().unwrap()["cp1"] = new_val;                            // inietto l'errore
                        },
                        "acc" => {
                            let val = var.acc.read().unwrap().inner().unwrap().clone();     // leggo il valore della variabile
                            let new_val = !val;                                             // nuovo valore da salvare (XOR per il bitflip)
                            var.acc.write().unwrap()["cp1"] = new_val;                            // inietto l'errore
                        },
                        _ => {
                            let parts: Vec<&str> = fault_list_entry.var.split(|c| c == '[').collect();
                            // Extract the matrix name
                            let matrix_name = parts[0];

                            // Extract indices
                            let indices = fault_list_entry.var
                                .split(|c| c == '[' || c == ']')
                                .filter(|&s| !s.is_empty() && s.chars().all(|c| c.is_digit(10))) // Filter to get only numeric parts
                                .map(|s| s.parse::<usize>().unwrap())
                                .collect::<Vec<_>>();

                            if indices.len() == 2 {
                                let row = indices[0];
                                let col = indices[1];

                                match matrix_name {
                                    "a" => {
                                        let val = var.a.read().unwrap()[row][col].inner().unwrap().clone();
                                        let new_val = val ^ (mask as i32);
                                        var.a.write().unwrap()[row][col]["cp1"] = new_val;
                                    }
                                    "b" => {
                                        let val = var.b.read().unwrap()[row][col].inner().unwrap().clone();
                                        let new_val = val ^ (mask as i32);
                                        var.b.write().unwrap()[row][col]["cp1"] = new_val;
                                    }
                                    "result" => {
                                        let val = var.result.read().unwrap()[row][col].inner().unwrap().clone();
                                        let new_val = val ^ (mask as i32);
                                        var.result.write().unwrap()[row][col]["cp1"] = new_val;
                                    }
                                    _ => {
                                        println!("non e' una matrice che conosco")
                                    }
                                }
                            } else {
                                println!("numero di indici diverso da 2");
                            }
                            /*
                            let str = fault_list_entry.var.as_str();
                            if str.starts_with("row") {
                                //println!("processo {:?}", str);
                                let index = fault_list_entry.var
                                    .split(|c| c == '[' || c == ']')
                                    .collect::<Vec<_>>()[1]
                                    .parse::<usize>().unwrap(); // ottengo l'indice dell'elemento nel vttore in cui iniettare l'errore
                                println!("{:?}",var.row.read());
                                let val = var.row.read().unwrap()[index].inner().unwrap().clone();
                                //println!("continuo");
                                let new_val = val ^ (mask as i32);
                                var.row.write().unwrap()[index]["cp1"] = new_val;
                                //println!("fatto {:?}", str);
                            }
                            else {
                                let parts: Vec<&str> = fault_list_entry.var.split(|c| c == '[').collect();


                                // Extract the matrix name
                                let matrix_name = parts[0];

                                // Extract indices
                                let indices = fault_list_entry.var
                                    .split(|c| c == '[' || c == ']')
                                    .filter(|&s| !s.is_empty() && s.chars().all(|c| c.is_digit(10))) // Filter to get only numeric parts
                                    .map(|s| s.parse::<usize>().unwrap())
                                    .collect::<Vec<_>>();

                                if indices.len() == 2 {
                                    let row = indices[0];
                                    let col = indices[1];

                                    match matrix_name {
                                        "a" => {
                                            let val = var.a.read().unwrap()[row][col].inner().unwrap().clone();
                                            let new_val = val ^ (mask as i32);
                                            var.a.write().unwrap()[row][col]["cp1"] = new_val;
                                        }
                                        "b" => {
                                            let val = var.b.read().unwrap()[row][col].inner().unwrap().clone();
                                            let new_val = val ^ (mask as i32);
                                            var.b.write().unwrap()[row][col]["cp1"] = new_val;
                                        }
                                        "result" => {
                                            let val = var.result.read().unwrap()[row][col].inner().unwrap().clone();
                                            let new_val = val ^ (mask as i32);
                                            var.result.write().unwrap()[row][col]["cp1"] = new_val;
                                        }
                                        _ => {
                                            println!("non e' una matrice che conosco")
                                        }
                                    }
                                } else {
                                    println!("numero di indici diverso da 2");
                                }
                            }
                        */
                        }
                    }

                }
            }
        }
        tx_injector.send("ricevuto").unwrap();
    }
}



pub fn injector_manager(rx_chan_fm_inj: Receiver<FaultListEntry>,
                        tx_chan_inj_anl: Sender<TestResult>,
                        target: String,
                        data: Data<i32>){

    panic::set_hook(Box::new(|_panic_info| {        // SE NECESSARIO RIMUOVERE
        // Print a simple message when a panic occurs
        if VERBOSE{
           eprintln!("A panic occurred!");
        }
    }));



    let mut handles_runner = vec![];
    let mut handles_injector = vec![];

    while let Ok(fault_list_entry) = rx_chan_fm_inj.recv(){

        let var = AlgorithmVariables::from_target(target.as_str(), data.clone());

        // thread
        let (tx_1, rx_1) = channel();
        let (tx_2, rx_2) = channel();

        let shared_variables = var;

        let runner_variables = Arc::clone(&shared_variables);
        let injector_variables = Arc::clone(&shared_variables);

        let fault_list_entry_runner = fault_list_entry.clone();

        handles_runner.push(thread::spawn(move || runner(runner_variables, fault_list_entry_runner, tx_1, rx_2)));     // lancio il thread che esegue l'algoritmo
        handles_injector.push(thread::spawn(move || injector(injector_variables, fault_list_entry, tx_2, rx_1)));      // lancio il thread iniettore
    }


    for handle in handles_runner {
        let result = handle.join().unwrap();
        tx_chan_inj_anl.send(result).unwrap();
    }


    for handle in handles_injector {
        handle.join().unwrap();
    }


    drop(tx_chan_inj_anl);
}