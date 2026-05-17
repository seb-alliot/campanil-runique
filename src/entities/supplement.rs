use runique::prelude::*;

model! {
    Supplement,
    table: "supplements",
    pk: id => Pk,
    {
        garniture_id: int     [fk(garnitures.id, set_null)],
        titre:        text    [max_length: 255],
        libelle:      text    [max_length: 500],
        prix:         decimal [required],
        disponible:   bool    [required, default: true],
        ordre:      int    [default: 0],
    },
    relations: {
        belongs_to: Garniture via garniture_id,
    },
    meta: {
        ordering: [ordre, titre],
        verbose_name: "Supplément",
        verbose_name_plural: "Suppléments",
    }
}
