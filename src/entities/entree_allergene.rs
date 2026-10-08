use runique::prelude::*;

model! {
    EntreeAllergene,
    table: "entree_allergenes",
    pk: id => Pk,
    {
        entree_id:    int [required],
        allergene_id: int [required],
    },
    relations: {
        belongs_to: Entree via entree_id [cascade],
        belongs_to: Allergene via allergene_id [cascade],
    },
}
