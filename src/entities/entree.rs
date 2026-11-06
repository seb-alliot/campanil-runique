use runique::prelude::*;

model! {
    Entree,
    table: "entrees",
    pk: id => Pk,
    enums: {
        UsageEntree: [
            Carte   = ("carte",    "Carte uniquement"),
            Menu    = ("menu",     "Menu uniquement"),
            LesDeux = ("les_deux", "Carte et menu"),
        ],
    },
    {
        titre:       text    [required, max_length: 255],
        label:       text    [max_length: 80, nullable],
        description: textarea [nullable],
        image:       image   [upload_to: "entrees/", nullable],
        prix:        decimal [max_digits: 10, decimal_places: 2, required],
        disponible:  bool    [default: true],
        usage:       choice  [enum(UsageEntree), required, default: "les_deux"],
        ordre:       int     [default: 0],
    },
    relations: {
        many_to_many: Allergene through EntreeAllergene via entree_id,
    },
    meta: {
        ordering: [ordre, titre],
        verbose_name: "Entrée",
        verbose_name_plural: "Entrées",
    }
}
