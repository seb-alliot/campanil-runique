use runique::prelude::*;

model! {
    Garniture,
    table: "garnitures",
    pk: id => Pk,
    enums: {
        TypeGarniture: [
            Feculent = ("feculent", "Féculent"),
            Legumes  = ("legumes",  "Légumes"),
            Sauce    = ("sauce",    "Sauce"),
        ],
    },
    {
        libelle:        text    [required, max_length: 100],
        type_garniture: choice  [enum(TypeGarniture), required],
        disponible:     bool    [required, default: true],
    },
    meta: {
        ordering: [type_garniture, libelle],
        verbose_name: "Garniture",
        verbose_name_plural: "Garnitures",
    }
}
