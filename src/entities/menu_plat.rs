use runique::prelude::*;

model! {
    MenuPlat,
    table: "menu_plats",
    pk: id => Pk,
    {
        menu_id: int [required, fk(menus.id, cascade)],
        plat_id: int [required, fk(plats.id, cascade)],
    },
}
