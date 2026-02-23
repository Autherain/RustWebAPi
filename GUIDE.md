# Guide : API Axum + Tokio + utoipa (pour dev Go)

Ce guide montre comment initialiser un projet Rust et construire une petite API avec **Axum**, **Tokio** et **utoipa** (OpenAPI/Swagger). Le code du projet est déjà en place ; ici on explique les concepts et les parallèles avec Go.

---

## 1. Parallèles Go / Rust

| Go | Rust |
|----|------|
| `go mod init` | `cargo init` |
| `go.mod` / `go get` | `Cargo.toml` / section `[dependencies]` |
| `func main()` | `fn main()` (+ `#[tokio::main]` pour l’async) |
| `net/http` + router (Chi, Echo…) | Axum : `Router`, handlers `async fn` |
| Goroutines | Futures + runtime **Tokio** |
| `sync.Mutex` + `map[K]V` | `Arc<RwLock<HashMap<K,V>>>` (ou `Arc<Mutex<...>>`) |
| `encoding/json` | **serde** + **serde_json** |
| `http.ListenAndServe` | `TcpListener::bind` + `axum::serve` |

- **Module / Crate** : en Go tu fais `go mod init monprojet` ; en Rust, `cargo init` (ou `cargo new monprojet`) crée un “crate” avec un `Cargo.toml`.
- **Dépendances** : en Go tu ajoutes avec `go get` ; en Rust tu les déclares dans `[dependencies]` du `Cargo.toml`, puis `cargo build` les résout (pas d’équivalent direct à `go mod tidy`).
- **Point d’entrée** : `main.go` → `src/main.rs` avec `fn main()`.
- **Async** : en Go les goroutines sont implicites ; en Rust l’async est explicite et nécessite un runtime. **Tokio** est le runtime utilisé par Axum (équivalent conceptuel de l’orchestration des goroutines).

---

## 2. Initialisation du projet

Si tu repartais de zéro :

```bash
# À la racine du repo (ou depuis le répertoire parent)
cargo init
# ou, pour un nouveau dossier nommé
cargo new helloWorldAPI --name hello_world_api
```

Tu obtiens :

- **Cargo.toml** : nom du package, version, dépendances (équivalent `go.mod`).
- **src/main.rs** : point d’entrée avec `fn main()`.

Les dépendances sont résolues au premier `cargo build` (ou `cargo run`), pas besoin d’une commande dédiée comme `go mod tidy`.

---

## 3. Dépendances (Cargo.toml)

Dans ce projet, le [Cargo.toml](Cargo.toml) déclare :

- **axum** : framework HTTP (routing, extractors, réponses) — équivalent conceptuel de `net/http` + un router.
- **tokio** : runtime async avec `features = ["full"]` pour TCP, etc.
- **utoipa** : génération de la spec OpenAPI à partir du code (macros).
- **utoipa-swagger-ui** : sert l’UI Swagger et le JSON OpenAPI (feature `axum`).
- **serde** / **serde_json** : sérialisation JSON (équivalent `encoding/json`).
- **uuid** : génération d’IDs pour les items créés.

Exemple de bloc (déjà présent dans le projet) :

```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
utoipa = { version = "5", features = ["axum_extras"] }
utoipa-swagger-ui = { version = "8", features = ["axum"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
uuid = { version = "1", features = ["v4", "serde"] }
```

*(On utilise utoipa-swagger-ui 8 pour rester compatible avec Axum 0.7.)*

---

## 4. Hello World

- **Route** : `GET /` → renvoie le texte `"Hello World"`.
- **En Go** : un `http.HandlerFunc` qui fait `w.Write([]byte("Hello World"))` et qu’on enregistre sur `/`.
- **En Rust (Axum)** : un handler `async fn` qui retourne `impl IntoResponse` (par ex. `&'static str`), enregistré avec `Router::new().route("/", get(hello))`.

Le serveur est démarré avec :

