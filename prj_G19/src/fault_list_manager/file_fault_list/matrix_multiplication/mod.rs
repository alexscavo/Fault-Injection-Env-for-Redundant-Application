pub fn matrix_multiplication(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let size: usize = a.len();
    let mut result: Vec<Vec<i32>> = vec![vec![0; size]; size]; // Inizializza la matrice risultante con 0

    #[allow(unused_assignments)]
    let mut i = 0;
    #[allow(unused_assignments)]
    let mut j = 0;
    #[allow(unused_assignments)]
    let mut k = 0;

    while i < size {
        j = 0;

        while j < size {
            let mut acc = 0;
            k = 0;

            while k < size {
                acc += a[i][k] * b[k][j];
                k += 1;
            }
            result[i][j] = acc; // Assegna direttamente il valore calcolato
            j += 1;
        }
        i += 1;
    }
    result
}