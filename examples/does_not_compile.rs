//! Ce fichier NE COMPILE PAS volontairement.
//! Lance : cargo build --example does_not_compile
//! pour voir les erreurs du compilateur (ownership, borrow, lifetime).

fn main() {
    // ========== Erreur 1 : use after move ==========
    let x = String::from("hello");
    let y = x;   // x est déplacé dans y
    println!("{}", x); // ERREUR : use of moved value: `x`
}
