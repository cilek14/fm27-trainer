//! Traduction française (FM27 Training Assistant).

pub fn translate_static(en: &'static str) -> &'static str {
    match en {
        " weeks" => " semaines",
        " · in use" => " · actif",
        "(100% legitimate via FM's native individual training event — zero memory cheats)" => "(100 % légitime via l'événement d'entraînement individuel natif de FM — aucune triche mémoire)",
        "(Checks GitHub Releases on launch, downloads the latest package and restarts automatically)" => "(Vérifie GitHub Releases au lancement, télécharge la dernière version et redémarre automatiquement)",
        "(moves across)" => "(décalé)",
        "<h2>Beyond training</h2><div class=\"card\"><ul>" => "<h2>Hors entraînement</h2><div class=\"card\"><ul>",
        "<h2>Calendar</h2><div class=\"card scroll\"><table class=\"cal\"><thead><tr><th>week of</th><th>period</th><th>matches</th><th>focus</th><th>schedule</th><th>tactical sessions</th></tr></thead><tbody>" => "<h2>Calendrier</h2><div class=\"card scroll\"><table class=\"cal\"><thead><tr><th>semaine</th><th>période</th><th>matchs</th><th>objectif</th><th>programme</th><th>séances tactiques</th></tr></thead><tbody>",
        "<h2>Priorities</h2><div class=\"card\"><table><tbody>" => "<h2>Priorités</h2><div class=\"card\"><table><tbody>",
        "<h2>Projected development</h2><div class=\"card\">" => "<h2>Progression estimée</h2><div class=\"card\">",
        "<h2>Tactical Effectiveness now</h2>" => "<h2>Efficacité tactique actuelle</h2>",
        "<p class=\"note\" style=\"margin-top:30px\">Session names and limits come from FM27's game data. The rhythm around matches follows Sports Interactive's presets.</p></main></body></html>" => "<p class=\"note\" style=\"margin-top:30px\">Les noms et limites des séances proviennent des données de FM27. Le rythme autour des matchs suit les modèles de Sports Interactive.</p></main></body></html>",
        "<p class=\"note\">Rough estimate (gain per session, decay when an area is not trained). Re-read the Breakdown every 2–4 weeks and regenerate the plan.</p></div>" => "<p class=\"note\">Estimation approximative (gain par séance, baisse sans entraînement). Relisez le bilan toutes les 2 à 4 semaines et régénérez le plan.</p></div>",
        "<p class=\"note\">With the module, FM27 Trainer writes these schedules into your calendar automatically. Without it: create them in FM (Training → Create new schedule).</p>" => "<p class=\"note\">Avec le module, FM27 Training Assistant inscrit ces programmes automatiquement dans votre calendrier. Sinon, créez-les dans FM (Entraînement → Créer un programme).</p>",
        "About" => "À propos",
        "Active in FM" => "Actif dans FM",
        "Activity" => "Journal d'activité",
        "Adds extra conditioning on training days (current week only; automatically skipped on low squad condition or 2+ matches)." => "Ajoute du physique supplémentaire les jours d'entraînement (cette semaine uniquement ; ignoré si condition faible ou 2+ matchs).",
        "Adjust values manually" => "Ajuster les valeurs manuellement",
        "Advice" => "Conseil",
        "Aerial" => "Jeu aérien",
        "Age" => "Âge",
        "Agility and Balance" => "Agilité et équilibre",
        "Allow changes on an untested FM build" => "Autoriser les modifications sur une version FM non testée",
        "Another in-possession role in the same line that the player's attributes suit clearly better." => "Un autre rôle avec ballon sur la même ligne nettement mieux adapté aux attributs du joueur.",
        "Apply" => "Appliquer",
        "Apply all Additional Focus" => "Appliquer tous les objectifs additionnels",
        "Apply recommended intensity" => "Appliquer l'intensité recommandée",
        "Apply training now" => "Appliquer l'entraînement maintenant",
        "Approve" => "Approuver",
        "Approve & Write to FM27" => "Approuver et écrire dans FM27",
        "Auto 'More rest' below condition (%)" => "« Plus de repos » auto sous condition (%)",
        "Auto 'More rest' in weeks with 2+ matches" => "« Plus de repos » auto les semaines à 2+ matchs",
        "Auto (balanced)" => "Auto (équilibré)",
        "Auto development mode above tactical understanding (%)" => "Mode développement auto au-dessus de (%)",
        "Awaiting IDs" => "IDs en attente",
        "Auto-read" => "Lecture auto (OCR)",
        "Auto-read from screen" => "Lecture auto à l'écran",
        "Automatically download and install updates" => "Télécharger et installer automatiquement les mises à jour",
        "Automatically guard intensity as condition changes" => "Protéger automatiquement l'intensité selon la condition",
        "Balanced" => "Équilibré",
        "Ball Control" => "Contrôle de balle",
        "Below squad average in this tactic." => "Sous la moyenne de l'effectif dans cette tactique.",
        "Below squad level; consider rotation in big matches." => "Sous le niveau du groupe ; envisagez une rotation dans les grands matchs.",
        "Best eleven" => "Meilleur onze",
        "Best eleven for this tactic" => "Meilleur onze pour cette tactique",
        "Better in the squad" => "Meilleur choix dans l'effectif",
        "Between matches" => "Entre les matchs",
        "Beyond training" => "Hors entraînement",
        "Bring FM to front before reading" => "Mettre FM au premier plan avant la lecture",
        "Build season plan" => "Créer le plan de saison",
        "Build week plan" => "Créer le plan de la semaine",
        "Building plan…" => "Création du plan…",
        "Buy Me a Coffee" => "Buy Me a Coffee",
        "Buy-In" => "Adhésion",
        "Buying…" => "Achat en cours…",
        "By this calculation your current eleven is already the best." => "Selon ce calcul, votre onze actuel est déjà optimal.",
        "Calendar sync and assistant-override protection appear once your FM27 save is loaded." => "La synchronisation du calendrier et la protection contre l'adjoint s'activent dès que votre partie FM27 est chargée.",
        "Camp:" => "Stage :",
        "Capability" => "Niveau",
        "Capture failed. Allow this app under System Settings → Privacy & Security → Screen Recording." => "Échec de capture. Autorisez l'application dans Réglages Système → Confidentialité et sécurité → Enregistrement de l'écran.",
        "Captured image is empty; is Screen Recording permission enabled?" => "Image capturée vide ; l'autorisation d'enregistrement d'écran est-elle activée ?",
        "Check for updates" => "Vérifier les mises à jour",
        "Checking…" => "Vérification…",
        "Close" => "Fermer",
        "Clear" => "Effacer",
        "Click 'Start FM with module' (Steam must be open)." => "Cliquez sur « Lancer FM avec le module » (Steam doit être ouvert).",
        "Club, squad, transfers and training follow the chosen manager. Read this team's Breakdown again before you let the app plan its training." => "Le club, l'effectif et l'entraînement suivent l'entraîneur sélectionné. Relisez l'efficacité tactique de cette équipe avant de planifier.",
        "Coming weeks" => "Semaines à venir",
        "Condition" => "Condition",
        "Congested week" => "Semaine chargée",
        "Connect to FM27" => "Se connecter à FM27",
        "Connect to FM27 for live training" => "Connectez-vous à FM27 pour l'entraînement en direct",
        "Connect to FM27 for player data" => "Connectez-vous à FM27 pour les données joueurs",
        "Connect to FM27 for tactic analysis" => "Connectez-vous à FM27 pour l'analyse tactique",
        "Connect to FM27 to read each player's attributes." => "Connectez-vous à FM27 pour lire les attributs de chaque joueur.",
        "Connect to Football Manager" => "Se connecter à Football Manager",
        "Connect to an active save in FM27 first to read squad condition, sharpness and age." => "Connectez-vous d'abord à une partie active dans FM27 pour lire condition, rythme et âge.",
        "Connect to an active save in FM27 to preview your tactic's roles per player." => "Connectez-vous à une partie active dans FM27 pour prévisualiser les rôles tactiques par joueur.",
        "Connected to FM27" => "Connecté à FM27",
        "Connected to FM27." => "Connecté à FM27.",
        "Created" => "Créé",
        "Cup" => "Coupe",
        "Current eleven" => "Onze actuel",
        "Day after match" => "Lendemain de match",
        "Day before match" => "Veille de match",
        "Day off" => "Jour de repos",
        "Default calendar" => "Calendrier par défaut",
        "Defensive Positioning" => "Placement défensif",
        "Depends on the players themselves (decisions, anticipation, teamwork). Pick players with higher Tactic Ability or keep instructions simpler." => "Dépend des joueurs (décisions, anticipation, collectif). Choisissez des joueurs plus intelligents tactiquement ou simplifiez les consignes.",
        "Derby / Final prep (1 week)" => "Prépa Derby / Finale (1 sem.)",
        "Development" => "Développement",
        "Double" => "Double",
        "Double Intensity" => "Intensité double",
        "Downloaded" => "Téléchargé",
        "Endurance" => "Endurance",
        "Error" => "Erreur",
        "Europe" => "Europe",
        "Every training day and the day before a match ends with Rest." => "Chaque jour d'entraînement et la veille de match se terminent par Repos.",
        "Everything already follows the plan." => "Tout respecte déjà le plan.",
        "Export & open" => "Exporter et ouvrir",
        "Extra focus on:" => "Priorité sur :",
        "FM's assistant re-plans the next 1–2 weeks every week. FM27 Training Assistant checks every 8 seconds and puts any change back on your plan. Keep the app open while you play." => "L'adjoint de FM modifie les 1–2 prochaines semaines chaque semaine. FM27 Training Assistant vérifie toutes les 8 secondes et remet votre plan. Gardez l'app ouverte en jouant.",
        "FM27 Training Assistant is free and stays free. If you want to give something back for the work, you can donate. Every feature works without it." => "FM27 Training Assistant est gratuit et le restera. Si vous souhaitez soutenir ce projet, vous pouvez offrir un café ; toutes les fonctions restent 100 % gratuites.",
        "FM27 Training Assistant only changes things on FM builds it knows (Steam on Windows and Mac, Game Pass / Xbox app on Windows). On other builds (after an FM update, or Epic for example) the app is read-only. Do club, budget and squad in the app match what FM shows? Then you can allow changes here. Every change is checked first, can be undone and only sticks once you save in FM. Back up your save first." => "FM27 Training Assistant ne modifie que les versions FM reconnues (Steam Windows/Mac, Game Pass). Sur d'autres versions (après une mise à jour ou Epic), il reste en lecture seule. Si le club et l'effectif correspondent à FM, vous pouvez autoriser l'écriture ici. Chaque modification est vérifiée et ne devient permanente qu'après sauvegarde dans FM. Pensez à sauvegarder avant.",
        "FM27 Training Assistant plans your training on your weakest areas and keeps it in FM's calendar." => "FM27 Training Assistant planifie vos entraînements selon vos points faibles et les maintient dans le calendrier de FM.",
        "FM27 is already running. Load your save; the app connects automatically." => "FM27 est déjà lancé. Chargez votre partie ; l'application se connecte automatiquement.",
        "FM27 is already running. Save your game, quit FM27 and try again." => "FM27 est déjà lancé. Sauvegardez, quittez FM27 puis réessayez.",
        "FM27 is starting through Steam. Load your save; the app connects automatically." => "FM27 démarre via Steam. Chargez votre partie ; la connexion est automatique.",
        "FM27 is starting with the module. Load your save; the app connects automatically." => "FM27 démarre avec le module. Chargez votre partie ; la connexion est automatique.",
        "FM27 not connected and no data read yet." => "FM27 non connecté et aucune donnée lue pour l'instant.",
        "FM27 not connected — open the game for live features." => "FM27 non connecté — ouvrez le jeu pour les fonctions en direct.",
        "FM27 screen" => "Écran FM27",
        "FM27 season plan" => "Plan de saison FM27",
        "FM27 week plan" => "Plan hebdomadaire FM27",
        "Fatigue" => "Fatigue",
        "Final Third" => "Dernier tiers",
        "First team" => "Équipe première",
        "Fit" => "Compatibilité",
        "Fit 12+ only" => "Uniq. note 12+",
        "Fitness camp (1 week)" => "Stage physique (1 sem.)",
        "Focus" => "Objectif",
        "Focuses extra slots on player development matching your chosen tactic style." => "Consacre les créneaux libres au développement adapté à votre style tactique.",
        "Folders and permissions" => "Dossiers et autorisations",
        "Forces set pieces and Team Bonding this week for a key derby or final (current week only)." => "Force coups de pied arrêtés et cohésion cette semaine avant un match décisif (semaine actuelle uniquement).",
        "Friday" => "Vendredi",
        "From file" => "Depuis un fichier",
        "Full Week of Rest" => "Semaine complète de repos",
        "Game date" => "Date du jeu",
        "Gegenpress (high press)" => "Gegenpress (pressing haut)",
        "General" => "Général",
        "Give minutes in rotation matches; rises through team tactical sessions." => "Donnez du temps de jeu en rotation ; progresse via les séances tactiques collectives.",
        "Go to Overview" => "Aller à l'aperçu",
        "Goalkeeper Distribution" => "Relance du gardien",
        "Goalkeeper Handling" => "Prise de balle gardien",
        "Goalkeeper Shot Stopping" => "Arrêts du gardien",
        "Grows by scouting the opponent and their players. Scout unknown (European) opponents in advance." => "Augmente en observant l'adversaire et ses joueurs. Supervisez les équipes inconnues à l'avance.",
        "Half" => "Demi",
        "Half Intensity" => "Demi-intensité",
        "Happiness" => "Moral",
        "Have your analysts/scouts report on the next opponent, and hire good analysts." => "Demandez à vos analystes/recruteurs un rapport sur le prochain adversaire.",
        "How well your players fit the roles of your tactic" => "Adéquation de vos joueurs aux rôles de votre tactique",
        "How do I connect?" => "Comment me connecter ?",
        "INJURED" => "BLESSÉ",
        "In Possession" => "Avec ballon",
        "In Possession Role" => "Rôle avec ballon",
        "Individual" => "Individuel",
        "Individual & Bottlenecks" => "Individuel & Points faibles",
        "Injured → half intensity for safe recovery." => "Blessé → demi-intensité pour une récupération sûre.",
        "Injury & Critical Risk" => "Blessures & Risque critique",
        "Injury risk" => "Risque de blessure",
        "Install & restart" => "Installer et redémarrer",
        "Intensity profile:" => "Profil d'intensité :",
        "International break" => "Trêve internationale",
        "Keep" => "Conserver",
        "Keep training on plan automatically" => "Maintenir le plan automatiquement",
        "Language" => "Langue",
        "Last check" => "Dernière vérif.",
        "League on" => "Championnat le",
        "Leave the Breakdown open in FM and the app re-reads it by itself." => "Laissez le détail de l'efficacité tactique ouvert dans FM : l'app le relit automatiquement.",
        "Less rest" => "Moins de repos",
        "Less suited to current role; check Tactics tab for a better-fitting role." => "Rôle actuel peu adapté ; consultez l'onglet Tactique pour un meilleur rôle.",
        "Launch FM27 via Steam, Game Pass or Epic." => "Lancez FM27 via Steam, Game Pass ou Epic.",
        "Launch FM27 with the module via Steam." => "Lancez FM27 avec le module via Steam.",
        "Live connection is unavailable on this system; reading and planning still work." => "La connexion en direct est indisponible sur ce système ; lecture et planification restent actives.",
        "Live features are not available on this system." => "Les fonctions en direct ne sont pas disponibles sur ce système.",
        "Live in FM" => "En direct dans FM",
        "Live in FM is not available on this system" => "En direct dans FM indisponible sur ce système",
        "Live: macOS only" => "En direct : macOS uniquement",
        "Load your save." => "Chargez votre partie.",
        "Load your save" => "Chargez votre partie",
        "Load your save. The app connects automatically." => "Chargez votre partie. La connexion est automatique.",
        "Load your save; the app connects automatically." => "Chargez votre partie ; l'application se connecte automatiquement.",
        "Loading squad attributes…" => "Chargement des attributs de l'effectif…",
        "Looks strictly at each player's own natural position and weakest core attributes — never forces backups into unsuitable tactic roles." => "Analyse uniquement le poste naturel et les attributs clés les plus faibles — n'impose jamais de rôle inadapté aux remplaçants.",
        "Low block & counter" => "Bloc bas et contre",
        "Main Drag" => "Frein principal",
        "Manager of" => "Entraîneur de",
        "Match and travel days stay with FM." => "Les jours de match et de voyage restent gérés par FM.",
        "Match day" => "Jour de match",
        "Match on" => "Match le",
        "Matches" => "Matchs",
        "Measure" => "Mesurer",
        "Menu" => "Menu",
        "More Double intensity for fit & young players with lower rest threshold." => "Plus d'intensité double pour les jeunes joueurs en forme avec seuil de repos réduit.",
        "More rest" => "Plus de repos",
        "Move one Team Bonding to Sunday rest day" => "Déplacer une Cohésion d'équipe au dimanche (repos)",
        "No BepInEx data found. Open Tactical Effectiveness once in FM27." => "Aucune donnée BepInEx trouvée. Ouvrez Efficacité Tactique une fois dans FM27.",
        "No game plugin data found. Open Tactical Effectiveness once in FM27." => "Aucune donnée du module trouvée. Ouvrez Efficacité Tactique une fois dans FM27.",
        "No Football Manager window found. Is FM27 running?" => "Fenêtre Football Manager introuvable. FM27 est-il ouvert ?",
        "No Screen Recording permission. Switch on FM27 Training Assistant under System Settings → Privacy & Security → Screen Recording, then restart the app." => "Autorisation Enregistrement de l'écran manquante. Activez FM27 Training Assistant dans Réglages Système → Confidentialité et sécurité → Enregistrement de l'écran, puis redémarrez.",
        "No changes yet. When FM re-plans a week, you'll see here what was put back." => "Aucun changement pour l'instant. Si FM modifie une semaine, les restaurations apparaîtront ici.",
        "No per-player BepInEx data yet. Open Tactical Effectiveness once in FM27 (with FM27Trainer.Bridge v0.6+ active) to capture individual tactical scores." => "Pas encore de données joueur BepInEx. Ouvrez Efficacité Tactique dans FM27 (avec Bridge actif) pour capturer les scores individuels.",
        "No per-player game plugin data yet. Open Tactical Effectiveness once in FM27 to capture individual tactical scores." => "Pas encore de données joueur du module. Ouvrez Efficacité Tactique une fois dans FM27 pour capturer les scores individuels.",
        "No players below 65 — the entire squad is at a solid tactical level!" => "Aucun joueur sous 65 — tout l'effectif a un niveau tactique solide !",
        "No tactical sessions" => "Aucune séance tactique (Mode développement)",
        "No valid players found to apply." => "Aucun joueur valide trouvé à appliquer.",
        "None: every player is in a role that suits him." => "Aucun : chaque joueur occupe un rôle qui lui convient.",
        "Normal" => "Normale",
        "Normal Intensity" => "Intensité normale",
        "Normal schedule based on season phase, fixtures and weakest areas." => "Programme standard selon la période, les matchs et vos points faibles.",
        "Not connected" => "Non connecté",
        "Not read yet." => "Pas encore lu.",
        "Not right? Attach last-breakdown.png and last-analysis.txt from the app folder (Settings → Open app folder) to your report." => "Un souci ? Joignez last-breakdown.png et last-analysis.txt depuis le dossier de l'app (Paramètres → Ouvrir le dossier) à votre signalement.",
        "OFF" => "ARRÊT",
        "ON" => "ACTIF",
        "OOP Role" => "Rôle sans ballon",
        "Off-season" => "Intersaison",
        "Off: FM overwrites your training with its own plan every week. Turn this on, or click 'Apply training now' every few game weeks." => "Désactivé : FM remplace votre entraînement chaque semaine. Activez cette option ou cliquez sur « Appliquer l'entraînement maintenant » régulièrement.",
        "Once at least two readings from different dates are recorded, a progress line chart will appear here." => "Dès que deux mesures à des dates différentes seront enregistrées, la courbe de progression s'affichera ici.",
        "Only bottlenecks (<65)" => "Uniquement <65",
        "Only days after today are changed." => "Seuls les jours après aujourd'hui sont modifiés.",
        "Open Individual Training once in FM so focus option IDs are captured." => "Ouvrez Entraînement individuel une fois dans FM pour capturer les ID des ateliers.",
        "Open Individual Training once in FM so position/role option IDs are captured." => "Ouvrez Entraînement individuel une fois dans FM pour capturer les ID de poste/rôle.",
        "Open app folder (last screenshot and reading)" => "Ouvrir le dossier de l'app",
        "Open export folder" => "Ouvrir le dossier d'export",
        "Open Tactical Effectiveness" => "Ouvrez Efficacité Tactique",
        "Open Tactical Effectiveness in FM27 and refresh data." => "Ouvrez Efficacité Tactique dans FM27 et actualisez les données.",
        "Open the Breakdown in FM and read it; do this every 2–3 game weeks." => "Ouvrez le bilan dans FM et relisez-le toutes les 2 à 3 semaines de jeu.",
        "Open the Tactical Effectiveness Breakdown in FM27 and read it." => "Ouvrez le détail d'Efficacité Tactique dans FM27 puis lancez la lecture.",
        "Out Of Possession" => "Sans ballon",
        "Out of Possession Role" => "Rôle sans ballon",
        "Outlined = set by FM27 Training Assistant · struck through = what FM wanted ·" => "Encadré = défini par l'Assistant · barré = prévu par FM ·",
        "Overall" => "Global",
        "Overview" => "Aperçu",
        "Partially applied" => "Partiellement appliqué",
        "Passing" => "Passes",
        "Period" => "Période",
        "Place" => "Poste",
        "Plan" => "Plan",
        "Plan ready. Green = will change." => "Plan prêt. Vert = sera modifié.",
        "Planner" => "Planificateur",
        "Planning" => "Planification",
        "Play people in their natural position and role. Use Player View to see who drags the rating down." => "Alignez les joueurs à leur poste et rôle naturels. Utilisez la vue joueurs pour repérer qui fait baisser la note.",
        "Player" => "Joueur",
        "Player Capability" => "Niveau du joueur",
        "Player development" => "Développement des joueurs",
        "Players expect the style of your manager profile. The further your tactic strays from it, the lower this is." => "Les joueurs attendent le style de votre profil d'entraîneur. Plus votre tactique s'en éloigne, plus ce score baisse.",
        "Players struggle to carry out their roles. Pick roles that suit their attributes or strengthen the squad." => "Les joueurs ont du mal dans leurs rôles. Choisissez des rôles adaptés à leurs attributs ou renforcez l'effectif.",
        "Pos" => "Pos",
        "Position" => "Poste",
        "Position / IP Role" => "Poste / Rôle PB",
        "Position Fit" => "Adéquation poste",
        "Possession / Tiki-Taka" => "Possession / Tiki-Taka",
        "Pre-match injury radar, bottleneck report, smart intensity and role-free Additional Focus advisor." => "Radar blessures avant-match, rapport des freins, intensité intelligente et objectif additionnel sans forcer de rôle.",
        "Pre-season (early)" => "Pré-saison (début)",
        "Pre-season (late)" => "Pré-saison (fin)",
        "Pre-season (mid)" => "Pré-saison (milieu)",
        "Preview" => "Aperçu",
        "Preview & Approve" => "Prévisualiser & Approuver",
        "Previews each player's tactical position and In/Out of Possession role first. Nothing is written into FM27 until you review, tick players, and click Approve & Write." => "Prévisualise d'abord le poste tactique et le rôle avec/sans ballon de chaque joueur. Rien n'est écrit dans FM27 avant votre validation et un clic sur Approuver et écrire.",
        "Priorities" => "Priorités",
        "Progress Over Time" => "Progression dans le temps",
        "Protective" => "Protecteur",
        "Quick set" => "Réglage rapide",
        "Quickness" => "Vivacité",
        "Quiet week" => "Semaine calme",
        "REST" => "REPOS",
        "Rate € per £" => "Taux € par £",
        "Re-target every (weeks)" => "Recalculer tous les (semaines)",
        "Read" => "Lire",
        "Read FM screen" => "Lire l'écran FM",
        "Read again" => "Relire",
        "Read all 17 exact factors from FM27 (BepInEx)." => "Lire les 17 facteurs exacts depuis FM27 (BepInEx).",
        "Read all 17 exact factors from game plugin." => "Lire les 17 facteurs exacts depuis le module du jeu.",
        "Read from BepInEx" => "Lire via BepInEx",
        "Read from file" => "Lire depuis un fichier",
        "Read from game plugin" => "Lire depuis le module",
        "Read from screen" => "Lire à l'écran",
        "Read options" => "Options de lecture",
        "Read this week (game time)." => "Lu cette semaine (temps de jeu).",
        "Reading your tactic…" => "Lecture de la tactique…",
        "Reading, planning and export do work." => "La lecture, la planification et l'export fonctionnent.",
        "Reading…" => "Lecture…",
        "Ready" => "Prêt",
        "Reason" => "Raison",
        "Recommended" => "Recommandé",
        "Recommended balance between development and freshness." => "Équilibre recommandé entre progression et fraîcheur.",
        "Reconnect" => "Reconnecter",
        "Recovery" => "Récupération",
        "Refresh squad" => "Actualiser l'effectif",
        "Refresh data" => "Actualiser les données",
        "Regular week" => "Semaine normale",
        "Relationships or support are low; Team Bonding and playing time help." => "Relations ou soutien faibles ; Cohésion d'équipe et temps de jeu aident.",
        "Rest" => "Repos",
        "Rest week: kept from FM" => "Semaine de repos : conservée",
        "Rest weeks (holidays) stay with FM." => "Les semaines de congés restent gérées par FM.",
        "Restored (this session)" => "Restaurés (cette session)",
        "Results, good team talks and positive player interactions raise support." => "Les résultats, les bonnes causeries et les discussions positives augmentent le soutien.",
        "Role Fit" => "Adéquation rôle",
        "Role fit and your best eleven are calculated once your FM27 save is loaded." => "L'adéquation aux rôles et le meilleur onze sont calculés dès que votre partie FM27 est chargée.",
        "Role Suitability" => "Adéquation au rôle",
        "Role in / out of possession" => "Rôle avec / sans ballon",
        "Roles that fit better" => "Rôles mieux adaptés",
        "Roles, positions and attributes come from FM. Which attributes belong to a role is our own estimate, and left or right is derived from the order of the places: treat the score as a guide, not as a number from FM. Nothing is changed in FM." => "Les rôles, postes et attributs proviennent de FM. Considérez ces notes comme un indicateur indicatif. Rien n'est modifié dans FM.",
        "Rotate / rest needed" => "Rotation / repos requis",
        "Saturday" => "Samedi",
        "Save your game and quit FM27." => "Sauvegardez votre partie et quittez FM27.",
        "Saving in FM keeps it in your save." => "Sauvegarder dans FM l'enregistre dans votre partie.",
        "Schedule Team Bonding (Extra-Curricular). Keep the core of the squad together and avoid unrest." => "Planifiez Cohésion d'équipe. Gardez le noyau soudé et évitez les tensions.",
        "Screen Recording permission" => "Autorisation Enregistrement écran",
        "Screenshot (PNG):" => "Capture d'écran (PNG) :",
        "Season" => "Saison",
        "Select all" => "Tout sélectionner",
        "Select a Breakdown screenshot (PNG) below to read from file." => "Sélectionnez ci-dessous une capture (PNG) pour lire depuis un fichier.",
        "Selling…" => "Vente en cours…",
        "Session names, limits and presets come from FM27's own game data. The module only reads and writes after checks, with undo. Single-player only. Font: Inter (SIL OFL). Icons: Phosphor (MIT)." => "Les séances, limites et modèles viennent des données de FM27. Le module vérifie tout avant d'écrire, avec annulation. Solo uniquement. Police : Inter (SIL OFL). Icônes : Phosphor (MIT).",
        "Set pieces every (weeks, 0 = off)" => "Coups de pied arrêtés tous les (sem., 0 = arrêt)",
        "Settings" => "Paramètres",
        "Sharpness" => "Rythme",
        "Shooting" => "Finition / Tirs",
        "Smart Additional Focus (role-free)" => "Objectif additionnel intelligent (sans forcer de rôle)",
        "Smart Injury-Safe Individual Intensity" => "Intensité individuelle anti-blessures",
        "Special camp" => "Stage spécial",
        "Squad Readiness" => "Préparation du groupe",
        "Squad condition, bottlenecks, intensity, tactic roles and Additional Focus appear once FM27 is running." => "Condition, points faibles, intensité, rôles tactiques et objectif additionnel s'affichent dès que FM27 est lancé.",
        "Squad looks fresh" => "L'effectif est frais",
        "Standard (auto)" => "Standard (auto)",
        "Start FM with module" => "Lancer FM avec le module",
        "Start FM27" => "Lancer FM27",
        "Start FM27 on your PC: Steam (or the button below), the Xbox app / Game Pass or Epic." => "Lancez FM27 sur votre PC : Steam (ou le bouton ci-dessous), Xbox / Game Pass ou Epic.",
        "Status" => "Statut",
        "Still not working? Attach last-capture.png and last-analysis.txt from the app folder (Settings → Open app folder) to your report." => "Toujours rien ? Joignez last-capture.png et last-analysis.txt depuis le dossier de l'app à votre rapport.",
        "Strength" => "Force",
        "Stricter injury guard: Half intensity earlier, 30+ veterans protected." => "Protection stricte : demi-intensité plus tôt, vétérans 30+ préservés.",
        "Suggested Focus" => "Objectif suggéré",
        "Sunday" => "Dimanche",
        "Support development" => "Soutenir le développement",
        "Support development (buymeacoffee.com/cilek14)" => "Soutenir le développement (buymeacoffee.com/cilek14)",
        "Support on Buy Me a Coffee" => "Soutenir sur Buy Me a Coffee",
        "Tactic" => "Tactique",
        "Tactic Role Individual Training" => "Entraînement individuel par rôle tactique",
        "Tactic style" => "Style tactique",
        "Tactic style:" => "Style tactique :",
        "Tactical" => "Tactique",
        "Tactical Effectiveness" => "Efficacité Tactique",
        "Tactical Effectiveness has not been read yet." => "Efficacité Tactique n'a pas encore été lue.",
        "Tactical Understanding" => "Compréhension tactique",
        "Tactics" => "Tactiques",
        "Team Bonding below Relationships (%)" => "Cohésion d'équipe sous Relations (%)",
        "The FM window is not visible. Bring FM into view (with the Breakdown open) and read again." => "La fenêtre FM n'est pas visible. Affichez FM à l'écran et relisez.",
        "The app connects automatically." => "L'application se connecte automatiquement.",
        "The app reads FM27 directly while it runs. Nothing to install." => "L'app lit directement FM27 pendant qu'il tourne. Rien à installer.",
        "The app talks to FM27 directly through a small module that starts with FM." => "L'app communique avec FM27 via un petit module lancé avec le jeu.",
        "The day after a match gets a tactical session after Recovery." => "Le lendemain de match reçoit une séance tactique après Récupération.",
        "The regular plan." => "Le plan équilibré standard.",
        "The rhythm around matches follows SI's presets; the focus shifts week by week." => "Le rythme autour des matchs suit les modèles SI ; l'objectif évolue chaque semaine.",
        "The way to spread your available players over the eleven places with the highest total fit. Injured and suspended players are left out." => "Répartit les joueurs disponibles aux onze postes avec la meilleure adéquation totale (hors blessés/suspendus).",
        "This FM build has not been verified with FM27 Trainer yet: read-only. Check that club and squad match FM; you can allow changes in Settings." => "Cette version de FM27 n'a pas encore été vérifiée : mode lecture seule. Si le club et l'effectif correspondent à FM, autorisez l'écriture dans Paramètres.",
        "This FM build is tested." => "Cette version de FM est vérifiée.",
        "This is not the tactic in use: FM keeps your starting eleven for that tactic only. So below you only see the best eleven for this tactic." => "Ce n'est pas la tactique active : FM ne conserve le onze que pour la tactique utilisée. Voici uniquement le meilleur onze théorique.",
        "This is not your own team. The app can only plan its training if FM keeps a calendar of its own for it (you run this team's training). The plan uses the values you read last." => "Ce n'est pas votre équipe première. L'app ne peut planifier que si FM maintient son propre calendrier.",
        "Those slots go to player development; match preparation stays. Meant for use with \"Keep at 100 %\" for tactical understanding (Squad)." => "Ces créneaux passent au développement des joueurs ; la préparation de match reste. Idéal quand la compréhension tactique est élevée.",
        "Thu (EL/ECL)" => "Jeu (C3/C4)",
        "Tip: open Individual Training once in FM27 to capture live focus IDs and enable one-click apply." => "Astuce : ouvrez Entraînement individuel une fois dans FM27 pour activer l'application en un clic.",
        "Tip: open Individual Training once in FM27 so live Position & Role option IDs are captured by BepInEx." => "Astuce : ouvrez Entraînement individuel une fois dans FM27 pour que BepInEx capture les ID de poste et rôle.",
        "Tip: open Individual Training once in FM27 so the game plugin can capture Position & Role IDs." => "Astuce : ouvrez Entraînement individuel une fois dans FM27 pour que le module capture les ID de poste et rôle.",
        "Too many team, player and opposition instructions. Drop the ones that add little and bring them back later." => "Trop de consignes d'équipe, individuelles et adverses. Retirez les moins utiles pour le moment.",
        "Train the session with the same name (Tactical Understanding group). Set your Primary Trained Tactic to the tactic you play." => "Travaillez la séance du même nom (groupe Compréhension tactique). Définissez votre tactique principale sur celle jouée.",
        "Training" => "Entraînement",
        "Training day" => "Jour d'entraînement",
        "Training of" => "Entraînement de",
        "Training-only build: only the training calendar is written; all editor/cheat tools are removed." => "Édition sans triche : seul le calendrier d'entraînement est écrit ; tous les outils d'édition/triche sont retirés.",
        "Training-only: only training schedules and options are written, zero memory cheats." => "Entraînement uniquement : seuls les programmes et réglages sont écrits, aucune triche mémoire.",
        "Tue (CL)" => "Mar (C1)",
        "Understanding" => "Compréhension",
        "Untested FM build: changes allowed." => "Version FM non testée : écriture autorisée.",
        "Untested FM build: read-only." => "Version FM non testée : lecture seule.",
        "Up to date" => "À jour",
        "Version & Updates" => "Version et mises à jour",
        "Veteran Guard" => "Protection vétérans",
        "Weakest Attributes" => "Attributs les plus faibles",
        "Weakest place" => "Poste le plus faible",
        "Wed (CL)" => "Mer (C1)",
        "Week" => "Semaine",
        "Week from" => "Semaine du",
        "Week of" => "Semaine du",
        "Weekly Fixture" => "Calendrier semaine",
        "Weekly and season plan as overview and export (works without the module)." => "Plan hebdomadaire et saisonnier avec aperçu et export (fonctionne sans module).",
        "Weekly schedules" => "Programmes hebdomadaires",
        "When all 8 tactical areas are above this threshold, tactical slots automatically switch to player development (0 = off)." => "Quand les 8 domaines tactiques dépassent ce seuil, les créneaux tactiques basculent vers le développement (0 = arrêt).",
        "Who drags the tactic down?" => "Qui tire la tactique vers le bas ?",
        "Windows has no text recognition (OCR) installed. Add English: Settings → Time & language → Language & region → Add a language → English, with 'Optical character recognition'." => "Aucune reconnaissance de texte (OCR) Windows installée. Ajoutez l'anglais avec OCR dans Paramètres → Heure et langue.",
        "Wing & fast attack" => "Ailes et attaque rapide",
        "Winter break" => "Trêve hivernale",
        "Works once FM runs with the module (see below)." => "Fonctionne dès que FM tourne avec le module.",
        "Writing training into FM…" => "Écriture dans FM…",
        "Your eleven in this tactic" => "Votre onze dans cette tactique",
        "accomplished" => "accompli",
        "automatic" => "automatique",
        "average fit" => "adéquation moy.",
        "awkward" => "maladroit",
        "bans" => "suspensions",
        "competent" => "compétent",
        "component" => "composante",
        "connect" => "connecter",
        "cup" => "coupe",
        "date,day,period,type,session1,session2,extra,schedule\n" => "date,jour,periode,type,seance1,seance2,extra,programme\n",
        "day" => "jour",
        "done" => "terminé",
        "empty" => "vide",
        "en" => "fr",
        "every" => "tous les",
        "excellent" => "excellent",
        "fair" => "correct",
        "good" => "bon",
        "ineffectual" => "inefficace",
        "injured" => "blessé",
        "low" => "faible",
        "manual" => "manuel",
        "match next Monday" => "match lundi prochain",
        "match previous Sunday" => "match dimanche dernier",
        "match(es)" => "match(s)",
        "module not reachable (is FM27 running with the module?)" => "module inaccessible (FM27 tourne-t-il ?)",
        "natural" => "naturel",
        "no eleven picked" => "aucun onze choisi",
        "none" => "aucun",
        "on plan" => "conforme",
        "players" => "joueurs",
        "refresh" => "actualiser",
        "role the app does not know yet" => "rôle encore inconnu",
        "source" => "source",
        "suspended" => "suspendu",
        "template" => "modèle",
        "type" => "type",
        "unconvincing" => "peu convaincant",
        "very low" => "très faible",
        "waiting for the Breakdown in FM" => "en attente des données de FM",
        "weeks" => "semaines",
        "✓ Set" => "✓ Appliqué",
        "✓ Written" => "✓ Écrit",
        _ => en,
    }
}

pub fn translate_dynamic(en: String) -> String {
    if en == "your eleven is already the best" {
        return "votre onze est déjà optimal".into();
    }
    if let Some(n) = en.strip_suffix(" role options captured") {
        return format!("{n} options de rôle capturées");
    }
    if let Some(rest) = en.strip_prefix("Selected: ").and_then(|r| r.strip_suffix(" players")) {
        return format!("Sélectionnés : {rest} joueurs");
    }
    if let Some(n) = en.strip_prefix("Sent approved tactic role training (").and_then(|r| r.strip_suffix(" actions) to FM27…")) {
        return format!("Entraînement par rôle approuvé ({n} actions) envoyé à FM27…");
    }
    if let Some(rest) = en.strip_prefix("Downloading update v") {
        return format!("Téléchargement de la mise à jour v{rest}");
    }
    if let Some(rest) = en.strip_prefix("Applying update v") {
        if let Some(ver) = rest.strip_suffix(", restarting…") {
            return format!("Installation de v{ver}, redémarrage…");
        }
        return format!("Application de v{rest}");
    }
    if let Some(ver) = en.strip_prefix("New version available: v") {
        return format!("Nouvelle version disponible : v{ver}");
    }
    if let Some(ver) = en.strip_prefix("Available: v") {
        return format!("Disponible : v{ver}");
    }
    if let Some(rest) = en.strip_prefix("Downloading v") {
        return format!("Téléchargement v{rest}");
    }
    if let Some(rest) = en.strip_prefix("Installing v") {
        return format!("Installation v{rest}");
    }
    if let Some(rest) = en.strip_prefix("Install v") {
        if let Some(ver) = rest.strip_suffix(" & restart") {
            return format!("Installer v{ver} et redémarrer");
        }
    }
    if let Some(rest) = en.strip_prefix("Last read: ") {
        return format!("Dernière lecture : {rest}");
    }
    if let Some(rest) = en.strip_prefix("Read: ") {
        return format!("Lu : {rest}");
    }
    if let Some(rest) = en.strip_prefix("no change (") {
        return format!("aucun changement ({rest}");
    }
    if let Some(rest) = en.strip_prefix("updated (") {
        return format!("mis à jour ({rest}");
    }
    if let Some(n) = en.strip_prefix("Read ").and_then(|r| r.strip_suffix(" of 17 gauges.")) {
        return format!("{n} indicateurs sur 17 lus.");
    }
    if let Some(w) = en.strip_prefix("Read ").and_then(|r| r.strip_suffix(" weeks ago (game time).")) {
        return format!("Lu il y a {w} semaines (temps du jeu).");
    }
    if let Some(rest) = en.strip_prefix("Please check (colour and fill disagree): ") {
        if let Some(mid) = rest.strip_suffix(". Fix them with 'Adjust values manually'.") {
            return format!("Vérifiez (couleur et remplissage divergent) : {mid}. Corrigez via « Ajuster les valeurs manuellement ».");
        }
    }
    if let Some(n) = en.strip_suffix(" days") {
        return format!("{n} jours");
    }
    if let Some(n) = en.strip_prefix("Under ") {
        return format!("Moins de {n}");
    }
    if let Some(n) = en.strip_prefix("Team ") {
        return format!("Équipe {n}");
    }
    if let Some(n) = en.strip_suffix(" readings") {
        return format!("{n} mesures");
    }
    if let Some(p) = en.strip_prefix("Saved: ") {
        return format!("Enregistré : {p}");
    }
    if let Some(g) = en.strip_prefix('+').and_then(|r| r.strip_suffix(" to gain")) {
        return format!("+{g} à gagner");
    }
    if let Some(avg) = en.strip_prefix("Squad condition ").and_then(|r| r.strip_suffix("% → 'More rest' automatically active.")) {
        return format!("Condition du groupe {avg}% → « Plus de repos » activé automatiquement.");
    }
    if let Some(avg) = en.strip_prefix("Current squad condition: ").and_then(|r| r.strip_suffix("% (0 = off).")) {
        return format!("Condition actuelle du groupe : {avg}% (0 = arrêt).");
    }
    if let Some((w, rest)) = en.split_once(" weeks · ") {
        if let Some(u) = rest.strip_suffix(" distinct weekly schedules") {
            return format!("{w} semaines · {u} programmes distincts");
        }
    }
    if let Some(d) = en.strip_prefix("Week of ") {
        if let Some((date, tail)) = d.split_once(": ") {
            if let Some(n) = tail.strip_suffix(" days put back on plan (FM had re-planned the week).") {
                return format!("Semaine du {date} : {n} jours remis au plan (FM avait modifié la semaine).");
            }
            if let Some(n) = tail.strip_suffix(" days written into FM.") {
                return format!("Semaine du {date} : {n} jours inscrits dans FM.");
            }
        }
        return format!("Semaine du {d}");
    }
    if let Some(n) = en.strip_suffix(" days written into FM.") {
        return format!("{n} jours inscrits dans FM.");
    }
    if let Some(n) = en.strip_suffix(" match(es)") {
        return format!("{n} match(s)");
    }
    if let Some(n) = en.strip_suffix(" days restored") {
        return format!("{n} jours restaurés");
    }
    if let Some(n) = en.strip_suffix(" days differ from your plan") {
        return format!("{n} jours diffèrent de votre plan");
    }
    if let Some(s) = en.strip_prefix("FM had planned: ") {
        return format!("FM avait prévu : {s}");
    }
    // Individual / Radar / Bottlenecks
    if let Some(v) = en.strip_suffix("+ yrs guarded") {
        return format!("{v}+ ans protégés");
    }
    if let Some(v) = en.strip_prefix("Sharpness ") {
        return format!("Rythme {v}");
    }
    if let Some((n, avg)) = en.split_once(" players · avg ") {
        return format!("{n} joueurs · moy. {avg}");
    }
    if let Some(n) = en.strip_suffix(" match(es) this week") {
        return format!("{n} match(s) cette semaine");
    }
    if let Some(n) = en.strip_suffix(" options captured") {
        return format!("{n} ateliers capturés");
    }
    if let Some(n) = en.strip_prefix("Updated individual training for ").and_then(|r| r.strip_suffix(" players in FM27.")) {
        return format!("Entraînement individuel mis à jour pour {n} joueurs dans FM27.");
    }
    if let Some(n) = en.strip_prefix("Sent intensity command for ").and_then(|r| r.strip_suffix(" players to FM27 (BepInEx)…")) {
        return format!("Commande d'intensité envoyée pour {n} joueurs à FM27 (BepInEx)…");
    }
    if let Some(n) = en.strip_prefix("Sent Additional Focus command for ").and_then(|r| r.strip_suffix(" players to FM27…")) {
        return format!("Commande d'objectif additionnel envoyée pour {n} joueurs à FM27…");
    }
    if let Some(mid) = en.strip_prefix("Injured → low-strain technical/tactical focus (").and_then(|r| r.strip_suffix(").")) {
        return format!("Blessé → objectif technique/tactique léger ({mid}).");
    }
    if let Some(mid) = en.strip_prefix("32+ → low-strain focus on ").and_then(|r| r.strip_suffix('.')) {
        return format!("32+ → objectif léger sur {mid}.");
    }
    if let Some(rest) = en.strip_prefix("Lowest core attributes for ") {
        if let Some((pos, tail)) = rest.split_once(": ") {
            if let Some((attrs, avg)) = tail.split_once(" (avg ") {
                let avg = avg.trim_end_matches(").");
                return format!("Attributs clés les plus faibles pour {pos} : {attrs} (moy. {avg}).");
            }
        }
    }
    if let Some(rest) = en.strip_prefix("Condition ") {
        if let Some((pct, thr)) = rest.split_once("% (<") {
            if let Some(thr) = thr.strip_suffix("%) → high injury risk, rest on matchday.") {
                return format!("Condition {pct}% (<{thr}%) → risque de blessure élevé, repos le jour de match.");
            }
            if let Some(thr) = thr.strip_suffix("%) → recovery priority.") {
                return format!("Condition {pct}% (<{thr}%) → priorité récupération.");
            }
        }
        if let Some(pct) = rest.strip_suffix("% · balanced workload.") {
            return format!("Condition {pct}% · charge équilibrée.");
        }
    }
    if let Some(rest) = en.strip_prefix("Veteran (") {
        if let Some((age, pct)) = rest.split_once("y) + congested week/condition ") {
            if let Some(pct) = pct.strip_suffix("% → limit muscle load.") {
                return format!("Vétéran ({age} ans) + semaine chargée/condition {pct}% → limiter la charge.");
            }
        }
        if let Some((age, pct)) = rest.split_once("y) fresh (") {
            if let Some(pct) = pct.strip_suffix("%) → capped at Normal to prevent injuries.") {
                return format!("Vétéran ({age} ans) en forme ({pct}%) → plafonné à Normale par précaution.");
            }
        }
    }
    if let Some((m, pct)) = en.split_once(" matches this week + condition ") {
        if let Some(pct) = pct.strip_suffix("% → prevent overload.") {
            return format!("{m} matchs cette semaine + condition {pct}% → éviter la surcharge.");
        }
    }
    if let Some(m) = en.strip_suffix(" matches this week → Double avoided for injury prevention.") {
        return format!("{m} matchs cette semaine → Double évitée contre les blessures.");
    }
    if let Some(rest) = en.strip_prefix("Rotation player: condition ") {
        if let Some((c, s)) = rest.split_once("%, sharpness ") {
            if let Some(s) = s.strip_suffix("% → build sharpness.") {
                return format!("Joueur de rotation : condition {c}%, rythme {s}% → gagner du rythme.");
            }
        }
    }
    if let Some(rest) = en.strip_prefix("Young prospect (") {
        if let Some((age, c)) = rest.split_once("y) & fresh (") {
            if let Some(c) = c.strip_suffix("%) → max development.") {
                return format!("Jeune espoir ({age} ans) & frais ({c}%) → développement maximal.");
            }
        }
    }
    if let Some(rest) = en.strip_prefix("Fresh (") {
        if let Some((c, s)) = rest.split_once("%), low sharpness (") {
            if let Some(s) = s.strip_suffix("%) → boost sharpness.") {
                return format!("Frais ({c}%), manque de rythme ({s}%) → retrouver le rythme.");
            }
        }
    }
    if let Some(c) = en.strip_prefix("Peak condition (").and_then(|r| r.strip_suffix("%) & light week → safe for Double intensity.")) {
        return format!("Condition optimale ({c}%) & semaine légère → intensité Double sûre.");
    }
    // Live & Vision
    if let Some(n) = en.strip_prefix("Tactical understanding put back to 100% for ").and_then(|r| r.strip_suffix(" players (FM had let it drop).")) {
        return format!("Compréhension tactique remise à 100 % pour {n} joueurs.");
    }
    if let Some(rest) = en.strip_prefix("The capture of the FM window has almost no text (") {
        if let Some(n) = rest.split_once(" lines).").map(|x| x.0) {
            return format!("La capture de la fenêtre FM contient très peu de texte ({n} lignes). Vérifiez que FM est visible (non réduit) ou passez en mode fenêtré / sans bordure.");
        }
    }
    if let Some(rest) = en.strip_prefix("No Breakdown labels found (OCR language ") {
        if let Some(lang) = rest.split_once(").").map(|x| x.0) {
            return format!("Aucun libellé d'Efficacité Tactique trouvé (OCR {lang}). Ouvrez Entraînement → Efficacité Tactique dans FM27 avec le détail affiché.");
        }
    }
    if let Some(rest) = en.strip_prefix("Breakdown screen not recognised (") {
        if let Some(n) = rest.split_once(" of 17 gauges").map(|x| x.0) {
            return format!("Écran non reconnu ({n} indicateurs sur 17 trouvés). Ouvrez le détail de l'Efficacité Tactique dans FM27 et réessayez.");
        }
    }
    en
}