- `TcpListener::bind("127.0.0.1:3000").await`
- puis `axum::serve(listener, app).await`

C’est l’équivalent de `http.ListenAndServe(":3000", router)`.

Voir dans [src/main.rs](src/main.rs) : fonction `hello` et construction du `Router` dans `main`.

---

## 5. État en RAM (Create + Get)

### Modèle

Une structure **Item** avec `id` et `name`, sérialisable avec **serde** (`Serialize` / `Deserialize`) — équivalent d’un struct Go avec des tags `json:"..."`.

### Stockage partagé

- **Arc** : permet de partager la même donnée entre plusieurs handlers (référence comptée).
- **RwLock** : accès lecture multiple / écriture exclusive (équivalent conceptuel de `sync.RWMutex`). On pourrait aussi utiliser `Mutex` (équivalent `sync.Mutex`) si on n’a pas besoin de lectures concurrentes.
- En Go tu aurais typiquement une `map[string]Item` protégée par un `sync.Mutex` (ou `RWMutex`) et passée par pointeur ou via un struct injecté.

Ici l’état est une struct **AppState** contenant `Arc<RwLock<HashMap<String, Item>>>`, qui implémente `Clone` et est passée au router avec `.with_state(state)`.

### Create (POST /items)

- Body JSON désérialisé en struct (ex. **CreateItemRequest** avec `name`).
- Génération d’un id (ici **uuid** v4), création de l’**Item**, insertion dans la `HashMap`, retour en 201 + JSON.

### Get (GET /items/:id)

- Paramètre de chemin extrait avec **Path&lt;String&gt;** (équivalent d’un paramètre de route en Go).
- Lecture dans la `HashMap` ; si trouvé → 200 + JSON, sinon 404.

Le code correspondant est dans [src/main.rs](src/main.rs) : `create_item`, `get_item`, `AppState`, et les routes `/items` et `/items/:id`.

---

## 6. OpenAPI / Swagger avec utoipa

- Un struct **ApiDoc** avec `#[derive(OpenApi)]` référence les paths (handlers) et les schemas (Item, CreateItemRequest).
- Les handlers sont annotés avec **#[utoipa::path(...)]** (méthode, path, request_body, responses).
- Les structs exposées en JSON dérivent **ToSchema** (utoipa) en plus de Serialize/Deserialize.

Swagger UI est monté via **utoipa_swagger_ui::SwaggerUi** :

- `.url("/api-docs/openapi.json", ApiDoc::openapi())` expose la spec OpenAPI.
- Le router Swagger est fusionné avec le router principal : `.merge(SwaggerUi::new("/swagger-ui").url(...))`.

En lançant l’API, la doc est disponible à : **http://127.0.0.1:3000/swagger-ui**.

---

## 7. Structure du projet

```
helloWorldAPI/
├── Cargo.toml      # Dépendances et métadonnées du crate
├── src/
│   └── main.rs     # Point d’entrée, routes, handlers, state, OpenAPI
└── GUIDE.md        # Ce fichier
```

Tout est dans un seul binaire ; on pourrait plus tard extraire state + handlers dans un `lib.rs` si besoin.

---

## 8. Lancer et tester

- **Démarrer l’API** : `cargo run` (équivalent de `go run .`).

Une fois le serveur lancé :

- **Hello World**  
  `curl http://localhost:3000/`
- **Créer un item**  
  `curl -X POST http://localhost:3000/items -H "Content-Type: application/json" -d '{"name":"test"}'`
- **Récupérer un item**  
  Récupérer l’`id` dans la réponse du POST, puis :  
  `curl http://localhost:3000/items/<id>`
- **Documentation**  
  Ouvrir http://127.0.0.1:3000/swagger-ui dans un navigateur.

---

## Note sur “Utopia”

Ce guide utilise **utoipa** (génération OpenAPI / Swagger). Si tu pensais à un autre outil (un crate nommé “Utopia”), la section 6 peut être adaptée en remplaçant utoipa par cette lib.
