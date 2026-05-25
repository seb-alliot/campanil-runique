use runique::prelude::*;

model! {
    EntreeAllergene,
    table: "entree_allergenes",
    pk: id => Pk,
    {
        entree_id:    int [required, fk(entrees.id, cascade)],
        allergene_id: int [required, fk(allergenes.id, cascade)],
    }
}
