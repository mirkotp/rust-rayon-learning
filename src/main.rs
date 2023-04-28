use rayon::prelude::{IntoParallelIterator, IndexedParallelIterator, ParallelIterator};

fn main() {
    println!("Also look at ./benches");
    test_zip();
}

fn test_zip() {
    let a = 1..1000;
    let b = 1001..2000;

    a.into_par_iter().zip(b.into_par_iter()).for_each(|(e1, e2)| {
        println!("{} - {}", e1, e2);
    });

    // Items are paired correctly, sample output
    // 250 - 1250
    // 251 - 1251
    // 875 - 1875
    // 876 - 1876
    // 750 - 1750
    // 751 - 1751
    // 752 - 1752
    // ...
}