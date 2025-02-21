pub(crate) mod bubble_sort_hardened;
pub(crate) mod matrix_multiplication_hardened;
pub(crate) mod selection_sort_hardened;

use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, Index, IndexMut, Sub, Mul};
use thiserror::Error;
use crate::fault_env::Data;

//-------------------------------------------------------------
#[derive(Clone, Copy)]
/// <h2>Tipo ```Hardened<T>``` </h2> <br>
/// <p>Questo nuovo tipo 'Hardened' ha al suo interno DUE COPIE
/// del valore della variabile di tipo T.
/// Questo per asserire alla realizzazione della 'Regola 1':
/// ogni variabile x deve essere duplicata facendone due copie x1 e x2. </p>
pub struct Hardened<T>{
    cp1: T,
    cp2: T,
}

impl<T> Hardened<T>
where T: Debug+PartialEq+Eq+Copy+Clone{
    ///Controllo di coerenza: si controlla che le due copie del valore della
    /// variabile siano uguali. E' la funzione utilizzata affinché venga rispettata
    /// la 'Regola 3' secondo cui ogni lettura deve essere preceduta dal controllo delle
    /// due copie, nel caso in cui questo fallisse, è stato trovato un fault!
    fn incoherent(&self)->bool{
        self.cp1 != self.cp2
    }

    /// L'operazione di assegnazione non può essere ridefinita (cioè non posso ridefinire '='
    /// in a=b) perché dovrei modificare la semantica del movimento caratteristica di Rust.
    /// L'operazione del tipo a=b, con a, b di tipo ```Hardened<T>``` deve essere fatta nel seguente
    /// modo: ```a.assign(b)```
    pub fn assign(&mut self, other: Hardened<T>)->Result<(), IncoherenceError>{
        if other.incoherent(){
            return Err(IncoherenceError::AssignFail)
        }
        //Regola 2: Ogni scrittura deve essere eseguita su entrambe le copie
        self.cp1 = other.cp1;
        self.cp2 = other.cp2;
        Ok(())
    }

    ///Crea un vettore ```Vec<Hardened<T>>``` da un Vec<T>
    pub fn from_vec(vet: Vec<T>)->Vec<Hardened<T>>{
        vet.iter().map(|&x| Hardened::from(x)).collect()
    }

    //Uso questa funzione in ottica di irrobustire un'intera matrice...
    pub fn from_mat(mat: Vec<Vec<T>>) -> Vec<Vec<Hardened<T>>> {
        mat.into_iter().map(|row| row.into_iter().map(|x| Hardened::from(x)).collect()).collect()
    }

    ///Estrae (dopo aver controllato la coerenza del dato) il dato
    /// di tipo T incapsulato al suo interno.
    pub fn inner(&self)->Result<T, IncoherenceError>{
        if self.incoherent(){
            return Err(IncoherenceError::InnerFail)
        }
        Ok(self.cp1)
    }
}

///Crea una variabile di tipo ```Hardened<T>``` da una di tipo T,
/// si assume che tale variabile sia copiabile.
impl<T> From<T> for Hardened<T> where T:Copy{
    fn from(value: T) -> Self {
        Self{cp1: value, cp2: value}
    }
}

pub trait IntoNestedVec {
    fn into_nested_vec(self) -> Vec<i32>;
}
impl IntoNestedVec for Vec<Hardened<i32>> {
    fn into_nested_vec(self) -> Vec<i32> {
        self.into_iter()
            .map(|hardened| hardened.cp1) // Estrai solo il campo cp1
            .collect()
    }
}

//---------------------OPERAZIONI ARITMETICHE-------------------------
//Tutte queste operazioni in caso di fallimento ritornano un Errore
//di tipo IncoherenceError, implementato usando il crate thiserror.
// a = b+c
impl<T> Add for Hardened<T>
where T: Add<Output=T>+PartialEq+Eq+Debug+Copy+Clone{
    type Output = Result<Hardened<T>, IncoherenceError>;
    fn add(self, rhs: Self) -> Self::Output {
        if self.incoherent() || rhs.incoherent(){
            return Err(IncoherenceError::AddFail)
        }
        Ok(Self{
            cp1: self.cp1 + rhs.cp1,
            cp2: self.cp2 + rhs.cp2,
        })
    }
}

