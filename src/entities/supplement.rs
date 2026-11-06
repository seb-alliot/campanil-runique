use runique::prelude::*;

model! {
    Supplement,
    table: "supplements",
    pk: id => Pk,
    {
        garniture_id: int     [nullable],
        titre:        text    [max_length: 255, nullable],
        libelle:      text    [max_length: 500, nullable],
        prix:         decimal [max_digits: 10, decimal_places: 2, required],
        disponible:   bool    [default: true],
        ordre:      int    [default: 0],
    },
    relations: {
        belongs_to: Garniture via garniture_id [set_null],
    },
    meta: {
        ordering: [ordre, titre],
        verbose_name: "Supplément",
        verbose_name_plural: "Suppléments",
    }
}
