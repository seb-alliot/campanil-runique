use runique::prelude::*;

#[form(schema = crate::entities::devis_traiteur, fields = [telephone, date_evenement, nb_personnes, message])]
pub struct DevisTraiteurForm;

impl RuniqueForm for DevisTraiteurForm {
    impl_form_access!(model);
    // `validator_get` defaults to `false` (runique) — submitting a devis
    // request never runs on a GET, no override needed.
}
