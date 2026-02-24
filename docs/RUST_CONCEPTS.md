# Concepts clés de Rust : ownership, emprunts, lifetimes

Ce document explique ce qui rend Rust particulier, en lien avec ton projet `helloWorldAPI`.

---

## 1. Ownership (propriété)

**Règle : chaque valeur a un et un seul propriétaire.**

Quand tu fais `let store = Store::new();`, la variable `store` **possède** la valeur. Si tu passes `store` à une fonction sans `&`, tu **déplaces** la valeur : elle n’est plus utilisable dans l’appelant.

```rust
// ✅ COMPILE
fn take_ownership(s: String) {
    println!("{}", s);
} // s est détruit ici

fn main() {
    let x = String::from("hello");
    take_ownership(x);
    // println!("{}", x);  // ❌ ERREUR : x a été déplacé (moved)
}
```

Dans ton `main.rs` :

```rust
let store = Store::new();
let state = AppState::new(store);  // store est déplacé dans AppState
// store n'existe plus ici ; state.store contient l'ancien store
let app = router(state);           // state est déplacé dans le routeur
```

Si tu avais besoin de réutiliser `store` après `AppState::new(store)`, il faudrait soit **cloner** (`AppState::new(store.clone())`), soit ne pas donner la propriété (voir emprunts ci‑dessous).

---

## 2. Emprunts : référence immuable `&T` et mutable `&mut T`

Au lieu de déplacer une valeur, on peut l’**emprunter** :

- **`&T`** = référence **immuable** : lecture seule, pas de modification.
- **`&mut T`** = référence **mutable** : tu peux modifier la valeur.

### Règles des emprunts

| Règle | Explication |
|-------|-------------|
| **Plusieurs `&T` en même temps** | Autant de lecteurs que tu veux. |
| **Un seul `&mut T` à la fois** | Pas d’autre emprunt (ni `&` ni `&mut`) pendant que tu as un `&mut`. |
| **Pas de `&mut` si un `&` existe encore** | Les lecteurs ne doivent pas coexister avec un writer. |

```rust
// ✅ COMPILE : plusieurs références immuables
fn main() {
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} {}", r1, r2);
}

// ❌ NE COMPILE PAS : mélange & et &mut
fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &mut s;   // erreur : s est déjà emprunté immuablement par r1
    println!("{}", r1);
}
```

```rust
// ✅ COMPILE : les & sont "terminés" avant le &mut
fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    println!("{}", r1);  // dernière utilisation de r1
    let r2 = &mut s;     // ok, plus aucun & en vie
    r2.push_str("!");
}
```

Dans tes handlers, `State(state): State<AppState>` te donne une **référence partagée** vers l’état ; Axum peut donc appeler plusieurs handlers en parallèle tant qu’ils ne font que lire (ou utilisent un intérieur partagé comme `Arc`).

---

## 3. Lifetimes (durées de vie)

Une **lifetime** (`'a`, `'static`, etc.) indique **combien de temps une référence reste valide**. Le but : interdire les références vers de la mémoire libérée.

### Référence vers une donnée locale

```rust
// ❌ NE COMPILE PAS : retourner une référence vers une valeur locale
fn bad() -> &str {
    let s = String::from("hello");
    &s   // s est détruit à la fin de la fonction → référence invalide
}
// Le compilateur dit : "missing lifetime specifier" ou "borrowed value does not live long enough"
```

```rust
// ✅ COMPILE : retourner une String (propriété), pas une référence
fn good() -> String {
    let s = String::from("hello");
    s
}
```

### Référence vers des données qui vivent assez longtemps

```rust
// ✅ COMPILE : la référence retournée vit autant que l'entrée
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let a = String::from("abc");
    let b = "xyz";
    let r = longest(a.as_str(), b);
    println!("{}", r);
}
```

Ici `'a` signifie : “le résultat vit au plus aussi longtemps que le plus court de `x` et `y`”. Le compilateur vérifie que tu n’utilises jamais `r` après la mort de `a` ou `b`.

### Dans ton code

```rust
pub async fn hello() -> &'static str {
    "Hello World"
}
```

`'static` veut dire : “cette référence est valide pour toute la durée du programme”. Les littéraux de chaîne (`"Hello World"`) sont stockés dans le binaire, donc c’est correct.

---

## 4. Résumé visuel

```
OWNERSHIP
  let x = valeur;    →  x possède la valeur
  f(x);              →  x est déplacé dans f (x invalide après)

BORROWING
  let r = &x;        →  r emprunte en lecture (immuable)
  let m = &mut x;    →  m emprunte en écriture (mutable)
  Règles : plusieurs & ok ; un seul &mut à la fois ; pas de & quand &mut est actif

LIFETIMES
  fn f<'a>(x: &'a T) -> &'a T   →  le retour vit au plus comme x
  &'static                         →  vit tout le programme (littéraux)
```

---

## 5. Exemples à lancer

Depuis la racine du projet :

```bash
# Exemples qui compilent
cargo run --example ownership_borrow
cargo run --example lifetimes_ok

# Exemple qui ne compile pas (pour voir l'erreur "use of moved value")
cargo build --example does_not_compile
```

---

## 6. Snippets à copier-coller pour voir d’autres erreurs

### Erreur : emprunt mutable pendant un emprunt immuable

Copie ceci dans un `main.rs` ou un exemple et lance `cargo build` :

```rust
fn main() {
    let mut s = String::from("world");
    let r1 = &s;
    let r2 = &mut s;   // erreur ici
    println!("{}", r1);
}
```

Message typique : *cannot borrow `s` as mutable because it is also borrowed as immutable*.

### Erreur : référence invalide (dangling)

```rust
fn dangling() -> &str {
    let s = String::from("dangling");
    &s   // s est détruit à la fin de la fonction
}

fn main() {
    let r = dangling();
    println!("{}", r);
}
```

Message typique : *missing lifetime specifier* ou *returns a reference to data owned by the current function*.

---

## 7. Lien avec ton projet

| Dans ton code | Concept |
|---------------|---------|
| `let store = Store::new();` puis `AppState::new(store)` | **Ownership** : `store` est déplacé dans `AppState`. |
| `State(state): State<AppState>` dans les handlers | **Emprunt** : Axum passe une référence partagée à l’état. |
| `pub async fn hello() -> &'static str` | **Lifetime** : la chaîne retournée vit tout le programme. |
| `Arc<dyn ItemRepository>` dans `Store` | **Partage** : plusieurs handlers peuvent utiliser le même store sans violation des règles d’emprunt. |

