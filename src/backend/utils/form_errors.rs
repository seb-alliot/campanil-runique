use runique::prelude::*;

/// Surfaces a form's global errors (e.g. the CSRF failure message, which lands
/// in `Forms.errors` rather than on a specific field) as flash messages.
pub fn form_error_flash<F: RuniqueForm>(form: &F) -> Option<Vec<FlashMessage>> {
    let errors = &form.get_form().errors;
    if errors.is_empty() {
        None
    } else {
        Some(
            errors
                .iter()
                .map(|e| FlashMessage::error(e.clone()))
                .collect(),
        )
    }
}
