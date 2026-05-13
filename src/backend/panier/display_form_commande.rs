use crate::formulaire::CommandeForm;

pub fn commander_form_from_body(data: &std::collections::HashMap<String, String>) -> CommandeForm {
    let get = |key: &str| -> Option<String> {
        data.get(key)
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
    };
    CommandeForm {
        mode_paiement: get("mode_paiement").unwrap_or_default(),
        heure_retrait: get("heure_retrait"),
        avec_livraison: get("avec_livraison").as_deref() == Some("1"),
        adresse_livraison: get("adresse_livraison"),
        ville_livraison: get("ville_livraison"),
        cp_livraison: get("cp_livraison"),
        heure_livraison: get("heure_livraison"),
    }
}
