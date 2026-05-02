use runique::prelude::*;

model! {
    Contact,
    table: "contacts",
    pk: id => Pk,
    {
        titre:       text     [required, max_length: 255],
        description: textarea [required],
        email:       email    [required],
        created_at:  datetime [auto_now],
    },
    meta: {
        ordering: [-created_at],
        verbose_name: "Message de contact",
        verbose_name_plural: "Messages de contact",
    }
}
