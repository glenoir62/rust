pub fn lifetime_test() {
    /*
        'a = un nom de lifetime (durée de vie)
        &'a i32 = une référence vers un i32 qui vit pendant 'a
        Garantie : La référence retournée est valide aussi longtemps que les deux paramètres
        But : Éviter les références vers des données détruites (dangling pointers)

        Les lifetimes sont la garantie de sécurité mémoire de Rust sans garbage collector ! 🛡️
     */

    let number1 = 14;        // ┐ number1 vit ici
    let number2 = 15;        // │ number2 vit ici
    // │
    let resultat = get_bigger_number(&number1, &number2);
    //                                │          │
    //                                └──────────┘
    //                           resultat emprunte l'un des deux
    // │
    println!("lifetime {resultat}");  // │ resultat est valide ici
    // │
}                            // ┘ tout est détruit ici


fn get_bigger_number<'a>(num1: &'a i32, num2: &'a i32) -> &'a i32 {
//                   ^^^       ^^              ^^             ^^
//                    |         |               |              |
//                    |         +---------------+              |
//                    |              |                         |
//            nom du lifetime    les deux params              |
//                                ont le même lifetime         |
//                                                             |
//                              la valeur retournée a le même lifetime

    /*
    Signification : "La référence retournée vivra aussi longtemps que la plus courte des deux références en entrée"

    Au lieu de copier les nombres, on passe des références (pointeurs) pour :

        ✅ Éviter les copies inutiles
        ✅ Économiser la mémoire
        ✅ Montrer que la fonction ne prend pas possession des valeurs
     */
    if num1 > num2 {
        num1
    } else {
        num2
    }
}