mod tests;
use crate::hardened::*;
pub fn matrix_multiplication(a: &Vec<Vec<Hardened<i32>>>, b: &Vec<Vec<Hardened<i32>>>) -> Result<Vec<Vec<Hardened<i32>>>, IncoherenceError> {
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
}