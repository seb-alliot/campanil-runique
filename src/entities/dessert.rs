use runique::prelude::*;

model! {
    Dessert,
    table: "desserts",
    pk: id => Pk,
    enums: {
        UsageDessert: [
            Carte   = ("carte",    "Carte uniquement"),
            Menu    = ("menu",     "Menu uniquement"),
            LesDeux = ("les_deux", "Carte et menu"),
        ],
    },
    {
        titre:       text    [required, max_length: 255],
        label:       text    [max_length: 80],
        description: textarea,
        image:       image   [upload_to: "desserts/"],
        prix:        decimal [required],
        disponible:  bool    [required, default: true],
        usage:       choice  [enum(UsageDessert), required, default: "les_deux"],
        ordre:       int     [default: 0],
    },
    relations: {
        many_to_many: Allergene through DessertAllergene via dessert_id,
    },
    meta: {
        ordering: [ordre, titre],
        verbose_name: "Dessert",
        verbose_name_plural: "Desserts",
    }
}
