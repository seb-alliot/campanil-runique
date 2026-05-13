use crate::formulaire::LoginForm;
use runique::prelude::*;

pub fn get_credentials(form: &LoginForm) -> Option<(String, String)> {
    let u = form.cleaned_string("username")?;
    let p = form.cleaned_string("password")?;
    Some((u, p))
}
