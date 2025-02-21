use std::sync::mpsc::{Receiver, Sender};
use crate::hardened::{Hardened, IncoherenceError};
use crate::injector::{BubbleSortVariables, MatrixMultiplicationVariables, SelectionSortVariables};

pub fn runner_selection_sort(variables: &SelectionSortVariables, tx_runner: Sender<&str>, rx_runner: Receiver<&str>) -> Result<Vec<Hardened<i32>>, IncoherenceError> {

    *variables.n.write().unwrap() = variables.vec.read().unwrap().len().into();
    tx_runner.send("i1").unwrap();
    rx_runner.recv().unwrap();

    *variables.j.write().unwrap() = Hardened::from(0);
    tx_runner.send("i2").unwrap();
    rx_runner.recv().unwrap();

    *variables.min.write().unwrap() = Hardened::from(10);
    tx_runner.send("i3").unwrap();
    rx_runner.recv().unwrap();

    *variables.i.write().unwrap() = Hardened::from(0);
    tx_runner.send("i4").unwrap();
    rx_runner.recv().unwrap();

    while *variables.i.read().unwrap() < (*variables.n.read().unwrap() - 1)? {
        tx_runner.send("i5").unwrap();
        rx_runner.recv().unwrap();

        variables.min.write().unwrap().assign(*variables.i.read().unwrap())?;
        tx_runner.send("i6").unwrap();
        rx_runner.recv().unwrap();

        variables.j.write().unwrap().assign((*variables.i.read().unwrap() + 1)?)?;
        tx_runner.send("i7").unwrap();
        rx_runner.recv().unwrap();

        while *variables.j.read().unwrap() < *variables.n.read().unwrap() {
            tx_runner.send("i8").unwrap();
            rx_runner.recv().unwrap();

            if variables.vec.read().unwrap()[*variables.j.read().unwrap()] < variables.vec.read().unwrap()[*variables.min.read().unwrap()] {
                tx_runner.send("i9").unwrap();
                rx_runner.recv().unwrap();

                variables.min.write().unwrap().assign(*variables.j.read().unwrap())?;
                tx_runner.send("i10").unwrap();
                rx_runner.recv().unwrap();
            }

            let tmp = (*variables.j.read().unwrap() + 1)?;  // necessario dato che non potrei fare j = j + 1, dato che dovrei acquisire un lock in lettura dopo averlo gia' acquisito sulla stessa variabile in scrittura
            variables.j.write().unwrap().assign(tmp)?;
            tx_runner.send("i11").unwrap();
            rx_runner.recv().unwrap();
        }

        variables.vec.write().unwrap().swap(variables.i.read().unwrap().inner()?, variables.min.read().unwrap().inner()?);
        tx_runner.send("i12").unwrap();
        rx_runner.recv().unwrap();

        let tmp = (*variables.i.read().unwrap() + 1)?;
        variables.i.write().unwrap().assign(tmp)?;
        tx_runner.send("i13").unwrap();
        rx_runner.recv().unwrap();
    }


    /*
    let mut n:Hardened<usize> = vet.len().into_data();
    let mut j= Hardened::from(0);
    let mut min = Hardened::from(0);
    //--------------SELECTION SORT-------------------------
    let mut i= Hardened::from(0);
    while i<(n-1)?{
        min.assign(i)?;                 //min=i
        j.assign((i+1)?)?;        //j=0
        //Ricerca del minimo
        while j<n{
            if vet[j]<vet[min]  {   min.assign(j)?; }
            j.assign((j+1)?)?;
        }
        //Scambio il minimo
        vet.swap(i.inner()?, min.inner()?);
        //Vado avanti
        i.assign((i+1)?)?;
    }
     */
    //------------------------------------------------------

    Ok(variables.vec.read().unwrap().clone())
}


