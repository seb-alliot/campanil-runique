use runique::prelude::*;

model! {
    Boisson,
    table: "boissons",
    pk: id => Pk,
    enums: {
        TypeBoisson: [
            VinRouge  = ("vin_rouge",    "Vin rouge"),
            VinRose   = ("vin_rose",     "Vin rosé"),
            VinBlanc  = ("vin_blanc",    "Vin blanc"),
            Champagne = ("champagne",    "Champagne"),
            Biere     = ("biere",        "Bière"),
            Aperitif  = ("aperitif",     "Apéritif"),
            Soft      = ("soft",         "Soft"),
            Eau       = ("eau_bouteille","Eau"),
            Digestif  = ("digestif",     "Digestif"),
        ],
    },
    {
        titre:        text    [required, max_length: 255],
        type_boisson: choice  [enum(TypeBoisson), required],
        prix:         decimal [required],
        description:  textarea,
        image:        image   [upload_to: "boissons/"],
        disponible:   bool    [required, default: true],
        ordre:        int     [default: 0],
        created_at:   datetime [auto_now],
    },
    meta: {
        ordering: [type_boisson, ordre, titre],
        verbose_name: "Boisson",
        verbose_name_plural: "Boissons",
    }
}
