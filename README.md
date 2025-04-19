# Happy to Bees Backend

Backend de l'application Happy to Bees, une plateforme de gestion d'apiculture.

## Fonctionnalités

- Gestion des utilisateurs (inscription, connexion, gestion de profil)
- Gestion des ruches (création, suivi, statistiques)
- Gestion des interventions (planification, suivi, historique)
- Gestion des productions (suivi du miel, statistiques)
- Gestion des matériels (inventaire, suivi de l'état)
- Gestion des poids (suivi du poids des ruches)
- Gestion des sessions (authentification, sécurité)

## Structure du Projet

```
src/
├── controllers/         # Contrôleurs pour les différentes entités
│   ├── intervention_controller.rs
│   ├── materiel_controller.rs
│   ├── poids_controller.rs
│   ├── production_controller.rs
│   ├── ruche_controller.rs
│   ├── session_controller.rs
│   └── utilisateur_controller.rs
├── models/             # Modèles de données
│   ├── intervention_models.rs
│   ├── materiel_models.rs
│   ├── poids_models.rs
│   ├── production_models.rs
│   ├── ruche_models.rs
│   ├── session_models.rs
│   └── utilisateur_models.rs
├── services/           # Services métier
│   ├── intervention_service.rs
│   ├── materiel_service.rs
│   ├── poids_service.rs
│   ├── production_service.rs
│   ├── ruche_service.rs
│   ├── session_service.rs
│   └── utilisateur_service.rs
├── db.rs              # Configuration de la base de données
├── main.rs            # Point d'entrée de l'application
```

## API Endpoints

### Utilisateurs
- `POST /api/utilisateurs` - Créer un nouvel utilisateur
- `POST /api/utilisateurs/login` - Connexion
- `GET /api/utilisateurs/{id}` - Obtenir un utilisateur
- `PUT /api/utilisateurs/{id}` - Mettre à jour un utilisateur
- `DELETE /api/utilisateurs/{id}` - Supprimer un utilisateur

### Ruches
- `GET /api/ruches` - Obtenir toutes les ruches
- `POST /api/ruches` - Créer une nouvelle ruche
- `GET /api/ruches/{id}` - Obtenir une ruche
- `PUT /api/ruches/{id}` - Mettre à jour une ruche
- `DELETE /api/ruches/{id}` - Supprimer une ruche

### Interventions
- `GET /api/interventions` - Obtenir toutes les interventions
- `POST /api/interventions` - Créer une nouvelle intervention
- `GET /api/interventions/{id}` - Obtenir une intervention
- `PUT /api/interventions/{id}` - Mettre à jour une intervention
- `DELETE /api/interventions/{id}` - Supprimer une intervention
- `GET /api/interventions/date/{date}` - Obtenir les interventions par date
- `GET /api/interventions/ruche/{ruche_id}` - Obtenir les interventions par ruche
- `GET /api/interventions/type/{type_intervention}` - Obtenir les interventions par type

### Productions
- `GET /api/productions` - Obtenir toutes les productions
- `POST /api/productions` - Créer une nouvelle production
- `GET /api/productions/{id}` - Obtenir une production
- `PUT /api/productions/{id}` - Mettre à jour une production
- `DELETE /api/productions/{id}` - Supprimer une production
- `GET /api/productions/ruche/{ruche_id}` - Obtenir les productions par ruche
- `GET /api/productions/ruche/{ruche_id}/statistiques` - Obtenir les statistiques de production

### Matériels
- `GET /api/materiels` - Obtenir tous les matériels
- `POST /api/materiels` - Créer un nouveau matériel
- `GET /api/materiels/{id}` - Obtenir un matériel
- `PUT /api/materiels/{id}` - Mettre à jour un matériel
- `DELETE /api/materiels/{id}` - Supprimer un matériel
- `GET /api/materiels/type/{type_materiel}` - Obtenir les matériels par type
- `GET /api/materiels/disponibles` - Obtenir les matériels disponibles
- `GET /api/materiels/etat/{etat}` - Obtenir les matériels par état

### Poids
- `GET /api/poids` - Obtenir tous les poids
- `POST /api/poids` - Créer un nouveau poids
- `GET /api/poids/{id}` - Obtenir un poids
- `PUT /api/poids/{id}` - Mettre à jour un poids
- `DELETE /api/poids/{id}` - Supprimer un poids
- `GET /api/poids/ruche/{ruche_id}` - Obtenir les poids par ruche
- `GET /api/poids/ruche/{ruche_id}/last` - Obtenir le dernier poids d'une ruche
- `GET /api/poids/ruche/{ruche_id}/average` - Obtenir la moyenne annuelle des poids
- `GET /api/poids/ruche/{ruche_id}/monthly-average` - Obtenir les moyennes mensuelles
- `GET /api/poids/ruche/{ruche_id}/evolution` - Obtenir l'évolution des poids

## Prérequis

- Rust (dernière version stable)
- PostgreSQL
- Diesel CLI

## Installation

1. Cloner le dépôt :
```bash
git clone https://github.com/votre-username/happy-to-bees-backend.git
cd happy-to-bees-backend
```

2. Installer les dépendances :
```bash
cargo build
```

3. Configurer la base de données :
```bash
diesel setup
diesel migration run
```

4. Configurer les variables d'environnement :
```bash
cp .env.example .env
# Éditer .env avec vos configurations
```

5. Lancer l'application :
```bash
cargo run
```

## Tests

Pour lancer les tests :
```bash
cargo test
```

## Contribution

1. Fork le projet
2. Créer une branche pour votre fonctionnalité (`git checkout -b feature/AmazingFeature`)
3. Commit vos changements (`git commit -m 'Add some AmazingFeature'`)
4. Push vers la branche (`git push origin feature/AmazingFeature`)
5. Ouvrir une Pull Request

## Licence

Ce projet est sous licence MIT. Voir le fichier `LICENSE` pour plus de détails. 