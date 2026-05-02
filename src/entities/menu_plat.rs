use runique::prelude::*;

model! {
    MenuPlat,
    table: "menu_plat",
    pk: id => Pk,
    {
        menu_id: int [required, fk(menus.id, cascade)],
        plat_id: int [required, fk(plats.id, cascade)],
    },
    relations: {
        belongs_to: Menu via menu_id,
        belongs_to: Plat via plat_id,
    },
    meta: {
        unique_together: [(menu_id, plat_id)],
    }
}
