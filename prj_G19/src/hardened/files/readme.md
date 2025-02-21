# File `dizionario.txt`
In questo file trovi i messaggi generati dal panic, in tutte le situazioni 
in cui non è possibile restituire un `Result<T,E>`. Il contenuto del file è:

```text
5
PartialEq::eq
PartialOrd::partial_cmp
Ord::cmp
Index<Hardened<usize>>::index
IndexMut<Hardened<usize>>::index_mut
```

Il primo carattere è l'intero n, seguono n stringhe associate ai casi 
possibili messaggi di errore generate dai panic.