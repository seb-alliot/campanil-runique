use crate::formulaire::LoginForm;
use runique::prelude::*;

pub fn get_credentials(form: &LoginForm) -> Option<(String, String)> {
    let u = form.cleaned::<String>("username")?;
    let p = form.cleaned::<String>("password")?;
    Some((u, p))
}
