use runique::prelude::*;

model! {
    MenuRestoPlat,
    table: "menu_resto_plat",
    pk: id => Pk,
    {
        menu_id: int [required, fk(menu_resto.id, cascade)],
        plat_id: int [required, fk(plats.id, cascade)],
        cours:   text [required, max_length: 10],
    },
    meta: {
        verbose_name: "Plat de menu resto",
        verbose_name_plural: "Plats de menus resto",
    }
}
