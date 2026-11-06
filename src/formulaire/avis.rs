use runique::prelude::*;

#[form(schema = crate::entities::avis, fields = [commentaire])]
pub struct AvisForm;

impl RuniqueForm for AvisForm {
}