impl Add<usize> for Hardened<usize>{
    type Output = Result<Hardened<usize>, IncoherenceError>;
    fn add(self, rhs: usize) -> Self::Output {
        if self.incoherent() {
            return Err(IncoherenceError::AddFail);
        }
        Ok(Self{
            cp1: self.cp1 + rhs,
            cp2: self.cp2 + rhs,
        })
    }
}

impl<T> Sub for Hardened<T>
where T:Sub<Output=T>+PartialEq+Eq+Debug+Copy+Clone{
    type Output=Result<Hardened<T>,IncoherenceError>;
    fn sub(self, rhs: Self) -> Self::Output {
        if self.incoherent() || rhs.incoherent(){
            return Err(IncoherenceError::SubFail)
        }
        Ok(Self{
            cp1: self.cp1 - rhs.cp1,
            cp2: self.cp2 - rhs.cp2,
        })
    }
}

impl Sub<usize> for Hardened<usize>{
    type Output = Result<Hardened<usize>, IncoherenceError>;
    fn sub(self, rhs: usize) -> Self::Output {
        if self.incoherent(){
            return Err(IncoherenceError::SubFail)
        }
        return Ok(Self{
            cp1: self.cp1 - rhs,
            cp2: self.cp2 - rhs,
        })
    }
}

// Mul per Hardened<T> per supportare la moltiplicazione elementare
impl<T> Mul for Hardened<T>
where T: Mul<Output = T> + PartialEq + Eq + Debug + Copy + Clone {
    type Output = Result<Hardened<T>, IncoherenceError>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.incoherent() || rhs.incoherent() {
            return Err(IncoherenceError::MulFail);
        }

        Ok(Self {
            cp1: self.cp1 * rhs.cp1,
            cp2: self.cp2 * rhs.cp2,
        })
    }
}

//------------------------------------------------------------------------

//------------------------OPERAZIONI DI CONFRONTO-------------------------
impl<T> PartialEq for Hardened<T>
where T:PartialEq+Eq+Debug+Copy+Clone{
    fn eq(&self, other: &Self) -> bool {
        if  other.incoherent(){
            panic!("PartialEq::eq")
        }
        self.cp1.eq(&other.cp1)
    }
}

impl<T> Eq for Hardened<T>
where T:PartialEq+Eq+Debug+Copy+Clone{      }

impl<T> PartialOrd for Hardened<T>
where T:PartialEq+PartialOrd+Eq+Debug+Copy+Clone{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if other.incoherent(){
            panic!("PartialOrd::partial_cmp")
        }
        self.cp1.partial_cmp(&other.cp1)
    }
}

impl<T> Ord for Hardened<T>
where T:PartialEq+PartialOrd+Ord+Eq+Debug+Copy+Clone{
    fn cmp(&self, other: &Self) -> Ordering {
        if other.incoherent(){
            panic!("Ord::cmp");
        }
        self.cp1.cmp(&other.cp1)
    }
}

//Funzioni per indicizzare un Vec usando un Hardened<usize>
impl<T> Index<Hardened<usize>> for Vec<Hardened<T>>{
    type Output=Hardened<T>;
    ///Estrae un riferimento immutabile
    fn index(&self, index: Hardened<usize>) -> &Self::Output {
        if index.incoherent(){
            panic!("Index<Hardened<usize>>::index");
        }
        self.index(index.cp1)
    }
}

