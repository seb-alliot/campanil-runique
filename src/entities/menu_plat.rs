use runique::prelude::*;

model! {
    MenuPlat,
    table: "menu_plats",
    pk: id => Pk,
    {
        menu_id: int [required],
        plat_id: int [required],
    },
    relations: {
        belongs_to: Menu via menu_id [cascade],
        belongs_to: Plat via plat_id [cascade],
    },
}
