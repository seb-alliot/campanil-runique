use runique::prelude::*;

#[form(schema = crate::entities::contact, fields = [raison, titre, email, description])]
pub struct ContactForm;

impl RuniqueForm for ContactForm {
    impl_form_access!(model);
    // `validator_get` defaults to `false` (runique) — submitting the contact
    // form never runs on a GET, no override needed.
}
