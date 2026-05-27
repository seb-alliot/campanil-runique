# Sécurité du projet — U Campanile

Ce document décrit les mécanismes de sécurité en place, basés sur les implémentations réelles de Runique.

---

## HTTPS

L'application tourne derrière HTTPS. En déploiement sans reverse proxy, Runique active ACME (Let's Encrypt) via `ACME_ENABLED=true` et gère lui-même l'émission et le renouvellement du certificat. Sur le VPS de production, c'est Certbot + Nginx qui prend en charge le TLS.

La redirection HTTP → HTTPS est forcée via `ENFORCE_HTTPS=true`. Le header `Strict-Transport-Security: max-age=31536000; includeSubDomains; preload` est injecté automatiquement par le middleware CSP de Runique.

---

## Sessions

Les sessions reposent sur `tower-sessions`. Le cookie de session est configuré avec les drapeaux suivants :

- `HttpOnly` — inaccessible depuis JavaScript, protège contre le vol de cookie via XSS
- `Secure` — envoyé uniquement sur HTTPS (activé quand `DEBUG=false`)
- `SameSite=Strict` — non envoyé sur les requêtes cross-site, protège contre le CSRF passif

La durée de session par défaut est de 24 heures. Les sessions anonymes peuvent être purgées sous pression mémoire (watermark à 128 MB) mais les sessions authentifiées sont protégées. Après redémarrage, les sessions authentifiées sont restaurées depuis la base de données.

---

## CSRF

La protection CSRF est implémentée par Runique. Le token est généré via HMAC-SHA256 sur la clé secrète de l'application, un identifiant lié à l'état de la session, et un timestamp en nanosecondes :

```text
// Utilisateur anonyme
HMAC-SHA256(SECRET_KEY, "runique.middleware.csrf" || session_id || timestamp_nanos)

// Utilisateur authentifié
HMAC-SHA256(SECRET_KEY, "runique.middleware.csrf" || user_id || timestamp_nanos)
```

Quand l'utilisateur est connecté, le token est lié à son `user_id` (récupéré depuis la session) et non à l'identifiant de session — ce qui garantit que le token reste stable entre les renouvellements de session tout en restant propre à l'utilisateur.

Le token est ensuite masqué (XOR avec 32 bytes aléatoires + base64url) avant injection dans le formulaire HTML, ce qui protège contre l'attaque BREACH (compression oracle sur HTTPS).

