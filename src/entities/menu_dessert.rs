use runique::prelude::*;

model! {
    MenuDessert,
    table: "menu_desserts",
    pk: id => Pk,
    {
        menu_id:    int [required],
        dessert_id: int [required],
    },
    relations: {
        belongs_to: Menu via menu_id [cascade],
        belongs_to: Dessert via dessert_id [cascade],
    },
}
