use runique::prelude::*;

#[form(schema = crate::entities::devis_traiteur, fields = [telephone, date_evenement, nb_personnes, message])]
pub struct DevisTraiteurForm;

impl RuniqueForm for DevisTraiteurForm {
    impl_form_access!(model);
}