pub fn runner_bubble_sort(variables: &BubbleSortVariables, tx_runner: Sender<&str>, rx_runner: Receiver<&str>) -> Result<Vec<Hardened<i32>>, IncoherenceError> {
    *variables.n.write().unwrap() = Hardened::from(variables.vet.read().unwrap().len());
    tx_runner.send("i1").unwrap();
    rx_runner.recv().unwrap();

    *variables.i.write().unwrap() = Hardened::from(0);
    tx_runner.send("i2").unwrap();
    rx_runner.recv().unwrap();

    while *variables.i.read().unwrap() < *variables.n.read().unwrap() {
        tx_runner.send("i3").unwrap();
        rx_runner.recv().unwrap();

        *variables.swapped.write().unwrap() = Hardened::from(false);
        tx_runner.send("i4").unwrap();
        rx_runner.recv().unwrap();

        *variables.j.write().unwrap() = Hardened::from(0);
        tx_runner.send("i5").unwrap();
        rx_runner.recv().unwrap();

        while *variables.j.read().unwrap() < ((*variables.n.read().unwrap() - *variables.i.read().unwrap())? - 1)? {
            tx_runner.send("i6").unwrap();
            rx_runner.recv().unwrap();

            if variables.vet.read().unwrap()[*variables.j.read().unwrap()].inner()? > variables.vet.read().unwrap()[(*variables.j.read().unwrap() + 1)?].inner()? {
                tx_runner.send("i7").unwrap();
                rx_runner.recv().unwrap();

                variables.vet.write().unwrap().swap(variables.j.read().unwrap().inner()?, (*variables.j.read().unwrap() + 1)?.inner()?);
                tx_runner.send("i8").unwrap();
                rx_runner.recv().unwrap();

                *variables.swapped.write().unwrap() = Hardened::from(true);
                tx_runner.send("i9").unwrap();
                rx_runner.recv().unwrap();

            }
            let tmp = (*variables.j.read().unwrap() + 1)?;
            variables.j.write().unwrap().assign(tmp)?;
            tx_runner.send("i10").unwrap();
            rx_runner.recv().unwrap();

        }

        if !variables.swapped.read().unwrap().inner()? {
            tx_runner.send("i11").unwrap();
            rx_runner.recv().unwrap();
            break;
        }

        let tmp = (*variables.i.read().unwrap() + 1)?;
        variables.i.write().unwrap().assign(tmp)?;
        tx_runner.send("i12").unwrap();
        rx_runner.recv().unwrap();
    }

    Ok(variables.vet.read().unwrap().clone())

    /*
    let n = Hardened::from(vet.len());
    let mut i = Hardened::from(0);

    while i < n {
        let mut swapped = Hardened::from(false);
        let mut j = Hardened::from(0);

        while j < ((n - i)? - 1)? {
            if vet[j].inner()? > vet[(j + 1)?].inner()? {
                vet.swap(j.inner()?, (j + 1)?.inner()?);
                swapped = Hardened::from(true);
            }
            j.assign((j + 1)?)?;
        }
        if !swapped.inner()? {
            break;
        }
        i.assign((i + 1)?)?;
    }
    Ok(())
    */
}



