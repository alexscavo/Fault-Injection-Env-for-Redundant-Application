#[cfg(test)]
 mod tests {
     use crate::hardened::{Hardened, IncoherenceError};
     use crate::hardened::bubble_sort_hardened::bubble_sort;
     #[test]
     fn test_bubble_sort_hardened() {
         let mut vec = vec![
             Hardened::from(31),
             Hardened::from(10),
             Hardened::from(15),
             Hardened::from(6),
             Hardened::from(4),
             Hardened::from(3),
         ];

         assert!(bubble_sort(&mut vec).is_ok());

         let sorted_vec = vec![
             Hardened::from(3),
             Hardened::from(4),
             Hardened::from(6),
             Hardened::from(10),
             Hardened::from(15),
             Hardened::from(31),
         ];

         assert_eq!(vec, sorted_vec);

     }
 }


