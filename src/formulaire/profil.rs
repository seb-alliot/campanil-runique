use runique::prelude::*;

#[form(schema = crate::entities::user_profil, fields = [telephone, adresse, code_postal, ville])]
pub struct ProfilForm;

impl RuniqueForm for ProfilForm {
    impl_form_access!(model);
}
