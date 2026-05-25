use runique::prelude::*;

model! {
    DessertAllergene,
    table: "dessert_allergenes",
    pk: id => Pk,
    {
        dessert_id:   int [required, fk(desserts.id, cascade)],
        allergene_id: int [required, fk(allergenes.id, cascade)],
    }
}
