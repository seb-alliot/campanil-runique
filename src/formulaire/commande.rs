use runique::prelude::*;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(transparent)]
pub struct CommandeAdminEditForm {
    pub form: Forms,
}

impl RuniqueForm for CommandeAdminEditForm {
    fn register_fields(form: &mut Forms) {
        form.add_js(&["js/admin/commande_materiel.js"]);
        form.field(&HiddenField::new("pret_materiel"));
        form.field(&HiddenField::new("penalite_envoyee"));

        form.field(
            &ChoiceField::new("statut")
                .label("Statut")
                .required()
                .add_choice("en_attente", "En attente")
                .add_choice("accepte", "Accepté")
                .add_choice("en_preparation", "En préparation")
                .add_choice("pret", "Prêt")
                .add_choice("en_cours_livraison", "En cours de livraison")
                .add_choice("livre", "Livré")
                .add_choice("termine", "Terminé")
                .add_choice("annule", "Annulé"),
        );
        form.field(
            &ChoiceField::new("mode_paiement")
                .label("Mode de paiement")
                .required()
                .add_choice("especes", "Espèces")
                .add_choice("carte_bancaire", "Carte bancaire")
                .add_choice("en_ligne", "Paiement en ligne"),
        );
        form.field(
            &ChoiceField::new("type_retrait")
                .label("Type de retrait")
                .required()
                .add_choice("sur_place", "Sur place")
                .add_choice("livraison", "Livraison"),
        );
        form.field(&DateTimeField::new("heure_retrait").label("Heure de retrait"));
        form.field(
            &TextField::text("adresse_livraison")
                .label("Adresse de livraison")
                .max_length(255, "255 caractères maximum"),
        );
        form.field(
            &TextField::text("ville_livraison")
                .label("Ville")
                .max_length(100, "100 caractères maximum"),
        );
        form.field(
            &TextField::text("cp_livraison")
                .label("Code postal")
                .max_length(10, "10 caractères maximum"),
        );
        form.field(&NumericField::decimal("prix_livraison").label("Prix de livraison"));
    }

    impl_form_access!();
}

pub struct CommandeForm {
    pub mode_paiement: String,
    pub type_retrait: String,
    pub heure_retrait: Option<String>,
    pub adresse_livraison: Option<String>,
    pub ville_livraison: Option<String>,
    pub cp_livraison: Option<String>,
}
