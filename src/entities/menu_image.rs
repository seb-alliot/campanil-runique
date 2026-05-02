use runique::prelude::*;

model! {
    MenuImage,
    table: "menu_images",
    pk: id => Pk,
    {
        menu_id: int   [required, fk(menus.id, cascade)],
        image:   image [required, upload_to: "menus/"],
        ordre:   int   [required, default: 0],
    },
    relations: {
        belongs_to: Menu via menu_id,
    },
    meta: {
        ordering: [menu_id, ordre],
    }
}
