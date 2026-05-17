use runique::prelude::*;

model! {
    MenuEnfantAllergene,
    table: "menu_enfant_allergenes",
    pk: id => Pk,
    {
        menu_enfant_id: int [required, fk(menu_enfants.id, cascade)],
        allergene_id:   int [required, fk(allergenes.id, cascade)],
    },
    meta: {
        verbose_name: "Allergène menu enfant",
        verbose_name_plural: "Allergènes menu enfant",
    }
}
