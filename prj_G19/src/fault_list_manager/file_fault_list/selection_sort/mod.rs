pub fn selection_sort(mut vet:Vec<i32>)->Vec<i32>{
    let n:usize = vet.len();
    #[allow(unused_assignments)]
    let mut j=0;
    #[allow(unused_assignments)]
    let mut min=0;

    //-----------------------SELECTION SORT-------------------------
    let mut i=0;
    while i< n -1{
        min=i;
        j=i+1;
        //Ricerca del minimo
        while j< n {
            if vet[j] < vet[min]{ min=j; }
            j = j+1;
        }
        //Scambio il minimo
        vet.swap(min,i);
        //Vado avanti
        i=i+1;
    }
    vet
}