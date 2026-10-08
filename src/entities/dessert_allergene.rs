use runique::prelude::*;

model! {
    DessertAllergene,
    table: "dessert_allergenes",
    pk: id => Pk,
    {
        dessert_id:   int [required],
        allergene_id: int [required],
    },
    relations: {
        belongs_to: Dessert via dessert_id [cascade],
        belongs_to: Allergene via allergene_id [cascade],
    },
}
