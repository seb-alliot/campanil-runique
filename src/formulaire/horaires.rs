use runique::prelude::*;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(transparent)]
pub struct HorairesGroupeForm {
    pub form: Forms,
}

impl RuniqueForm for HorairesGroupeForm {
    fn register_fields(form: &mut Forms) {
        form.field(
            &CheckboxField::new("jour")
                .label("Jours")
                .add_choice("lundi", "Lundi")
                .add_choice("mardi", "Mardi")
                .add_choice("mercredi", "Mercredi")
                .add_choice("jeudi", "Jeudi")
                .add_choice("vendredi", "Vendredi")
                .add_choice("samedi", "Samedi")
                .add_choice("dimanche", "Dimanche"),
        );
        form.field(&TimeField::new("ouverture_midi").label("Ouverture midi"));
        form.field(&TimeField::new("fermeture_midi").label("Fermeture midi"));
        form.field(&TimeField::new("ouverture_soir").label("Ouverture soir"));
        form.field(&TimeField::new("fermeture_soir").label("Fermeture soir"));
        form.field(&BooleanField::new("ferme").label("Fermé"));
        form.field(&TextField::textarea("note").label("Note"));
    }

    impl_form_access!();
}
