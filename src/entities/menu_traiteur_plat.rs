use runique::prelude::*;

model! {
    MenuTraiteurPlat,
    table: "menu_traiteur_plats",
    pk: id => Pk,
    {
        menu_traiteur_id: int [required],
        plat_id:          int [required, fk(plats.id, cascade)],
    },
    meta: {
        unique_together: [(menu_traiteur_id, plat_id)],
    }
}
