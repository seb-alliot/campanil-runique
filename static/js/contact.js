document.addEventListener('DOMContentLoaded', () => {
    const form = document.getElementById('contact-form');
    if (!form) return;

    form.addEventListener('submit', async (e) => {
        e.preventDefault();

        const btn = document.getElementById('contact-submit');
        btn.disabled = true;

        const res = await fetch('/contact', {
            method: 'POST',
            body: new FormData(form),
        });

        if (res.redirected || res.ok) {
            form.hidden = true;
            document.getElementById('contact-success').hidden = false;
        } else {
            btn.disabled = false;
        }
    });
});
