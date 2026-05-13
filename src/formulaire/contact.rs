use runique::prelude::*;

#[form(schema = crate::entities::contact, fields = [titre, email, description])]
pub struct ContactForm;

impl RuniqueForm for ContactForm {
    impl_form_access!(model);
}
