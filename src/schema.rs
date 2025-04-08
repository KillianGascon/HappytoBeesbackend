// @generated automatically by Diesel CLI.

diesel::table! {
    interventions (id) {
        id -> Int4,
        id_ruche -> Nullable<Int4>,
        date_intervention -> Nullable<Date>,
        description_intervention -> Nullable<Text>,
        #[max_length = 255]
        photo_intervention -> Nullable<Varchar>,
    }
}

diesel::table! {
    materiel (id) {
        id -> Int4,
        id_ruche -> Nullable<Int4>,
        #[max_length = 50]
        nom_materiel -> Nullable<Varchar>,
        #[max_length = 50]
        type_materiel -> Nullable<Varchar>,
        #[max_length = 20]
        etat_materiel -> Nullable<Varchar>,
    }
}

diesel::table! {
    poids (id) {
        id -> Int4,
        id_ruche -> Nullable<Int4>,
        poids_ruche -> Nullable<Int4>,
        date_creation -> Nullable<Date>,
    }
}

diesel::table! {
    production (id) {
        id -> Int4,
        id_ruche -> Nullable<Int4>,
        quantite_production -> Nullable<Int4>,
        date_creation -> Nullable<Date>,
    }
}

diesel::table! {
    ruche (id) {
        id -> Int4,
        id_apiculteur -> Nullable<Int4>,
        #[max_length = 255]
        photo_ruche -> Nullable<Varchar>,
        numero_ruche -> Nullable<Int4>,
        #[max_length = 50]
        nom_ruche -> Nullable<Varchar>,
        nombre_cadres_corp -> Nullable<Int4>,
        nombre_hausses -> Nullable<Int4>,
        nombre_cadres_hausse -> Nullable<Int4>,
        nombre_cadre_couvain -> Nullable<Int4>,
        nombre_cadre_nourriture -> Nullable<Int4>,
        nombre_cadre_libre -> Nullable<Int4>,
    }
}

diesel::table! {
    sessions (id) {
        id -> Int4,
        id_utilisateur -> Nullable<Int4>,
        token -> Nullable<Text>,
        user_agent -> Nullable<Text>,
        ip_address -> Nullable<Text>,
        date_creation -> Nullable<Timestamp>,
        date_expiration -> Nullable<Timestamp>,
        est_valide -> Nullable<Bool>,
    }
}

diesel::table! {
    utilisateur (id) {
        id -> Int4,
        #[max_length = 50]
        nom_apiculteur -> Nullable<Varchar>,
        #[max_length = 50]
        prenom_apiculteur -> Nullable<Varchar>,
        #[max_length = 255]
        mail -> Nullable<Varchar>,
        #[max_length = 15]
        telephone -> Nullable<Varchar>,
        #[max_length = 255]
        mot_de_passe -> Nullable<Varchar>,
        numero_apiculteur -> Nullable<Int4>,
        date_naissance -> Nullable<Date>,
    }
}

diesel::joinable!(interventions -> ruche (id_ruche));
diesel::joinable!(materiel -> ruche (id_ruche));
diesel::joinable!(poids -> ruche (id_ruche));
diesel::joinable!(production -> ruche (id_ruche));
diesel::joinable!(ruche -> utilisateur (id_apiculteur));
diesel::joinable!(sessions -> utilisateur (id_utilisateur));

diesel::allow_tables_to_appear_in_same_query!(
    interventions,
    materiel,
    poids,
    production,
    ruche,
    sessions,
    utilisateur,
);