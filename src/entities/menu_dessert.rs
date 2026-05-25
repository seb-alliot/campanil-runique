use runique::prelude::*;

model! {
    MenuDessert,
    table: "menu_desserts",
    pk: id => Pk,
    {
        menu_id:    int [required, fk(menus.id, cascade)],
        dessert_id: int [required, fk(desserts.id, cascade)],
    },
}