impl<T> IndexMut<Hardened<usize>> for Vec<Hardened<T>>{
    fn index_mut(&mut self, index: Hardened<usize>) -> &mut Self::Output {
        if index.incoherent(){
            panic!("IndexMut<Hardened<usize>>::index_mut");
        }
        self.index_mut(index.cp1)
    }
}
//Per iniettare nelle variabili si potrebbero utilizzare la notazione Var["cp1"], Var["cp2"]
impl<T> Index<&str> for Hardened<T>{
    type Output=T;
    fn index(&self, index: &str) -> &Self::Output {
        match index{
            "cp1" => {  &self.cp1 },
            "cp2" => {   &self.cp2 },
            _ => panic!("Index")
        }
    }
}

impl<T> IndexMut<&str> for Hardened<T>{
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        match index{
            "cp1" => {  &mut self.cp1 },
            "cp2" => {   &mut self.cp2 },
            _ => panic!("IndexMut")
        }
    }
}

//Per poter stampare il tipo Hardened<T> con la macro println!() e il
// modificatore {:?}
impl<T> Debug for Hardened<T> where T:Debug+PartialEq+Eq+Copy+Clone{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.incoherent(){
            panic!("Debug::fmt");
        }
        self.cp1.fmt(f)
    }
}



//-------------------------------------------------------------
///Tipo di errore generato tutte le volte che fallisce il controllo
/// di coerenza delle due copie all'interno di una variabile di tipo
/// ```Hardened<T>```.
#[derive(Error, Debug, Clone)]
pub enum IncoherenceError{
    #[error("IncoherenceError::AssignFail: assignment failed")]
    AssignFail,
    #[error("IncoherenceError::AddFail: due to incoherence add failed")]
    AddFail,
    #[error("IncoherenceError::SubFail: due to incoherence add failed")]
    SubFail,
    #[error("IncoherenceError::MulFail: due to incoherence mul failed")]
    MulFail,
    #[error("IncoherenceError::IndexMutFail ")]
    IndexMutFail,
    #[error("IncoherenceError::IndexFail ")]
    IndexFail,
    #[error("IncoherenceError::OrdFail ")]
    OrdFail,
    #[error("IncoherenceError::PartialOrdFail ")]
    PartialOrdFail,
    #[error("IncoherenceError::PartialEqFail ")]
    PartialEqFail,
    #[error("IncoherenceError::InnerFail")]
    InnerFail,
}

//Funzioni per il conteggio 'passivo' delle istruzioni eseguite

pub fn run_for_count_selection_sort(vettore: Data<i32>) ->usize{

    let mut vet = vettore.into_vector();
    let n:usize = vet.len();
    let mut j;
    let mut min;

    let mut count=5;
    //-----------------------SELECTION SORT-------------------------
    let mut i=0;
    while i< n -1{
        count=count+1;
        min=i;
        count=count+1;
        j=i+1;
        //Ricerca del minimo
        count=count+1;
        while j< n {
            count=count+1;
            if vet[j] < vet[min]{
                count=count+1;
                min=j;  }
            count=count+1;
            j = j+1;
        }
        count=count+1;
        //Scambio il minimo
        vet.swap(min,i);
        //Vado avanti
        count=count+1;
        i=i+1;

        //conto il while di dopo (se necessario)
        if i< n -1 {count=count+1}
    }
    count
}
pub fn run_for_count_bubble_sort(vettore: Data<i32>) ->usize{
    let mut count=2;
    let mut vet=vettore.into_vector();
    let n = vet.len();
    let mut i = 0;

    while i < n {
        count+=1;
        let mut swapped = false;
        count+=1;
        let mut j = 0;
        count+=1;

        while j < n - i - 1 {
            count+=1;
            if vet[j] > vet[j + 1] {
                count+=1;
                vet.swap(j, j + 1);
                count+=1;
                swapped = true;
                count+=1;
            }
            j += 1;
            count+=1;
        }
        if !swapped {
            count+=1;
            break;
        }
        i += 1;
        count+=1;
    }
    count
}
//a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>
pub fn run_for_count_matrix_mul(matrici: Data<i32>, size:usize)->usize{

    let matrices = matrici.into_matrices();
    let a = matrices.0;
    let b = matrices.1;

    let mut result: Vec<Vec<i32>> = Vec::new();
    let mut count=3;
    for i in 0..size {
        count+=1;
        let mut row: Vec<i32> = Vec::new(); // Crea una nuova riga
        count+=1;
        for j in 0..size {
            count+=1;
            let mut acc = 0;
            count+=1;
            for k in 0..size {
                count+=1;
                acc += a[i][k] * b[k][j];
                count+=1;
            }
            row.push(acc); // Aggiunge il valore calcolato alla riga
            count+=1;
        }
        result.push(row); // Aggiunge la riga alla matrice risultante
        count+=1;
    }
    //result
    count
}




