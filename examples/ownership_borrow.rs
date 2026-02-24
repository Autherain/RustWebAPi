//! Exemples ownership et emprunts — ce fichier COMPILE.
//! Lance avec : cargo run --example ownership_borrow

fn take_ownership(s: String) {
    println!("take_ownership: {}", s);
} // s est détruit ici

#[allow(dead_code)]
fn borrow_immutable(s: &String) {
    println!("borrow_immutable: {}", s);
}

fn borrow_mutable(s: &mut String) {
    s.push_str(" (modifié)");
}

fn main() {
    // --- Ownership : un seul propriétaire ---
    let x = String::from("hello");
    take_ownership(x);
    // println!("{}", x);  // ❌ Décommenter = erreur : x a été déplacé (moved)

    // --- Emprunt immuable : plusieurs & en même temps OK ---
    let s1 = String::from("world");
    let r1 = &s1;
    let r2 = &s1;
    println!("r1 = {}, r2 = {}", r1, r2);

    // --- Emprunt mutable : un seul &mut à la fois ---
    let mut s2 = String::from("rust");
    borrow_mutable(&mut s2);
    println!("après borrow_mutable: {}", s2);

    // --- Ordre important : les & doivent être "terminés" avant un &mut ---
    let mut s3 = String::from("order");
    let r = &s3;
    println!("r = {}", r); // dernière utilisation de r
    let m = &mut s3;       // OK : plus aucun & en vie
    m.push_str(" matters");
    println!("s3 = {}", s3);
}
