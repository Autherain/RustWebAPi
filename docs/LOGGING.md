# Logging avec tracing

Pas de `logger::default()` ni de logger à passer en paramètre. Un **subscriber** global est initialisé une fois dans `main` ; tout le reste du code utilise les macros **tracing** directement.

## Utilisation (store, domain, server)

Dans n’importe quel module, appelez :

- `tracing::trace!(...)` — très détaillé
- `tracing::debug!(...)` — debug
- `tracing::info!(...)` — info
- `tracing::warn!(...)` — avertissement
- `tracing::error!(...)` — erreur

Les **attributs** (champs structurés) se mettent dans la macro :

```rust
// Message seul
tracing::info!("item créé");

// Avec attributs (équivalent "With(...)")
tracing::info!(item_id = %id, name = %name, "item créé");

// Erreur avec détail (? = Debug, % = Display)
tracing::error!(?err, id = %item_id, "get_by_id failed");
```

- `%var` — affiche `var` avec son `Display`
- `?var` — affiche `var` avec son `Debug`
- `%var` et `?var` ajoutent le champ au JSON en prod

## Où logger

- **Domain** : règles métier, validation, décisions.
- **Store** : accès données (create, get_by_id, etc.).
- **Server / handlers** : entrée requête, réponse, erreurs HTTP.

Aucun paramètre « logger » à ajouter aux fonctions : le contexte de tracing est global (et plus tard on pourra y ajouter un `trace_id` via un span dans un middleware).

## Niveau (RUST_LOG)

- Défaut : `info`.
- Pour tout en debug : `RUST_LOG=debug`.
- Pour un module : `RUST_LOG=hello_world_api=debug` ou `hello_world_api::store=debug`.

## Format

En prod le subscriber est configuré en **JSON** (une ligne = un objet JSON par événement). Les champs structurés et les spans (quand tu en ajouteras) apparaissent dans le JSON.
