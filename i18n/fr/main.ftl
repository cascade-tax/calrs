# Booking confirmation page (templates/confirmed.html)

confirmed-page-title-pending = Réservation en attente
confirmed-page-title-booked = Réservation confirmée

confirmed-heading-reschedule-requested = Reprogrammation demandée
confirmed-heading-rescheduled = Reprogrammé !
confirmed-heading-pending = En attente de confirmation
confirmed-heading-booked = C'est réservé !

confirmed-subtitle-reschedule-requested = Votre demande de reprogrammation a été envoyée à { $host }. Vous recevrez un e-mail à l'adresse { $email } une fois qu'elle sera approuvée.
confirmed-subtitle-rescheduled = Votre réservation a été reprogrammée. Un e-mail de confirmation a été envoyé à { $email }.
confirmed-subtitle-pending = Votre demande de réservation a été envoyée à { $host }. Vous recevrez un e-mail à l'adresse { $email } une fois qu'elle sera confirmée.
confirmed-subtitle-booked = Un e-mail de confirmation a été envoyé à { $email }.

confirmed-detail-event = Événement :
confirmed-detail-date = Date :
confirmed-detail-time = Heure :
confirmed-detail-with = Avec :
confirmed-detail-location = Lieu :
confirmed-detail-notes = Notes :
confirmed-detail-additional-guests = Invités supplémentaires :

confirmed-book-another = Réserver un autre créneau

# Slot picker (templates/slots.html)

slots-location-video = Visioconférence
slots-location-phone = Appel téléphonique

slots-tz-label = Votre fuseau horaire
slots-time-format-label = Format de l'heure

slots-view-month = Vue mensuelle
slots-view-week = Vue hebdomadaire
slots-view-column = Vue en liste

slots-weekday-mon = Lun
slots-weekday-tue = Mar
slots-weekday-wed = Mer
slots-weekday-thu = Jeu
slots-weekday-fri = Ven
slots-weekday-sat = Sam
slots-weekday-sun = Dim

slots-weekday-mon-short = L
slots-weekday-tue-short = M
slots-weekday-wed-short = M
slots-weekday-thu-short = J
slots-weekday-fri-short = V
slots-weekday-sat-short = S
slots-weekday-sun-short = D

slots-select-date = Choisissez une date
slots-loading-availability = Chargement des disponibilités...
slots-click-highlighted = Cliquez sur une date en surbrillance pour voir les créneaux disponibles
slots-no-times-month = Aucun créneau disponible ce mois-ci
slots-no-times-day = Aucun créneau disponible ce jour
slots-no-availability-participants = Aucune disponibilité commune trouvée pour tous les participants ce mois-ci
slots-week-more = autres

# Booking form (templates/book.html)

