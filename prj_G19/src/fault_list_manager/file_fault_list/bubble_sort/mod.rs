pub fn bubble_sort(mut vet: Vec<i32>) -> Vec<i32> {
    let n:usize = vet.len();
    let mut i = 0;

    while i < n {
        let mut swapped = false;
        let mut j = 0;

        while j < n - i - 1 {
            if vet[j] > vet[j + 1] {
                vet.swap(j, j + 1);
                swapped = true;
            }
            j += 1;
        }
        if !swapped {
            break;
        }
        i += 1;
    }
    vet
}
