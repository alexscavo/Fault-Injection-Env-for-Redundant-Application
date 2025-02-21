#[cfg(test)]
mod tests {
    use crate::hardened::{Hardened, IncoherenceError};
    use crate::hardened::selection_sort_hardened::selection_sort;
    #[test]
    //Provo a usare il nuovo tipo per ordinare un vettore
    fn test_sort() {
        let mut myvec = Hardened::from_vec(vec![31, 10, 15, 6, 4, 3]);
        assert!(selection_sort(&mut myvec).is_ok());
        let mut myvec2 = Hardened::from_vec(vec![3, 4, 6, 10, 15, 31]);
        assert_eq!(myvec, myvec2);
    }
    #[test]
    fn test_sort_hardened() {
        // Creazione di un vettore Hardened non ordinato
        let mut myvec = Hardened::from_vec(vec![31, 10, 15, 6, 4, 3]);

        // Ordinamento del vettore e controllo che l'operazione vada a buon fine
        assert!(selection_sort(&mut myvec).is_ok());

        // Vettore Hardened atteso dopo l'ordinamento
        let myvec_sorted = Hardened::from_vec(vec![3, 4, 6, 10, 15, 31]);

        // Confronto tra il vettore ordinato e il risultato atteso
        assert_eq!(myvec, myvec_sorted);
    }
}