pub fn runner_matrix_multiplication(variables: &MatrixMultiplicationVariables, tx_runner: Sender<&str>, rx_runner: Receiver<&str>) -> Result<Vec<Hardened<i32>>, IncoherenceError> {
    *variables.size.write().unwrap() = Hardened::from(variables.a.read().unwrap().len());
    tx_runner.send("i1").unwrap();
    rx_runner.recv().unwrap();

    *variables.result.write().unwrap() =  vec![vec![Hardened::from(0); variables.size.read().unwrap().inner()?]; variables.size.read().unwrap().inner()?];
    tx_runner.send("i2").unwrap();
    rx_runner.recv().unwrap();

    *variables.i.write().unwrap() = Hardened::from(0);
    tx_runner.send("i3").unwrap();
    rx_runner.recv().unwrap();

    *variables.j.write().unwrap() = Hardened::from(0);
    tx_runner.send("i4").unwrap();
    rx_runner.recv().unwrap();

    *variables.k.write().unwrap() = Hardened::from(0);
    tx_runner.send("i5").unwrap();
    rx_runner.recv().unwrap();

    while *variables.i.read().unwrap() < *variables.size.read().unwrap() {
        tx_runner.send("i6").unwrap();
        rx_runner.recv().unwrap();
        /*
        *variables.row.write().unwrap() = Hardened::from_vec(Vec::new());
        tx_runner.send("i7").unwrap();
        rx_runner.recv().unwrap();
         */

        variables.j.write().unwrap().assign(Hardened::from(0))?;
        tx_runner.send("i7").unwrap();
        rx_runner.recv().unwrap();

        while *variables.j.read().unwrap() < *variables.size.read().unwrap() {
            tx_runner.send("i8").unwrap();
            rx_runner.recv().unwrap();

            *variables.acc.write().unwrap() = Hardened::from(0);
            tx_runner.send("i9").unwrap();
            rx_runner.recv().unwrap();

            variables.k.write().unwrap().assign(Hardened::from(0))?;
            tx_runner.send("i10").unwrap();
            rx_runner.recv().unwrap();

            while *variables.k.read().unwrap() < *variables.size.read().unwrap() {
                tx_runner.send("i11").unwrap();
                rx_runner.recv().unwrap();

                let tmp = (*variables.acc.read().unwrap() + (
                    variables.a.read().unwrap()[variables.i.read().unwrap().inner()?][variables.k.read().unwrap().inner()?] *
                        variables.b.read().unwrap()[variables.k.read().unwrap().inner()?][variables.j.read().unwrap().inner()?]
                )?)?;
                variables.acc.write().unwrap().assign(tmp)?;
                tx_runner.send("i12").unwrap();
                rx_runner.recv().unwrap();

                let tmp = (*variables.k.read().unwrap() + 1)?;
                variables.k.write().unwrap().assign(tmp)?;
                tx_runner.send("i13").unwrap();
                rx_runner.recv().unwrap();
            }
            /*
            variables.row.write().unwrap().push(*variables.acc.read().unwrap());
            tx_runner.send("i14").unwrap();
            rx_runner.recv().unwrap();

             */

            variables.result.write().unwrap()[variables.i.read().unwrap().inner()?][variables.j.read().unwrap().inner()?].assign(variables.acc.read().unwrap().clone())?;
            tx_runner.send("i14").unwrap();
            rx_runner.recv().unwrap();

            let tmp = (*variables.j.read().unwrap() + 1)?;
            variables.j.write().unwrap().assign(tmp)?;
            tx_runner.send("i15").unwrap();
            rx_runner.recv().unwrap();
        }

        let tmp = (*variables.i.read().unwrap() + 1)?;
        variables.i.write().unwrap().assign(tmp)?;
        tx_runner.send("i16").unwrap();
        rx_runner.recv().unwrap();
    }

    Ok(variables.result.read().unwrap().clone().into_iter().clone().flatten().collect::<Vec<Hardened<i32>>>())
}

#[cfg(test)]
    mod tests{
    use std::thread;
    use std::sync::Arc;
    use std::sync::mpsc::channel;
    use crate::fault_env::Data::Matrices;
    use crate::fault_list_manager::fault_manager;
    use crate::injector::{injector, runner, AlgorithmVariables};

    #[test]
        fn test_run_matrix_multiplication(){
            let fault_list = "src/fault_list_manager/file_fault_list/matrix_multiplication/matrix_mul_FL.json".to_string();
            let target = "matrix_multiplication".to_string();
            let data = Matrices(vec![vec![5, 7, 6, 5], vec![7, 10, 8, 7], vec![6, 8, 10, 9], vec![5, 7, 9, 10]], vec![vec![68, -41, -17, 10], vec![-41, 25, 10, -6], vec![-17, 10, 5, -3], vec![10, -6, -3, 2]]);
            let (tx_chan_fm_inj, rx_chan_fm_inj) = channel();
            let (tx_chan_inj_anl, rx_chan_inj_anl) = channel();
            fault_manager(tx_chan_fm_inj,fault_list);



        //INJECTOR MANAGER
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
            handles_injector.push(thread::spawn(move || injector(injector_variables, fault_list_entry, tx_2, rx_1)));
            break;
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
    }

/*
let size = Hardened::from(a.len());
    let mut result = vec![vec![Hardened::from(0); size.inner()?]; size.inner()?];

    let mut i = Hardened::from(0);
    let mut j = Hardened::from(0);
    let mut k = Hardened::from(0);

    while i < size {
        j.assign(Hardened::from(0))?;

        while j < size {
            let mut acc = Hardened::from(0);
            k.assign(Hardened::from(0))?;

            while k < size {
                acc.assign((acc + (a[i.inner()?][k.inner()?]*b[k.inner()?][j.inner()?])? )? )?;
                k.assign((k + 1)?)?;
            }
            result[i.inner()?][j.inner()?].assign(acc)?;
            j.assign((j + 1)?)?;
        }
        i.assign((i + 1)?)?;
    }
    Ok(result)
 */



