pub fn afficher_contenu<T>(vecteur: Vec<T>)
where
    T: std::fmt::Debug,
{
    for element in &vecteur {
        println!("{:?}", element);
    }
}
pub fn plus_grand<T: std::cmp::PartialOrd>(x: T, y: T) -> T {
    if x > y {
        x
    } else {
        y
    }
}