//------------------------------------------------------

#[cfg(test)]
mod tests{
    use std::panic::catch_unwind;
    use crate::Hardened;

    use crate::IncoherenceError;
    #[test]
    fn test_add_ok(){
        //Arrange
        let a = Hardened::from(3);
        let b = Hardened::from(2);
        //Act
        let c = (a+b);
        //Assert
        assert_eq!(c.is_ok(), true);
        assert_eq!(c.unwrap().inner().unwrap(), 5);
    }
    #[test]
    fn test_add_err(){
        let mut a = Hardened::from(3);
        let  b = Hardened::from(2);
        a.cp1 = a.cp1 & 0;      //Injection

        let c=a+b;
        assert!(c.is_err());
    }
    #[test]
    fn test_ord(){
        let a = Hardened::from(5);
        let b = Hardened::from(4);
        assert!(a>b);
    }
    #[test]
    fn test_add_with_usize(){
        let a = Hardened::from(4);
        let ris = a+5;
        assert!(ris.is_ok());
        assert_eq!(ris.unwrap().inner().unwrap(), 9);
    }

    #[test]
    //Test per verificare il corretto funzionamento di from_mat
    fn test_from_mat(){
        let input_matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9]
        ];

        let expected_output = vec![
            vec![Hardened::from(1), Hardened::from(2), Hardened::from(3)],
            vec![Hardened::from(4), Hardened::from(5), Hardened::from(6)],
            vec![Hardened::from(7), Hardened::from(8), Hardened::from(9)]
        ];

        // Call the function
        let output_matrix = Hardened::from_mat(input_matrix);

        // Assert the output is as expected
        assert_eq!(output_matrix, expected_output);
    }
    #[test]
    fn test_matrix(){
        let mat = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9]
        ];

        assert_eq!(4, mat[1][0]);
    }
    //Test su Index/IndexMut<&str> for Hardened<T>
    #[test]
    fn test_indexMut_Hardened_for_injection(){
        let mut myhd =Hardened::from(3);
        myhd["cp1"] = 2;
        assert_eq!(myhd.incoherent(), true);
    }
    #[test]
    fn test_index_Hardened_for_injection(){
        let mut myhd=Hardened::from(2);
        assert_eq!(myhd["cp1"],2);
    }
    #[test]
    #[should_panic]
    fn test_index_panic(){
        let mut myvar=Hardened::from(2);
        _=myvar["cp3"];
    }
    #[test]
    #[should_panic]
    fn test_indexMut_panic(){
        let mut myvar=Hardened::from(2);
        myvar["cpe2ejnkjndf"] = 2;
    }
    #[test]
    fn test_from_mat_hardened() {
        // Matrice di input con valori interi
        let input_matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9]
        ];

        // Matrice attesa con elementi di tipo Hardened
        let expected_output = vec![
            vec![Hardened::from(1), Hardened::from(2), Hardened::from(3)],
            vec![Hardened::from(4), Hardened::from(5), Hardened::from(6)],
            vec![Hardened::from(7), Hardened::from(8), Hardened::from(9)]
        ];

        // Chiamata della funzione e confronto con l'output atteso
        let output_matrix = Hardened::from_mat(input_matrix);
        assert_eq!(output_matrix, expected_output);
    }
        #[test]
        fn test_get_message(){
            let payload = catch_unwind(|| {
                panic!("funge"); }).unwrap_err();

            let msg = panic_message::panic_message(&payload);
            assert_eq!("funge", msg);
        }

    }
//}
