use runique::prelude::*;

model! {
    MenuEntree,
    table: "menu_entrees",
    pk: id => Pk,
    {
        menu_id:   int [required],
        entree_id: int [required],
    },
    relations: {
        belongs_to: Menu via menu_id [cascade],
        belongs_to: Entree via entree_id [cascade],
    },
}
