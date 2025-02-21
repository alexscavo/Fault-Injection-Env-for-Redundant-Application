mod tests;
use crate::hardened::*;
pub fn bubble_sort(vet: &mut Vec<Hardened<i32>>) -> Result<(), IncoherenceError> {

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
}