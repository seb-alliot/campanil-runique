use crate::formulaire::CommandeForm;

pub fn commander_form_from_body(data: &std::collections::HashMap<String, String>) -> CommandeForm {
    let get = |key: &str| -> Option<String> {
        data.get(key)
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
    };
    CommandeForm {
        mode_paiement: get("mode_paiement").unwrap_or_default(),
        type_retrait: get("type_retrait").unwrap_or_else(|| "sur_place".to_string()),
        heure_retrait: get("heure_retrait"),
        adresse_livraison: get("adresse_livraison"),
        ville_livraison: get("ville_livraison"),
        cp_livraison: get("cp_livraison"),
        prix_livraison: get("prix_livraison"),
    }
}
