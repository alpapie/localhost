# Documentation du Projet

## Arborescence du Projet

src
├── config
│ └── parse.rs
├── config.json
├── error
│ └── mod.rs
├── main.rs
├── request
│ ├── mod.rs
│ └── parse_header.rs
└── response
└── mod.rs

markdown
Copy code

## Description des Dossiers et Fichiers

### 1. src/config/parse.rs

Ce fichier contient les fonctions et les structures pour lire et analyser le fichier de configuration `config.json`.

- **parse_config()** : Fonction principale pour lire et analyser le fichier de configuration.
- **Config** : Structure représentant les options de configuration.

### 2. src/config.json

Fichier de configuration principal pour le serveur. Contient les paramètres suivants :

- **server_address** : Adresse IP et port du serveur.
- **error_pages** : Chemins vers les pages d'erreur personnalisées.
- **client_max_body_size** : Limite de la taille des corps de requête.
- **routes** : Liste des routes avec leurs configurations spécifiques.

### 3. src/error/mod.rs

Module de gestion des erreurs. Définit les erreurs possibles et leurs traitements.

- **ServerError** : Enumération des erreurs possibles du serveur.
- **handle_error()** : Fonction pour gérer les erreurs et générer les réponses appropriées.

### 4. src/main.rs

Fichier principal du projet. Contient le point d'entrée du serveur.

- **main()** : Fonction principale qui initialise et lance le serveur.
- **setup_server()** : Fonction pour configurer et démarrer le serveur.

### 5. src/request/mod.rs

Module de gestion des requêtes. Regroupe les fonctions et structures liées aux requêtes HTTP.

- **Request** : Structure représentant une requête HTTP.
- **parse_request()** : Fonction pour analyser les requêtes HTTP.

### 6. src/request/parse_header.rs

Fichier pour l'analyse des en-têtes HTTP des requêtes.

- **parse_headers()** : Fonction pour extraire et analyser les en-têtes d'une requête HTTP.
- **Header** : Structure représentant un en-tête HTTP.

### 7. src/response/mod.rs

Module de gestion des réponses. Regroupe les fonctions et structures liées aux réponses HTTP.

- **Response** : Structure représentant une réponse HTTP.
- **build_response()** : Fonction pour créer et envoyer des réponses HTTP.

## Utilisation

### Configuration

1. Ouvrir le fichier `config.json` et modifier les paramètres selon les besoins du serveur.
2. Lancer le serveur en exécutant la commande suivante :
   ```sh
   cargo run