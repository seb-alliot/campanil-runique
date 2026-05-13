use runique::prelude::*;

model! {
    Supplement,
    table: "supplements",
    pk: id => Pk,
    {
        libelle:    text    [required, max_length: 255],
        prix:       decimal [required],
        disponible: bool    [required, default: true],
    },
    meta: {
        ordering: [libelle],
        verbose_name: "Supplément",
        verbose_name_plural: "Suppléments",
    }
}
