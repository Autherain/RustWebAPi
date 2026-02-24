//! Exemples lifetimes — ce fichier COMPILE.
//! Lance avec : cargo run --example lifetimes_ok

/// La référence retournée vit au plus aussi longtemps que les deux entrées ('a).
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/// Littéral de chaîne : durée de vie 'static (valide tout le programme).
fn static_hello() -> &'static str {
    "Hello World"
}

fn main() {
    let a = String::from("abc");
    let b = String::from("xy");
    let r = longest(a.as_str(), b.as_str());
    println!("longest = {}", r);

    println!("{}", static_hello());

    // Exemple avec des durées différentes
    let s1 = String::from("court");
    let result;
    {
        let s2 = String::from("plus long texte");
        result = longest(s1.as_str(), s2.as_str());
        println!("dans le bloc: {}", result);
    }
    // println!("{}", result);  // ❌ Décommenter = erreur : result pointe dans s2, détruit au }
    println!("s1 vit encore: {}", s1);
}
