fn main() {
    println!("Hello, world,test!");
    let x:i32 = 33;
    x.afficher();
    println!("C'est un test : {}", x);

}

// Définition du trait Affichable
trait Affichable {
    fn afficher(&self);
}

// Implémentation du trait Affichable pour le type i32
impl Affichable for i32 {
    fn afficher(&self) {
        println!("C'est un nombre : {}", self);
    }
}