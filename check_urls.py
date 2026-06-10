#!/usr/bin/env python3
# /// script
# requires-python = ">=3.11"
# dependencies = ["requests"]
# ///
import requests

BASE = "http://localhost:3000"

# (path, expected_status) — 302 pour les routes auth-only
ROUTES = [
    ("/",                           200),
    ("/carte",                      200),
    ("/menus",                      200),
    ("/contact",                    200),
    ("/connexion",                  200),
    ("/inscription",                200),
    ("/forgot-password",            200),
    ("/mentions-legales",           200),
    ("/sitemap.xml",                200),
    ("/llms.txt",                   200),
    ("/panier",                     200),
    ("/compte",                     302),
    ("/service",                    302),
    ("/deconnexion",                302),
    ("/boissons/vin_rouge",         200),
]

OK = "\033[92m OK \033[0m"
FAIL = "\033[91mFAIL\033[0m"

errors = 0
for path, expected in ROUTES:
    try:
        r = requests.get(BASE + path, allow_redirects=False, timeout=5)
        ok = r.status_code == expected
        label = OK if ok else FAIL
        print(f"[{label}] {r.status_code} (attendu {expected})  {path}")
        if not ok:
            errors += 1
    except Exception as e:
        print(f"[{FAIL}] ERR  {path}  — {e}")
        errors += 1

print(f"\n{len(ROUTES) - errors}/{len(ROUTES)} OK")