book-page-title = Réserver { $title }
book-back-to-times = Retour aux créneaux
book-name-label = Votre nom
book-name-placeholder = Jeanne Dupont
book-email-label = Adresse e-mail
book-email-placeholder = jeanne@example.com
book-email-invalid = Veuillez saisir une adresse e-mail complète, avec le nom de domaine (par ex. jeanne@example.com).
book-notes-label = Notes
book-notes-optional = (facultatif)
book-notes-placeholder = Y a-t-il des points que vous aimeriez aborder ?
book-additional-guests-label = Invités supplémentaires
book-additional-guests-hint = (facultatif, jusqu'à { $max })
book-add-guest-btn = + Ajouter un invité
book-guest-email-placeholder = collegue@example.com
captcha-label = Vérification de sécurité
captcha-initial-state = Vérifiez que vous êtes humain
captcha-verifying = Vérification en cours...
captcha-solved = Vous êtes humain
captcha-error = Erreur
captcha-troubleshooting = Dépannage
captcha-wasm-disabled = Activez WASM pour une résolution plus rapide
captcha-verify-aria = Cliquez pour vérifier que vous êtes humain
captcha-verifying-aria = Vérification en cours, veuillez patienter
captcha-verified-aria = Vérifié
captcha-required = Veuillez vérifier que vous êtes humain
captcha-error-aria = Une erreur est survenue, veuillez réessayer
book-confirm-button = Confirmer la réservation

# Shared labels used across the cancel / decline / approve / reschedule / claim flows

common-detail-guest = Invité :
common-detail-reason = Motif :
common-reason-optional = (facultatif)
common-close-page = Vous pouvez fermer cette page.

# Cancel flow (booking_cancel_form.html, booking_cancelled_guest.html)

cancel-page-title = Annuler la réservation
cancel-heading = Annuler la réservation
cancel-subtitle = Vous êtes sur le point d'annuler votre réservation.
cancel-reason-label = Motif
cancel-reason-placeholder-host = Indiquez à l'organisateur la raison...
cancel-button = Annuler la réservation
cancelled-heading = Réservation annulée
cancelled-subtitle = Votre réservation a été annulée et l'organisateur a été informé.

# Decline flow (booking_decline_form.html, booking_declined.html)

decline-page-title = Refuser la réservation
decline-heading = Refuser la réservation
decline-subtitle = Vous êtes sur le point de refuser cette demande de réservation.
decline-reason-placeholder-guest = Indiquez à l'invité la raison...
decline-button = Refuser la réservation
declined-heading = Réservation refusée
declined-subtitle = La réservation a été refusée et l'invité a été informé.

# Approve flow (booking_approve_form.html, booking_approved.html)

approve-page-title = Approuver la réservation
approve-heading = Approuver la réservation
approve-subtitle = Vous êtes sur le point d'approuver cette demande de réservation.
approve-button = Approuver la réservation
approved-heading = Réservation approuvée
approved-subtitle = La réservation a été confirmée et un e-mail de confirmation a été envoyé à { $email }.

# Claim flow (booking_claim_form.html, booking_claimed.html, booking_already_claimed.html)

claim-page-title = Prendre la réservation
claim-heading = Prendre la réservation
claim-subtitle = Vous êtes sur le point de prendre en charge cette réservation. Vous serez ajouté comme participant.
claim-assigned-to = Attribuée à :
claim-button = Prendre cette réservation
claimed-page-title = Réservation prise en charge
claimed-heading = Réservation prise en charge
claimed-subtitle = Vous avez pris en charge cette réservation. Une invitation a été envoyée à votre adresse e-mail.
already-claimed-page-title = Déjà prise en charge
already-claimed-heading = Déjà prise en charge
already-claimed-subtitle = Cette réservation a déjà été prise en charge par { $name }.

# Generic error page (booking_action_error.html)

action-error-page-title = Erreur d'action sur la réservation

# Host-initiated reschedule (booking_host_reschedule.html)

host-resched-page-title = Reprogrammer la réservation — calrs
host-resched-heading = Reprogrammer la réservation
host-resched-subtitle = Cela enverra à { $guest } un e-mail lui demandant de choisir un nouveau créneau.
host-resched-currently = Actuellement :
host-resched-button = Envoyer la demande de reprogrammation
host-resched-cancel-link = Annuler

# Guest reschedule confirmation (booking_reschedule_confirm.html)

resched-confirm-page-title = Confirmer la reprogrammation
resched-confirm-heading = Confirmer la reprogrammation
resched-confirm-subtitle = Vous êtes sur le point de déplacer votre réservation à un nouveau créneau.
resched-was = Avant :
resched-new = Maintenant :
resched-button = Confirmer la reprogrammation
resched-back-to-picker = Retour au choix du créneau

# Base layout chrome (templates/base.html)

base-loader-checking = Vérification des disponibilités
base-loader-please-wait = Veuillez patienter, chargement des dernières données de calendrier...
base-stop-impersonating = Arrêter l'usurpation
base-theme-toggle = Changer de thème
base-powered-by = Propulsé par

# Profile (templates/profile.html)

profile-pick-event-type-invite = Choisissez un type d'événement pour réserver un créneau horaire.
profile-no-event-type = Aucun type d'événement disponible pour le moment.

# Month and weekday names + per-locale date format patterns.

common-month-1 = janvier
common-month-2 = février
common-month-3 = mars
common-month-4 = avril
common-month-5 = mai
common-month-6 = juin
common-month-7 = juillet
common-month-8 = août
common-month-9 = septembre
common-month-10 = octobre
common-month-11 = novembre
common-month-12 = décembre

common-weekday-long-mon = lundi
common-weekday-long-tue = mardi
common-weekday-long-wed = mercredi
common-weekday-long-thu = jeudi
common-weekday-long-fri = vendredi
common-weekday-long-sat = samedi
common-weekday-long-sun = dimanche

# French dates: no comma, day before month, lowercase day/month names.
common-format-month-year = { $month } { $year }
common-format-long-date = { $weekday } { $day } { $month } { $year }

# Email signatures and shared bits (src/email.rs)

email-signature = — calrs
email-action-reschedule = Reprogrammer
email-action-cancel-booking = Annuler la réservation

# Email: guest booking confirmation

email-confirm-subject = { $event } — { $date }
email-confirm-greeting = Bonjour { $name },
email-confirm-headline = Votre réservation est confirmée !
email-confirm-ics-attached-plain = Une invitation est jointe à cet e-mail.
email-confirm-ics-attached-html = Une invitation est jointe à cet e-mail.
email-confirm-need-to-cancel = Besoin d'annuler ? { $url }

# Email: guest reminder

email-reminder-subject = Rappel : { $event } à { $time }
email-reminder-headline = Votre rendez-vous approche.

# Email: guest cancellation

email-cancel-subject = Annulée : { $event } — { $date }
email-cancel-headline-by-host = Votre réservation a été annulée par { $host }.
email-cancel-headline-by-guest = Votre réservation a été annulée.
email-cancel-ics-attached-plain = Une annulation de calendrier est jointe.
email-cancel-ics-attached-html = Une annulation de calendrier est jointe à cet e-mail.

# Dashboard sidebar and shared chrome (templates/dashboard_base.html)

nav-section-scheduling = Planification
nav-overview = Vue d'ensemble
nav-event-types = Types d'événement
nav-bookings = Réservations
nav-teams = Équipes
nav-section-shared-links = Liens partagés
nav-invite-links = Liens d'invitation
nav-section-calendars = Agendas
nav-sources = Sources
nav-section-personal = Personnel
nav-settings = Profil et paramètres
nav-troubleshoot = Diagnostic
nav-section-admin = Administration
nav-admin-panel = Panneau d'administration
nav-sign-out = Se déconnecter
nav-release-notes = Voir les notes de version

# Timezone mismatch banner (templates/dashboard_base.html)

tz-banner-text = Le fuseau horaire de votre navigateur est { $detected } alors que votre fuseau de réservation est { $current }.
tz-banner-update = Mettre à jour
tz-banner-dismiss = Ignorer

# Markdown editor toolbar (templates/dashboard_base.html)

editor-link-prompt = Saisissez l'URL :
editor-link-default-label = texte du lien
editor-placeholder-text = texte
editor-nothing-to-preview = Rien à prévisualiser

# Dashboard overview (templates/dashboard_overview.html)

overview-page-title = Tableau de bord
overview-welcome = Bienvenue, { $name }
overview-public-page = Page publique :
overview-avail-banner-title = Disponibilité par défaut
overview-avail-banner-body = Vos horaires de travail par défaut ont été fixés du lundi au vendredi, de 9h00 à 17h00. Ils sont utilisés lorsque d'autres personnes vous incluent dans une réunion de groupe dynamique.
overview-avail-banner-cta = Vérifier vos disponibilités
overview-dismiss = Ignorer
overview-getting-started = Pour commencer
overview-getting-started-help = Suivez ces étapes pour commencer à accepter des réservations.
overview-step-connect-calendar = Connecter un agenda
overview-step-first-event-type = Créer votre premier type d'événement
overview-step-share-link = Partager votre lien de réservation
overview-pending-approval = En attente d'approbation
overview-booking-with = { $title } avec { $guest }
overview-badge-pending = en attente
overview-guest-booked = Réservé par l'invité :
overview-confirm = Confirmer
overview-decline = Refuser
overview-stat-event-types = Types d'événement
overview-stat-upcoming = Réservations à venir
overview-stat-pending = En attente d'approbation
overview-stat-sources = Sources d'agenda
overview-quick-actions = Créer un type d'événement
overview-action-public-title = Page de réservation publique
overview-action-public-desc = Partagez un lien — n'importe qui peut choisir un créneau et réserver avec vous.
overview-action-team-title = Planification d'équipe
overview-action-team-desc = Répartissez les réservations entre les membres de l'équipe ou trouvez un créneau où tout le monde est libre.
overview-action-team-desc-empty = Créez d'abord une équipe, puis configurez des types d'événement partagés.
overview-action-private-title = Privé, sur invitation
overview-action-private-desc = Générez des liens à usage unique pour des contacts précis. Personne d'autre ne peut réserver.
overview-action-shared-title = Liens d'invitation partagés
overview-action-shared-desc = Tout collègue de l'équipe peut générer des liens de réservation à partager à l'extérieur.
overview-action-reason-calendar = Connectez d'abord un agenda
overview-action-reason-ask-admin = Demandez à un administrateur de créer une équipe
overview-action-reason-team-admin = Nécessite une équipe — créez-en une d'abord
overview-action-reason-team-member = Nécessite une équipe — demandez à un administrateur

# Dashboard bookings (templates/dashboard_bookings.html)

bookings-page-title = Réservations
bookings-pending-approval = En attente d'approbation
bookings-available-to-claim = À prendre en charge
bookings-upcoming = Réservations à venir
bookings-with = { $title } avec { $guest }
bookings-guest-booked = Réservé par l'invité :
bookings-resource = Ressource :
bookings-confirm = Confirmer
bookings-reschedule = Reprogrammer
bookings-decline = Refuser
bookings-claim = Prendre en charge
bookings-badge-awaiting-reschedule = reprogrammation en attente
bookings-cancel = Annuler
bookings-reason-placeholder = Motif (facultatif)
bookings-confirm-cancel = Confirmer l'annulation
bookings-back = Retour
bookings-empty = Aucune réservation à venir pour le moment.<br>Partagez vos { $link } pour que l'on puisse réserver avec vous.
bookings-empty-link-label = liens de types d'événement

# Dashboard teams listing (templates/dashboard_teams.html)

teams-page-title = Équipes
teams-heading = Équipes
teams-new = Nouvelle
teams-badge-public = publique
teams-badge-private = privée
teams-settings = Paramètres
teams-view = Voir
teams-empty = Aucune équipe pour le moment.
teams-empty-admin = { $link } pour collaborer avec votre équipe.
teams-empty-admin-link-label = Créez-en une
teams-empty-member = Les équipes sont créées par les administrateurs. Demandez au vôtre d'en créer une et de vous y ajouter.

# Dashboard invite links (templates/dashboard_internal.html)

invite-links-page-title = Liens d'invitation
invite-links-heading = Liens d'invitation
invite-links-new = Nouvel événement interne
invite-links-help = Générez des liens de réservation à usage unique pour les types d'événement internes. Tout collègue authentifié peut créer et partager des liens ici.
invite-links-duration = { $minutes } min
invite-links-hosted-by = Organisé par { $host }
invite-links-get-link = Obtenir un lien
invite-links-invites = Invitations
invite-links-empty = Aucun type d'événement interne pour le moment.<br>{ $link } avec la visibilité « Interne » pour permettre à tout collègue de générer des liens de réservation.
invite-links-empty-link-label = Créez un type d'événement
invite-links-js-generating = Génération...
invite-links-js-copied = Copié !
invite-links-js-error = Erreur

teams-member-count =
    { $count ->
        [one] { $count } membre
       *[other] { $count } membres
    }

# Dashboard calendar sources (templates/dashboard_sources.html)

sources-page-title = Sources d'agenda
sources-heading = Sources d'agenda
sources-add = Ajouter
sources-last-sync = Dernière synchronisation :
sources-sync = Synchroniser
sources-full-resync = Resynchronisation complète
sources-full-resync-title = Vider le cache et récupérer tous les événements depuis le serveur
sources-test = Tester
sources-reconnect = Reconnecter
sources-reconnect-title = Relancer le processus de consentement Google
sources-edit = Modifier
sources-remove = Supprimer
sources-remove-confirm = Supprimer la source « { $name } » ? Tous les événements synchronisés depuis cette source seront effacés.
sources-no-write-calendar = Aucun agenda d'écriture sélectionné. Les réservations confirmées restent dans calrs et ne sont pas envoyées vers cet agenda. Choisissez-en un ci-dessous pour activer l'écriture.
sources-write-bookings-to = Écrire les réservations dans :
sources-write-none = Aucun (ne pas écrire)
sources-empty = Aucune source d'agenda connectée. { $link } pour vérifier les disponibilités.
sources-empty-link-label = Ajoutez-en une

# Dashboard event types listing (templates/dashboard_event_types.html)

event-types-page-title = Types d'événement
event-types-heading = Types d'événement
event-types-new = Nouveau
event-types-badge-disabled = désactivé
event-types-badge-internal = interne
event-types-badge-private = privé
event-types-badge-resources = ressources
event-types-send-invites = Envoyer des invitations
event-types-duration = { $minutes } min
event-types-mode-collective = collectif
event-types-mode-round-robin = tour de rôle
event-types-edit = Modifier
event-types-disable = Désactiver
event-types-enable = Activer
event-types-embed = Intégrer
event-types-overrides = Exceptions
event-types-team-settings = Paramètres de l'équipe
event-types-invites = Invitations
event-types-view-public = Voir la page publique
event-types-view-page = Voir la page
event-types-delete = Supprimer
event-types-delete-confirm = Supprimer le type d'événement « { $title } » ? Cette action est irréversible.
event-types-empty = Aucun type d'événement pour le moment. { $link } pour commencer à accepter des réservations.
event-types-empty-link-label = Créez-en un

# Markdown editor toolbar (templates/settings.html, templates/team_form.html)

editor-bold = Gras (Ctrl+B)
editor-italic = Italique (Ctrl+I)
editor-strikethrough = Barré
editor-code = Code en ligne
editor-link = Insérer un lien (Ctrl+K)
editor-toggle-preview = Afficher ou masquer l'aperçu
editor-preview = Aperçu

# Profile and settings (templates/settings.html)

settings-page-title = Paramètres
settings-heading = Profil et paramètres
settings-public-page-label = Votre page de réservation publique
settings-copy = Copier
settings-copied = Copié !
settings-open = Ouvrir
settings-avatar = Avatar
settings-upload = Téléverser
settings-remove = Supprimer
settings-display-name = Nom affiché
settings-display-name-placeholder = Votre nom
settings-username = Nom d'utilisateur
settings-username-hint = (utilisé dans votre URL de réservation)
settings-username-pattern-title = Minuscules, chiffres et tirets uniquement
settings-username-help = Votre page de réservation publique :
settings-title = Fonction
settings-title-placeholder = ex. Ingénieure logiciel, Chef de produit
settings-title-help = Affichée sur votre profil public et dans la barre latérale.
settings-bio = Biographie
settings-bio-placeholder = Présentez-vous en quelques mots...
settings-bio-help = Affichée sur votre page de réservation publique. Prend en charge **gras**, *italique*, ~~barré~~, `code` et [liens](url).
settings-booking-email = E-mail de réservation
settings-booking-email-help = Cette adresse apparaîtra sur vos pages de réservation publiques et dans les notifications. Laissez vide pour utiliser votre adresse de connexion.
settings-booking-email-warning = Assurez-vous que cette adresse existe chez votre fournisseur de messagerie. Sinon, les notifications ne seront pas remises.
settings-timezone = Fuseau horaire
settings-timezone-help = Vos règles de disponibilité et vos horaires de réservation sont calculés dans ce fuseau horaire.
settings-language = Langue
settings-language-auto = Auto (langue du navigateur)
settings-language-help = Choisissez une langue d'interface, ou laissez sur Auto pour suivre le réglage de votre navigateur.
settings-dynamic-group = Autoriser les autres à m'inclure dans les liens de groupe dynamiques
settings-dynamic-group-help = Une fois activé, d'autres utilisateurs peuvent créer des URL de réunion collective ponctuelles qui vous incluent (ex. { $example }).
settings-lend-resource = Prêter mon accès agenda pour les réservations de ressources
settings-lend-resource-help = Lorsqu'une réservation doit réserver une ressource partagée (laboratoire de démo, salle de réunion) accessible en écriture par votre compte agenda, autoriser calrs à utiliser vos identifiants enregistrés pour cette écriture.
settings-default-availability = Disponibilité par défaut
settings-default-availability-help = Vos horaires de travail par défaut. Utilisés pour les liens de groupe dynamiques lorsque d'autres vous incluent dans une réunion.
settings-copy-to-all = Copier sur tous les jours
settings-copy-to-all-title = Copier les plages du premier jour activé vers tous les autres jours activés
settings-add-window = Ajouter une plage horaire
settings-remove-window = Supprimer la plage
settings-save = Enregistrer les paramètres
settings-appearance = Apparence
settings-theme-system = Système
settings-theme-light = Clair
settings-theme-dark = Sombre

# Sign in (templates/auth/login.html)

login-page-title = Connexion
login-heading = Connexion
login-subtitle = Connectez-vous à votre compte calrs
login-sso = Se connecter avec le SSO
login-or = ou
login-email = E-mail
login-password = Mot de passe
login-submit = Se connecter par e-mail
login-no-account = Vous n'avez pas de compte ? { $link }
login-register-link = Inscrivez-vous

# Registration (templates/auth/register.html)

register-page-title = Inscription
register-heading = Créer un compte
register-subtitle = Créez un nouveau compte calrs
register-domains-limited = L'inscription est réservée à : { $domains }
register-name = Nom
register-name-placeholder = Votre nom
register-email = E-mail
register-password = Mot de passe
register-password-hint = (12 caractères minimum)
register-submit = Créer le compte
register-have-account = Vous avez déjà un compte ? { $link }
register-signin-link = Connectez-vous

# Authentication errors (src/auth.rs)

auth-error-rate-limited = Trop de tentatives de connexion. Veuillez réessayer plus tard.
auth-error-invalid-credentials = Adresse e-mail ou mot de passe incorrect
auth-error-internal = Erreur interne
auth-error-registration-disabled = Les inscriptions sont désactivées.
auth-error-name-length = Le nom doit contenir entre 1 et 255 caractères
auth-error-email-length = L'adresse e-mail doit contenir entre 1 et 255 caractères
auth-error-email-invalid = Veuillez saisir une adresse e-mail valide
auth-error-email-domain = Domaine de messagerie non autorisé
auth-error-password-length = Le mot de passe doit contenir au moins 12 caractères
auth-error-email-taken = Cette adresse e-mail est déjà utilisée
auth-error-create-failed = Échec de la création du compte

# Calendar source test and write-back setup (templates/source_test.html, templates/source_write_setup.html)

source-test-page-title = Source d'agenda
source-test-sync-heading = Synchronisation : { $name }
source-test-heading = Test de connexion
source-write-page-title = Configurer l'écriture dans l'agenda
source-write-back = Retour au tableau de bord
source-write-heading = Où enregistrer les réservations ?
source-write-help = Lorsqu'une personne réserve une réunion avec vous, calrs peut créer automatiquement l'événement dans votre agenda. Choisissez l'agenda dans lequel écrire les réservations pour { $name }.
source-write-save = Enregistrer
source-write-skip = Passer pour l'instant
source-write-sync-results = Résultats de la synchronisation

source-write-event-count =
    { $count ->
        [one] { $count } événement
       *[other] { $count } événements
    }

# Date overrides (templates/overrides.html)

overrides-page-title = Exceptions de date
overrides-heading = Exceptions de date
overrides-back-teams = Retour aux équipes
overrides-back-event-types = Retour aux types d'événement
overrides-intro = Ajoutez des exceptions à des dates précises pour { $title }
overrides-add-heading = Ajouter une exception
overrides-date = Date
overrides-type = Type d'exception
overrides-type-blocked = Bloquer toute la journée
overrides-type-custom = Horaires personnalisés
overrides-start-time = Heure de début
overrides-end-time = Heure de fin
overrides-add-submit = Ajouter l'exception
overrides-existing = Exceptions existantes
overrides-badge-blocked = bloquée
overrides-badge-custom = horaires personnalisés
overrides-delete = Supprimer
overrides-delete-confirm = Supprimer cette exception ?
overrides-empty = Aucune exception de date pour le moment.<br>Utilisez le formulaire ci-dessus pour bloquer des dates précises (jours fériés, congés) ou définir des horaires personnalisés.

# Public team page (templates/team_profile.html)

team-profile-subtitle = Choisissez un type d'événement pour réserver un créneau.
team-profile-empty = Aucun type d'événement disponible pour le moment.

# Availability troubleshoot (templates/troubleshoot.html, src/web/mod.rs)

troubleshoot-page-title = Diagnostic
troubleshoot-empty = Aucun type d'événement trouvé. { $link } pour commencer à diagnostiquer vos disponibilités.
troubleshoot-empty-link-label = Créez-en un
troubleshoot-subtitle = Comprenez pourquoi les créneaux sont disponibles ou bloqués pour { $title }
troubleshoot-duration = { $minutes } min
troubleshoot-buffer-before = { $minutes } min de marge avant
troubleshoot-buffer-after = { $minutes } min de marge après
troubleshoot-min-notice = { $minutes } min de préavis
troubleshoot-blocked-override = Bloqué par une exception de date (jour de congé)
troubleshoot-custom-hours-active = Exception d'horaires personnalisés active (remplace les règles hebdomadaires)
troubleshoot-legend-available = Disponible
troubleshoot-legend-calendar-event = Événement d'agenda
troubleshoot-legend-booking = Réservation
troubleshoot-legend-resource = Ressource occupée
troubleshoot-legend-outside = Hors horaires
troubleshoot-legend-buffer = Marge / préavis minimum
troubleshoot-blocked-slots = Créneaux bloqués
troubleshoot-none-date-blocked = Cette date est bloquée par une exception de disponibilité (jour de congé). Aucun créneau disponible.
troubleshoot-none-custom-hours = Exception d'horaires personnalisés active, mais aucune plage correspondante. Vérifiez le réglage de l'exception.
troubleshoot-none-no-rules = Aucune règle de disponibilité pour ce jour de la semaine. Ce type d'événement n'est pas réservable le { $date }.
troubleshoot-none-all-bookable = Aucun créneau bloqué pendant les horaires de disponibilité. Tous les horaires sont réservables.
troubleshoot-label-outside = Hors disponibilité
troubleshoot-label-available = Disponible
troubleshoot-label-min-notice = Préavis minimum ({ $minutes } min)
troubleshoot-label-beyond-horizon = Au-delà de l'horizon de réservation ({ $days } jours)
troubleshoot-label-buffer = Marge ({ $minutes } min)
troubleshoot-label-resource-busy = Ressource occupée : { $names }
troubleshoot-detail-around = Autour de : { $label }
troubleshoot-detail-around-booking = Autour de la réservation de { $guest }
troubleshoot-reason-calendar-event = Événement d'agenda : { $label }
troubleshoot-reason-booking = Réservation : { $label }

# Invite management (templates/invite_form.html)

invites-heading = Invitations
invites-back-teams = Retour aux équipes
invites-back-event-types = Retour aux types d'événement
invites-intro = Envoyez des liens d'invitation pour { $title }
invites-capped = <strong>La saisie a été limitée à { $max } destinataires par envoi.</strong> Envoyez le reste dans un autre lot.
invites-failed-hint = — consultez les journaux du serveur pour en savoir plus.
invites-quick-link = Lien rapide
invites-quick-link-help = Générez un lien à usage unique et copiez-le dans votre presse-papiers.
invites-get-link = Obtenir un lien
invites-or-email = Ou envoyer par e-mail
invites-recipients = Destinataires
invites-recipients-hint = (une adresse par ligne, { $max } au maximum)
invites-message = Message personnel
invites-message-hint = (facultatif, envoyé à chaque destinataire)
invites-message-placeholder = Au plaisir de vous présenter une démo...
invites-expires-in = Expire dans
invites-expires-days = { $days } jours
invites-expires-never = Jamais
invites-allow-multiple = Autoriser plusieurs réservations par destinataire
invites-send = Envoyer les invitations
invites-sent-heading = Invitations envoyées
invites-badge-expired = expirée
invites-badge-used = utilisée
invites-badge-active = active
invites-sent-by = Envoyée par { $name }
invites-uses = { $used }/{ $max } utilisations
invites-expires-at = Expire le { $date }
invites-copy-link = Copier le lien
invites-delete = Supprimer
invites-delete-confirm = Supprimer cette invitation ?
invites-empty = Aucune invitation envoyée pour le moment. Utilisez le formulaire ci-dessus pour envoyer un lien de réservation.
invites-js-generating = Génération...
invites-js-copied = Copié !
invites-js-error = Erreur

invites-sent-count =
    { $count ->
        [one] { $count } invitation envoyée.
       *[other] { $count } invitations envoyées.
    }

invites-skipped-invalid =
    { $count ->
        [one] { $count } ligne invalide ignorée :
       *[other] { $count } lignes invalides ignorées :
    }

invites-skipped-duplicate =
    { $count ->
        [one] { $count } ligne en double ignorée :
       *[other] { $count } lignes en double ignorées :
    }

invites-failed =
    { $count ->
        [one] { $count } invitation en échec (BDD ou SMTP) :
       *[other] { $count } invitations en échec (BDD ou SMTP) :
    }

# Calendar source form (templates/source_form.html)

source-form-title-edit = Modifier la source d'agenda
source-form-title-add = Ajouter un agenda
source-form-heading-edit = Modifier la source d'agenda
source-form-heading-add = Connecter un agenda
source-form-subtitle-edit = Mettez à jour la connexion. Laissez le mot de passe vide pour conserver l'actuel. Après avoir changé l'URL ou le nom d'utilisateur, lancez une synchronisation pour rafraîchir la liste des agendas détectés.
source-form-subtitle-add = Connectez un serveur CalDAV ou Microsoft Exchange (EWS) pour que calrs puisse vérifier vos disponibilités lors des réservations.
source-form-backend = Backend
source-form-preset = Préréglage
source-form-connect-google = Se connecter avec Google
source-form-google-unavailable = Google Agenda n'est pas disponible. Contactez votre administrateur.
source-form-name = Nom affiché
source-form-name-placeholder = Mon agenda
source-form-url-caldav = URL CalDAV
source-form-url-ews = URL du point de terminaison EWS
source-form-username = Nom d'utilisateur
source-form-password = Mot de passe
source-form-password-keep = Laissez vide pour conserver l'actuel
source-form-password-placeholder = Mot de passe d'application ou du compte
source-form-skip-test = Ignorer le test de connexion
source-form-skip-test-help = À utiliser si le test se bloque (fréquent sur certaines installations BlueMind/Zimbra). Vous pourrez tester la connexion plus tard.
source-form-save = Enregistrer les modifications
source-form-add = Ajouter la source d'agenda
source-form-help-google-configured = Cliquez sur le bouton ci-dessous pour autoriser calrs à accéder à votre Google Agenda.
source-form-help-google-unconfigured = L'intégration Google Agenda n'est pas encore configurée. Demandez à votre administrateur de renseigner les identifiants OAuth2 Google dans le panneau d'administration.

# Calendar source form: provider help (templates/source_form.html)

source-form-help-bluemind = <strong>BlueMind</strong> — Utilisez le point de terminaison DAV de votre serveur BlueMind.<br> En général : <code>https://mail.yourcompany.com/dav/</code><br> Le nom d'utilisateur est votre <strong>adresse e-mail</strong> (ex. <code>alice@yourcompany.com</code>), pas seulement l'identifiant.<br> Si le test de connexion se bloque, cochez « Ignorer le test de connexion » et lancez directement une synchronisation.
source-form-help-nextcloud = <strong>Nextcloud</strong> — Utilisez la racine WebDAV, pas l'URL d'un agenda précis.<br> En général : <code>https://cloud.example.com/remote.php/dav</code>
source-form-help-fastmail = <strong>Fastmail</strong> — Indiquez votre adresse complète dans le chemin de l'URL.<br> Exemple : <code>https://caldav.fastmail.com/dav/calendars/user/you@fastmail.com/</code><br> Utilisez un mot de passe d'application (Settings &rarr; Privacy &amp; Security &rarr; Integrations).
source-form-help-icloud = <strong>iCloud</strong> — Utilisez <code>https://caldav.icloud.com/</code><br> Un mot de passe d'application est nécessaire, à créer sur <a href="https://appleid.apple.com" target="_blank" style="color: var(--accent);">appleid.apple.com</a> (Sécurité &rarr; Mots de passe pour application).
source-form-help-zimbra = <strong>Zimbra</strong> — Utilisez le point de terminaison DAV de votre serveur Zimbra.<br> En général : <code>https://mail.example.com/dav/</code>
source-form-help-sogo = <strong>SOGo</strong> — Utilisez le point de terminaison DAV de SOGo.<br> En général : <code>https://mail.example.com/SOGo/dav/</code>
source-form-help-radicale = <strong>Radicale</strong> — Utilisez l'URL racine du serveur.<br> En général : <code>https://cal.example.com/</code>
source-form-help-exchange = <strong>Microsoft Exchange (EWS)</strong>. Utilisez le point de terminaison SOAP :<br> <code>https://mail.example.com/EWS/Exchange.asmx</code><br> Le nom d'utilisateur est l'adresse de la boîte aux lettres ; le mot de passe doit accepter l'authentification HTTP Basic sur TLS (à activer sur une boîte de service si votre tenant l'a désactivée).<br> Pensez également à choisir <strong>Microsoft Exchange (EWS)</strong> dans la liste Backend ci-dessus.
source-form-help-google = <strong>Google Agenda</strong> : connexion via OAuth2. Aucun mot de passe requis.<br>
source-form-help-other = Saisissez l'<strong>URL racine DAV</strong> de votre serveur CalDAV, pas celle d'un agenda précis ni un lien public.<br> calrs découvrira automatiquement vos agendas via PROPFIND (RFC 4791).

# Markdown editor toolbar, short labels (templates/team_form.html, templates/team_settings.html)

editor-bold-short = Gras
editor-italic-short = Italique
editor-link-short = Insérer un lien

# Team creation (templates/team_form.html)

team-form-heading = Nouvelle équipe
team-form-name = Nom de l'équipe
team-form-name-placeholder = Ingénierie
team-form-slug = Identifiant
team-form-slug-hint = (identifiant compatible URL)
team-form-slug-pattern-title = Minuscules, chiffres et tirets uniquement
team-form-description = Description
team-form-optional = (facultatif)
team-form-description-placeholder = En quelques mots, le rôle de cette équipe...
team-form-description-help = Affichée sur la page de l'équipe. Prend en charge **gras**, *italique* et [liens](url).
team-form-visibility = Visibilité
team-form-public = Publique
team-form-private = Privée
team-form-visibility-help = Les équipes privées reçoivent un jeton d'invitation à partager. Les équipes publiques apparaissent sur la page de profil d'équipe.
team-form-members = Membres
team-form-members-help = Vous serez ajouté comme administrateur de l'équipe automatiquement. Ajoutez des utilisateurs ou liez des groupes OIDC.
team-form-search-placeholder = Rechercher des utilisateurs ou des groupes...
team-form-search-users = Utilisateurs
team-form-search-groups = Groupes OIDC
team-form-you = (vous)
team-form-submit = Créer l'équipe

# Team settings (templates/team_settings.html)

team-settings-page-title = Paramètres
team-settings-subtitle = Paramètres de l'équipe — les administrateurs de l'équipe peuvent les modifier.
team-settings-public-url = URL publique
team-settings-public-url-help = N'importe qui peut réserver via ce lien.
team-settings-invite-link = Lien d'invitation
team-settings-invite-link-help = Partagez ce lien pour donner accès à la page de réservation de cette équipe privée.
team-settings-avatar = Avatar de l'équipe
team-settings-profile = Profil
team-settings-description-placeholder = Présentez cette équipe...
team-settings-description-help = Affichée sur la page de réservation publique de l'équipe. Prend en charge **gras**, *italique* et [liens](url).
team-settings-visibility-help = Les équipes publiques sont listées sur la page de profil d'équipe. Les équipes privées nécessitent un lien d'invitation.
team-settings-members-help = Gérez les membres de cette équipe. Ajoutez des utilisateurs ou liez des groupes OIDC pour une synchronisation automatique.
team-settings-role-member = Membre
team-settings-role-admin = Administrateur
team-settings-oidc-group = Groupe OIDC
team-settings-remove = Retirer
team-settings-save = Enregistrer les modifications
team-settings-danger-zone = Zone sensible
team-settings-danger-help = Supprimer définitivement cette équipe. Les types d'événement seront dissociés (pas supprimés). Cette action est irréversible.
team-settings-delete = Supprimer cette équipe
team-settings-delete-confirm = Supprimer l'équipe « { $name } » ? Cette action est irréversible.

# Event type form (templates/event_type_form.html)

etf-heading-edit = Modifier le type d'événement
etf-heading-new = Nouveau type d'événement
etf-team = Équipe
etf-team-hint = (facultatif — laissez vide pour un type d'événement personnel)
etf-team-personal = Personnel
etf-scheduling-mode = Mode de planification
etf-mode-round-robin = Tour de rôle — attribuer à un membre disponible
etf-mode-collective = Collectif — tous les membres doivent être disponibles
etf-scheduling-mode-help = Le tour de rôle attribue la réservation à un membre disponible (le moins chargé d'abord). Le mode collectif exige que tous les membres soient libres en même temps.
etf-title = Titre
etf-title-placeholder = Appel de découverte de 30 min
etf-slug = Identifiant
etf-slug-placeholder = généré à partir du titre
etf-description-placeholder = Un court appel de découverte pour parler de...
etf-description-help = Affichée sur la page de réservation. Prend en charge **gras**, *italique* et [liens](url).
etf-location = Lieu
etf-location-link = Visioconférence (URL fixe)
etf-location-jitsi = Jitsi (salon généré automatiquement)
etf-location-webhook = Webhook (fournisseur personnalisé)
etf-location-phone = Téléphone
etf-location-in-person = En personne
etf-location-custom = Personnalisé
etf-location-details = Détails
etf-location-details-placeholder = https://meet.example.com/ma-salle
etf-pattern-placeholder = Laissez vide pour utiliser le motif par défaut de l'organisation
etf-duration = Durée (minutes)
etf-slot-interval = Intervalle entre créneaux (minutes)
etf-slot-interval-placeholder = Identique à la durée
etf-slot-interval-help = Fréquence de démarrage des créneaux. Laissez vide pour suivre la durée.
etf-required-members = Membres requis
etf-required-members-help = Tous les membres cochés doivent être libres pour qu'un créneau soit proposé. Décochez les membres à exclure (leur disponibilité sera ignorée).
etf-member-priority = Priorité des membres
etf-member-priority-help = Les membres les plus prioritaires reçoivent les réservations en premier lorsqu'ils sont disponibles. À priorité égale, la répartition suit le nombre de réservations récentes.
etf-member-timezone-title = Fuseau horaire du membre. Ses horaires de travail personnels sont interprétés dans ce fuseau.
etf-priority-high = Haute
etf-priority-medium = Moyenne
etf-priority-low = Basse
etf-section-availability = Disponibilité
etf-timezone-help = Les horaires ci-dessous sont interprétés dans ce fuseau. Pour un type d'événement d'équipe, choisissez le fuseau de travail de l'équipe (pas forcément celui du créateur).
etf-reset-default = Rétablir mes horaires par défaut
etf-reset-default-title = Remplacer ces horaires par la disponibilité par défaut de votre profil
etf-availability-prefilled = Pré-rempli depuis votre { $link }. Vous pouvez le remplacer ici pour ce type d'événement.
etf-availability-prefilled-link = disponibilité par défaut
etf-section-buffers = Marges et préavis
etf-buffer-before = Marge avant (min)
etf-buffer-after = Marge après (min)
etf-min-notice = Préavis minimum
etf-min-notice-help = Délai minimum entre la réservation et la réunion.
etf-section-limits = Limites de réservation
etf-first-slot-only = Un seul créneau par jour
etf-first-slot-only-help = N'afficher que le premier horaire disponible de chaque journée.
etf-freq-limit = Limiter la fréquence des réservations
etf-freq-limit-help = Limiter le nombre de réservations de cet événement par période.
etf-add-limit = Ajouter une limite
etf-section-options = Options de réservation
etf-requires-confirmation = Confirmation requise
etf-requires-confirmation-help = Les réservations resteront en attente jusqu'à votre validation depuis le tableau de bord.
etf-sms = Notifications SMS
etf-sms-off = Désactivé, aucun numéro demandé
etf-sms-optional = Facultatif, les invités peuvent laisser un numéro
etf-sms-required = Obligatoire, les invités doivent laisser un numéro
etf-sms-help = Envoie un SMS à l'invité lorsque sa réservation est confirmée, déplacée, annulée ou sur le point de commencer, en plus de l'e-mail. Un invité qui laisse le champ vide ne reçoit tout simplement pas de SMS. Nécessite une passerelle SMS dans le { $link }.
etf-admin-panel-link = panneau d'administration
etf-additional-guests = Invités supplémentaires
etf-guests-none = Les invités ne peuvent pas en ajouter d'autres
etf-additional-guests-help = Permettre à la personne qui réserve d'inviter d'autres participants, qui recevront l'invitation d'agenda.
etf-default-view = Vue d'agenda par défaut
etf-view-month = Mois — grille d'agenda avec liste de créneaux
etf-view-week = Semaine — colonnes sur 7 jours avec créneaux
etf-view-column = Colonne — jours listés avec leurs créneaux
etf-view-week-short = semaine
etf-view-column-short = colonne
etf-default-view-help = La vue affichée par défaut aux invités. Ils peuvent en changer à tout moment.
etf-conflict-calendars = Agendas de conflits
etf-conflict-calendars-help = Choisissez les agendas à consulter pour détecter les conflits. Si aucun n'est sélectionné, tous sont utilisés.
etf-no-resources = Aucune ressource partagée configurée pour le moment. Ajoutez-en une (laboratoire de démo, salle de réunion) dans le { $link } pour l'exiger ici.
etf-section-access = Accès et notifications
etf-visibility-public = Public — visible sur votre profil
etf-visibility-internal = Interne — tout collègue peut générer des liens d'invitation
etf-visibility-private = Privé — uniquement sur lien d'invitation
etf-visibility-help = Détermine qui peut voir et réserver ce type d'événement.
etf-vis-internal = Interne
etf-reminder = Rappel de réservation
etf-reminder-none = Aucun rappel
etf-reminder-help = Envoyer un e-mail de rappel à vous et à votre invité avant la réunion.
etf-dynamic-group = Lien de groupe dynamique
etf-dynamic-group-help = Créez un lien de réunion ponctuel qui vérifie vos disponibilités et celles d'autres utilisateurs.
etf-dynamic-group-search = Rechercher un utilisateur à ajouter...
etf-dynamic-group-note = Seuls les utilisateurs qui autorisent les liens de groupe dynamiques sont affichés.
etf-dynamic-group-url = URL du lien de groupe
etf-watcher-teams = Équipes observatrices
etf-watcher-teams-help = Les équipes sélectionnées seront notifiées à chaque réservation. Leurs membres peuvent prendre en charge une réservation pour y participer.
etf-save = Enregistrer les modifications
etf-create = Créer le type d'événement
etf-js-loading = Chargement...
etf-js-no-default = Aucun défaut défini
etf-js-reset-done = Rétabli !
etf-js-error = Erreur
etf-js-remove-limit = Supprimer la limite
etf-period-day = Par jour
etf-period-week = Par semaine
etf-period-month = Par mois
etf-period-year = Par an

# Event type form: runtime summary hints (templates/event_type_form.html)


# %1 and %2 are substituted client-side; the values are only known once a field is edited.

etf-hint-no-days = Aucun jour défini
etf-hint-every-day = Tous les jours
etf-fmt-day-one = %1 jour
etf-fmt-day-other = %1 jours
etf-fmt-hours = %1 h
etf-fmt-minutes = %1 min
etf-hint-buffer-both = %1 min avant, %2 min après
etf-hint-buffer-before = %1 min de marge avant
etf-hint-buffer-after = %1 min de marge après
etf-hint-notice = %1 de préavis
etf-hint-no-buffers = Aucune marge, réservation à tout moment
etf-hint-max = Max %1
etf-hint-period-day = /jour
etf-hint-period-week = /semaine
etf-hint-period-month = /mois
etf-hint-period-year = /an
etf-hint-no-limits = Aucune limite
etf-hint-confirmation-required = Confirmation requise
etf-hint-auto-confirmed = Confirmation automatique
etf-hint-extra-guests-one = jusqu'à %1 invité supplémentaire
etf-hint-extra-guests-other = jusqu'à %1 invités supplémentaires
etf-hint-view = vue %1
etf-hint-reminder = rappel %1 avant
etf-hint-no-reminder = aucun rappel

etf-guests-up-to =
    { $count ->
        [one] Jusqu'à { $count } invité supplémentaire
       *[other] Jusqu'à { $count } invités supplémentaires
    }

etf-reminder-hours =
    { $count ->
        [one] { $count } heure avant
       *[other] { $count } heures avant
    }

etf-reminder-days =
    { $count ->
        [one] { $count } jour avant
       *[other] { $count } jours avant
    }

# Event type form: preset banners and meeting-pattern help (templates/event_type_form.html)
# Literal braces are escaped as {"{"} because Fluent reads a bare { as a placeable.

etf-preset-public = Création d'un type d'événement <strong>public</strong> &mdash; toute personne disposant du lien peut réserver.
etf-preset-private = Création d'un type d'événement <strong>privé</strong> &mdash; seules les personnes que vous invitez peuvent réserver.
etf-preset-internal = Création d'un type d'événement <strong>interne</strong> &mdash; tout collègue peut partager le lien de réservation.
etf-preset-team = Création d'un type d'événement <strong>d'équipe</strong> &mdash; les réservations sont réparties entre les membres.
etf-pattern-hint = Motif personnalisé facultatif. Jetons : <code>{"{"}username{"}"}</code>, <code>{"{"}event{"}"}</code>, <code>{"{"}date{"}"}</code>, <code>{"{"}random{"}"}</code>. Laissez vide pour utiliser le motif par défaut configuré par un administrateur.
etf-pattern-random-warning = Ce motif ne contient pas de jeton <code>{"{"}random{"}"}</code>. Deux réservations de ce type d'événement le même jour partageront le même salon, et le second invité pourra entrer dans la réunion du premier. N'utilisez des salons fixes que si c'est bien l'effet recherché.
etf-webhook-hint = L'URL de réunion propre à chaque réservation est récupérée depuis le webhook configuré par un administrateur sous Administration &rarr; Webhook de réunion. Aucune URL n'est nécessaire ici.
