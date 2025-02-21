#[cfg(test)]
mod tests {
    use crate::hardened::Hardened;
    use crate::hardened::matrix_multiplication_hardened::matrix_multiplication;
    #[test]
    fn test_matrix_multiplication_hardened_simple_5x5() {
        // Matrice Hardened A (5x5)
        let a = Hardened::from_mat(vec![
            vec![1, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 0],
            vec![0, 0, 1, 0, 0],
            vec![0, 0, 0, 1, 0],
            vec![0, 0, 0, 0, 1],
        ]);

        // Matrice Hardened B (5x5)
        let b = Hardened::from_mat(vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15],
            vec![16, 17, 18, 19, 20],
            vec![21, 22, 23, 24, 25],
        ]);

        // Matrice Hardened attesa per il risultato
        // Risultato di moltiplicare una matrice identità per B dovrebbe essere B stessa
        let expected_output = Hardened::from_mat(vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15],
            vec![16, 17, 18, 19, 20],
            vec![21, 22, 23, 24, 25],
        ]);

        // Esecuzione della moltiplicazione e verifica del risultato
        match matrix_multiplication(&a, &b) {
            Ok(result) => assert_eq!(result, expected_output),
            Err(e) => panic!("Errore di incoerenza: {:?}", e),
        }
    }

    #[test]
    fn test_matrix_multiplication() {
        // Matrice A
        let a = vec![
            vec![Hardened::from(1), Hardened::from(2), Hardened::from(3)],
            vec![Hardened::from(4), Hardened::from(5), Hardened::from(6)],
            vec![Hardened::from(7), Hardened::from(8), Hardened::from(9)],
        ];

        // Matrice B
        let b = vec![
            vec![Hardened::from(9), Hardened::from(8), Hardened::from(7)],
            vec![Hardened::from(6), Hardened::from(5), Hardened::from(4)],
            vec![Hardened::from(3), Hardened::from(2), Hardened::from(1)],
        ];

        // Risultato atteso della moltiplicazione
        let expected = vec![
            vec![Hardened::from(30), Hardened::from(24), Hardened::from(18)],
            vec![Hardened::from(84), Hardened::from(69), Hardened::from(54)],
            vec![Hardened::from(138), Hardened::from(114), Hardened::from(90)],
        ];

        // Calcola il risultato
        let result = matrix_multiplication(&a, &b);

        // Assicurati che non ci siano errori
        assert!(result.is_ok());

        // Confronta il risultato con quello atteso
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_matrix_multiplication_hardened_simple_5x5_mul_fail() {
        // Matrice Hardened A (5x5)
        let a = Hardened::from_mat(vec![
            vec![1, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 0],
            vec![0, 0, 1, 0, 0],
            vec![0, 0, 0, 1, 0],
            vec![0, 0, 0, 0, 1],
        ]);

        // Matrice Hardened B (5x5)
        let mut b = Hardened::from_mat(vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15],
            vec![16, 17, 18, 19, 20],
            vec![21, 22, 23, 24, 25],
        ]);

        // Matrice Hardened attesa per il risultato
        // Risultato di moltiplicare una matrice identità per B dovrebbe essere B stessa
        let expected_output = Hardened::from_mat(vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15],
            vec![16, 17, 18, 19, 20],
            vec![21, 22, 23, 24, 25],
        ]);

        b[0][0]["cp2"] = 2;

        // Esecuzione della moltiplicazione e verifica del risultato
        match matrix_multiplication(&a, &b) {
            Ok(result) => assert_eq!(result, expected_output),
            Err(e) => panic!("Errore di incoerenza: {:?}", e),
        }
    }
}