La validation utilise `subtle::ConstantTimeEq` — comparaison en temps constant — pour prévenir les attaques par timing. Les requêtes GET et HEAD sont exemptes (pas d'effet de bord).

---

## SRI (Subresource Integrity)

Au démarrage, Runique calcule les hashes SHA-256 de tous les fichiers statiques du projet et construit une `integrity_map`. Le mécanisme fonctionne en deux étapes :

### Étape 1 — Preprocessing des templates (au démarrage, avant la première requête)

Chaque balise `{% static "chemin" %}` avec un chemin littéral est transformée. Si le fichier est présent dans l'`integrity_map`, le preprocessing produit :

```html
{{ "chemin" | static }}" integrity="sha256-..." crossorigin="anonymous"
```

Le `"` après l'expression Tera ferme l'attribut `src` ou `href` ; `integrity` et `crossorigin` sont injectés comme attributs HTML séparés.

### Étape 2 — Rendu Tera (à chaque requête)

`{{ "chemin" | static }}` est résolu en `/static/chemin?v=TOKEN`. Le template final reçu par le navigateur est :

```html
<script src="/static/app.js?v=TOKEN" integrity="sha256-..." crossorigin="anonymous"></script>
```

Le navigateur vérifie le hash avant d'exécuter ou d'appliquer la ressource — toute modification du fichier invalide le hash et bloque le chargement. Ce mécanisme ne s'applique qu'aux chemins littéraux ; les chemins dynamiques (`{% static var %}`) sont résolus en URL simple sans SRI.

---

## CSP (Content Security Policy)

Runique injecte une politique CSP stricte à chaque réponse. Un nonce de 128 bits est généré par requête et injecté dans les balises `<script>` via le tag `{% csp %}`. La configuration par défaut inclut notamment :

- `default-src: 'none'`
- `script-src: 'self'` + nonce par requête
- `object-src: 'none'`
- `frame-ancestors: 'none'`
- `form-action: 'self'`

Headers de sécurité supplémentaires injectés automatiquement :

- `X-Content-Type-Options: nosniff`
- `X-Frame-Options: DENY`
- `X-XSS-Protection: 1; mode=block`
- `Referrer-Policy: strict-origin-when-cross-origin`
- `Cross-Origin-Embedder-Policy: require-corp`
- `Cross-Origin-Opener-Policy: same-origin`
- `Cross-Origin-Resource-Policy: same-origin`

---

## Permissions-Policy

Runique injecte automatiquement le header `Permissions-Policy` à chaque réponse. La configuration par défaut bloque l'accès à toutes les API matérielles et de vie privée :

- Capteurs désactivés : `accelerometer`, `gyroscope`, `magnetometer`, `ambient-light-sensor`
- Périphériques désactivés : `camera`, `microphone`, `bluetooth`, `usb`, `hid`, `midi`, `serial`
- Localisation et identité désactivées : `geolocation`, `idle-detection`
- Paiement désactivé : `payment`
- Empreinte numérique réduite : `interest-cohort`, `local-fonts`
- Capture désactivée : `display-capture`
- XR désactivé : `xr-spatial-tracking`
- `sync-xhr` désactivé (API dépréciée)

Autorisés uniquement sur la même origine : `fullscreen`, `picture-in-picture`, `publickey-credentials-create`, `publickey-credentials-get` (nécessaires pour les passkeys).

---

## XSS

Tera (le moteur de templates) active l'autoescape sur tous les fichiers `.html` et `.xml`. Toute variable `{{ var }}` est automatiquement échappée. L'utilisation de `{{ var | safe }}` est réservée aux cas où le contenu est explicitement validé comme sûr.

---

## Mots de passe

Le hachage des mots de passe est géré par `PasswordService` dans Runique. L'algorithme par défaut est **Argon2id** avec un sel aléatoire généré via `OsRng`. Bcrypt et Scrypt sont également supportés.

Le moteur de formulaire intègre un traitement automatique (`auto_process`) : lors de la validation d'un champ mot de passe, le mot de passe en clair est haché de façon transparente avant toute persistance. Un `pre_hash_hook` optionnel permet d'ajouter une validation personnalisée (complexité, longueur minimale) avant le hachage.

Le service détecte automatiquement l'algorithme utilisé à partir du préfixe du hash stocké (`$argon2id$`, `$2`, `$scrypt$`), ce qui permet un upgrade progressif des anciens hashes lors de la connexion.

---

## Rate Limiting

Un middleware de rate limiting par IP est implémenté par Runique avec une fenêtre glissante. La configuration par défaut est de 60 requêtes par 60 secondes. Au-delà, la réponse est HTTP 429 avec un header `Retry-After`.

Sur Campanile, le rate limiting est appliqué de façon ciblée par route :

- Routes de connexion et d'inscription : toutes les méthodes HTTP
- Routes de formulaires de contact, devis et commande : uniquement les requêtes POST

L'IP réelle est extraite par le middleware `trusted_proxies` (voir section dédiée).

---

## Proxies de confiance

Runique injecte le middleware `trusted_proxies` qui extrait l'IP réelle du client en tenant compte de la chaîne de proxies. L'algorithme :

1. Si l'IP de connexion directe n'est pas dans la liste des proxies de confiance, elle est utilisée telle quelle.
2. Sinon, le header `X-Forwarded-For` est parcouru **de droite à gauche** en sautant les entrées de confiance, jusqu'à trouver la première IP non fiable — c'est l'IP client réelle.
3. Si toutes les entrées XFF sont de confiance, l'entrée la plus à gauche (déclarée par le client) est utilisée.

Les plages de confiance par défaut couvrent les réseaux privés RFC 1918 et les loopbacks :

- `127.0.0.0/8`, `::1` — loopback
- `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16` — privé IPv4
- `fc00::/7` — unique local IPv6

Cela empêche un attaquant d'usurper son IP en forgeant un header `X-Forwarded-For` depuis l'extérieur.

---

## Anti-bot (Honeypot)

Un champ honeypot est injecté dans tous les formulaires. Son nom est un identifiant hexadécimal de 16 caractères généré aléatoirement et lié à la session — il est donc différent pour chaque utilisateur et change à chaque nouvelle session.

Le champ est masqué visuellement par un fichier CSS dédié (`hp.css`, classe `position: absolute; left: -9999px`). Un bot qui remplit tous les champs du formulaire remplira aussi le honeypot, et la soumission sera automatiquement invalide — sans que le handler ne le sache.

---

## Uploads

La validation des fichiers uploadés repose sur deux niveaux :

1. **Magic bytes** — les premiers octets du fichier sont lus et comparés aux signatures connues (JPEG, PNG, GIF, WebP, AVIF). L'extension seule ne suffit pas.
2. **Liste blanche d'extensions** — les fichiers SVG sont explicitement bloqués (risque XSS via SVG inline).

La taille maximale par défaut est de 10 MB, configurable par champ. Les dimensions maximales d'image sont également configurables. En cas d'échec de validation, les fichiers partiellement uploadés sont supprimés automatiquement.

---

## Sanitisation HTML (XSS côté serveur)

Le moteur de formulaire de Runique applique une sanitisation via la crate **ammonia** sur tous les champs texte avant validation. Deux modes coexistent :

- **Mode strict** (champs texte ordinaires) — toutes les balises HTML sont supprimées. Les protocoles dangereux (`javascript:`, `vbscript:`, `data:`, `file:`) sont retirés même s'ils sont obfusqués ou contiennent des octets nuls.
- **Mode rich** (champs rich-text) — seule une liste blanche de balises et attributs est autorisée. Les URLs sont limitées aux schémas `http` et `https`, les URLs relatives sont bloquées, les commentaires HTML sont supprimés, et `rel="noopener noreferrer"` est forcé sur tous les liens.

La sanitisation est appliquée en amont de la validation des champs — une valeur malveillante ne passe jamais au moteur de validation ni au handler.

---

## Injection SQL

Toutes les interactions avec la base de données passent par SeaORM et la macro `search!{}` de Runique. Aucune requête SQL brute n'est construite par concaténation de chaînes. Les valeurs sont transmises comme paramètres liés, ce qui empêche structurellement les injections SQL.

---

## ALLOWED_HOSTS

Le middleware `allowed_hosts` valide le header `Host` de chaque requête entrante contre une liste d'hôtes autorisés. Les requêtes vers un hôte non reconnu reçoivent une réponse HTTP 400 — ce qui empêche les attaques par host header injection et le cache poisoning.

La liste supporte les correspondances exactes (`campanile.fr`), les wildcards de sous-domaine (`.campanile.fr` autorise `www.campanile.fr`, `api.campanile.fr`, etc.) et le wildcard total (`*`). Pour HTTP/2, le pseudo-header `:authority` est utilisé en l'absence du header `Host`.

---

## Open Redirect

Runique valide les URLs de redirection avant de les utiliser. Les chemins relatifs (`/path`) sont toujours considérés sûrs. Les URLs absolues sont vérifiées contre la liste `ALLOWED_HOSTS`. Les adresses IPv6 (y compris les formes courtes `::1` et les IPv4-mapped) sont correctement normalisées avant comparaison.

---

## Réinitialisation de mot de passe

Le flow de réinitialisation de mot de passe est implémenté par Runique avec plusieurs couches de protection :

- **Token UUID v4** — opaque, aléatoire, impossible à deviner.
- **TTL de 1 heure** — le token expire automatiquement. Les tokens expirés sont purgés du store en mémoire à chaque nouvelle génération.
- **Usage unique** — `consume()` supprime le token du store au moment de la validation du formulaire. Un lien de réinitialisation ne peut être utilisé qu'une seule fois.
- **Email chiffré dans l'URL** — l'email est chiffré avec le token comme clé (CTR-SHA256) et authentifié par un tag HMAC-SHA256 (16 bytes) vérifié en temps constant via `ct_eq`. Cela empêche la manipulation de l'URL pour usurper un autre compte.
- **Non-énumération** — que l'email soumis existe ou non dans la base, la réponse est toujours identique (`"Vérifiez votre boîte mail"`). L'existence d'un compte n'est jamais révélée.
- **Rate limiting dédié** — les routes `/forgot-password` et `/reset-password` ont leur propre limiteur : 5 requêtes maximum, blocage 300 secondes.

---

## CORS

Runique intègre un middleware CORS basé sur `tower-http`, mais il est **désactivé par défaut**. Il nécessite une activation explicite via `.with_cors()` dans le builder. Campanile ne le configure pas — l'application est entièrement rendue côté serveur et ne reçoit aucune requête cross-origin légitime.

La combinaison `any_origin()` + `allow_credentials(true)` est détectée comme une `BuildError` au démarrage, ce qui empêche une configuration dangereuse d'atteindre la production.

---

## Mode DEBUG

La variable d'environnement `DEBUG` contrôle deux comportements de sécurité distincts :

- `DEBUG=false` — le flag `Secure` est activé sur le cookie de session (envoi HTTPS uniquement) et les pages d'erreur ne renvoient pas de stack trace au navigateur.
- `DEBUG=true` — utilisé uniquement en développement local. Ne jamais déployer en production avec `DEBUG=true`.

---

## Variables sensibles

La `SECRET_KEY` (utilisée pour le HMAC CSRF et le chiffrement des tokens de réinitialisation) est lue exclusivement depuis la variable d'environnement. Elle n'est jamais présente dans le code source ni dans le dépôt git. Le fichier `.env` est listé dans `.gitignore`.
