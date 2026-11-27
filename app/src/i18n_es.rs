//! Traducción al español (FM27 Training Assistant).

pub fn translate_static(en: &'static str) -> &'static str {
    match en {
        " weeks" => " semanas",
        " · in use" => " · en uso",
        "(100% legitimate via FM's native individual training event — zero memory cheats)" => "(100 % legítimo mediante el evento nativo de entrenamiento individual de FM — cero trucos de memoria)",
        "(Checks GitHub Releases on launch, downloads the latest package and restarts automatically)" => "(Comprueba GitHub Releases al iniciar, descarga la última versión y reinicia automáticamente)",
        "(moves across)" => "(se desplaza)",
        "<h2>Beyond training</h2><div class=\"card\"><ul>" => "<h2>Fuera del entrenamiento</h2><div class=\"card\"><ul>",
        "<h2>Calendar</h2><div class=\"card scroll\"><table class=\"cal\"><thead><tr><th>week of</th><th>period</th><th>matches</th><th>focus</th><th>schedule</th><th>tactical sessions</th></tr></thead><tbody>" => "<h2>Calendario</h2><div class=\"card scroll\"><table class=\"cal\"><thead><tr><th>semana</th><th>periodo</th><th>partidos</th><th>enfoque</th><th>plan</th><th>sesiones tácticas</th></tr></thead><tbody>",
        "<h2>Priorities</h2><div class=\"card\"><table><tbody>" => "<h2>Prioridades</h2><div class=\"card\"><table><tbody>",
        "<h2>Projected development</h2><div class=\"card\">" => "<h2>Progreso estimado</h2><div class=\"card\">",
        "<h2>Tactical Effectiveness now</h2>" => "<h2>Eficacia táctica actual</h2>",
        "<p class=\"note\" style=\"margin-top:30px\">Session names and limits come from FM27's game data. The rhythm around matches follows Sports Interactive's presets.</p></main></body></html>" => "<p class=\"note\" style=\"margin-top:30px\">Los nombres y límites de las sesiones provienen de los datos de FM27. El ritmo en torno a los partidos sigue las plantillas de Sports Interactive.</p></main></body></html>",
        "<p class=\"note\">Rough estimate (gain per session, decay when an area is not trained). Re-read the Breakdown every 2–4 weeks and regenerate the plan.</p></div>" => "<p class=\"note\">Estimación aproximada (ganancia por sesión, descenso sin entrenar). Vuelve a leer el desglose cada 2–4 semanas y regenera el plan.</p></div>",
        "<p class=\"note\">With the module, FM27 Trainer writes these schedules into your calendar automatically. Without it: create them in FM (Training → Create new schedule).</p>" => "<p class=\"note\">Con el módulo activo, FM27 Training Assistant escribe estos planes automáticamente en tu calendario. Sin él, créalos en FM (Entrenamiento → Crear plan).</p>",
        "About" => "Acerca de",
        "Active in FM" => "Activo en FM",
        "Activity" => "Registro de actividad",
        "Adds extra conditioning on training days (current week only; automatically skipped on low squad condition or 2+ matches)." => "Añade físico extra en los días de entrenamiento (solo esta semana; se omite si hay baja forma o 2+ partidos).",
        "Adjust values manually" => "Ajustar valores manualmente",
        "Advice" => "Consejo",
        "Aerial" => "Juego aéreo",
        "Age" => "Edad",
        "Agility and Balance" => "Agilidad y equilibrio",
        "Allow changes on an untested FM build" => "Permitir cambios en versión de FM no probada",
        "Another in-possession role in the same line that the player's attributes suit clearly better." => "Otro rol con balón en la misma línea que se adapta claramente mejor a los atributos del jugador.",
        "Apply" => "Aplicar",
        "Apply all Additional Focus" => "Aplicar todos los Enfoques Adicionales",
        "Apply recommended intensity" => "Aplicar intensidad recomendada",
        "Apply training now" => "Aplicar entrenamiento ahora",
        "Approve" => "Aprobar",
        "Approve & Write to FM27" => "Aprobar y escribir en FM27",
        "Auto 'More rest' below condition (%)" => "« Más descanso » auto bajo condición (%)",
        "Auto 'More rest' in weeks with 2+ matches" => "« Más descanso » auto en semanas con 2+ partidos",
        "Auto (balanced)" => "Auto (equilibrado)",
        "Auto development mode above tactical understanding (%)" => "Modo desarrollo auto por encima de (%)",
        "Awaiting IDs" => "Esperando IDs",
        "Auto-read" => "Lectura auto (OCR)",
        "Auto-read from screen" => "Lectura automática de pantalla",
        "Automatically download and install updates" => "Descargar e instalar actualizaciones automáticamente",
        "Automatically guard intensity as condition changes" => "Proteger automáticamente la intensidad según la condición",
        "Balanced" => "Equilibrado",
        "Ball Control" => "Control de balón",
        "Below squad average in this tactic." => "Por debajo de la media del equipo en esta táctica.",
        "Below squad level; consider rotation in big matches." => "Por debajo del nivel de la plantilla; valora rotar en partidos clave.",
        "Best eleven" => "Mejor once",
        "Best eleven for this tactic" => "Mejor once para esta táctica",
        "Better in the squad" => "Mejor opción en plantilla",
        "Between matches" => "Entre partidos",
        "Beyond training" => "Fuera del entrenamiento",
        "Bring FM to front before reading" => "Traer FM al frente antes de leer",
        "Build season plan" => "Crear plan de temporada",
        "Build week plan" => "Crear plan semanal",
        "Building plan…" => "Creando plan…",
        "Buy Me a Coffee" => "Buy Me a Coffee",
        "Buy-In" => "Compromiso",
        "Buying…" => "Comprando…",
        "By this calculation your current eleven is already the best." => "Según este cálculo, tu once actual ya es el mejor.",
        "Camp:" => "Concentración:",
        "Capability" => "Capacidad",
        "Capture failed. Allow this app under System Settings → Privacy & Security → Screen Recording." => "Error al capturar. Permite la app en Ajustes del Sistema → Privacidad y seguridad → Grabación de pantalla.",
        "Captured image is empty; is Screen Recording permission enabled?" => "La imagen capturada está vacía; ¿está activo el permiso de Grabación de pantalla?",
        "Check for updates" => "Buscar actualizaciones",
        "Checking…" => "Comprobando…",
        "Clear" => "Limpiar",
        "Click 'Start FM with module' (Steam must be open)." => "Pulsa « Iniciar FM con módulo » (Steam debe estar abierto).",
        "Club, squad, transfers and training follow the chosen manager. Read this team's Breakdown again before you let the app plan its training." => "Club, plantilla y entrenamiento siguen al mánager elegido. Vuelve a leer los datos de este equipo antes de planificar.",
        "Coming weeks" => "Próximas semanas",
        "Condition" => "Condición",
        "Congested week" => "Semana cargada",
        "Connect to FM27" => "Conectar con FM27",
        "Connect to FM27 to read each player's attributes." => "Conecta con FM27 para leer los atributos de cada jugador.",
        "Connect to Football Manager" => "Conectar con Football Manager",
        "Connect to an active save in FM27 first to read squad condition, sharpness and age." => "Conecta primero con una partida activa en FM27 para leer condición, ritmo y edad.",
        "Connect to an active save in FM27 to preview your tactic's roles per player." => "Conecta con una partida activa en FM27 para previsualizar los roles tácticos por jugador.",
        "Connected to FM27" => "Conectado a FM27",
        "Connected to FM27." => "Conectado a FM27.",
        "Created" => "Creado",
        "Cup" => "Copa",
        "Current eleven" => "Once actual",
        "Day after match" => "Día post-partido",
        "Day before match" => "Víspera de partido",
        "Day off" => "Día libre",
        "Default calendar" => "Calendario predeterminado",
        "Defensive Positioning" => "Colocación defensiva",
        "Depends on the players themselves (decisions, anticipation, teamwork). Pick players with higher Tactic Ability or keep instructions simpler." => "Depende de los jugadores (decisiones, anticipación, trabajo en equipo). Elige jugadores con mayor capacidad táctica o simplifica instrucciones.",
        "Derby / Final prep (1 week)" => "Prep. Derbi / Final (1 sem.)",
        "Development" => "Desarrollo",
        "Double" => "Doble",
        "Double Intensity" => "Doble intensidad",
        "Downloaded" => "Descargado",
        "Endurance" => "Resistencia",
        "Error" => "Error",
        "Europe" => "Europa",
        "Every training day and the day before a match ends with Rest." => "Cada día de entrenamiento y la víspera de partido terminan con Descanso.",
        "Everything already follows the plan." => "Todo sigue ya el plan.",
        "Export & open" => "Exportar y abrir",
        "Extra focus on:" => "Enfoque extra en:",
        "FM's assistant re-plans the next 1–2 weeks every week. FM27 Training Assistant checks every 8 seconds and puts any change back on your plan. Keep the app open while you play." => "El segundo entrenador de FM replanifica las próximas 1–2 semanas cada semana. FM27 Training Assistant revisa cada 8 segundos y restaura tu plan. Mantén la app abierta mientras juegas.",
        "FM27 Training Assistant is free and stays free. If you want to give something back for the work, you can donate. Every feature works without it." => "FM27 Training Assistant es gratis y seguirá siéndolo. Si quieres apoyar el proyecto puedes invitar a un café; todas las funciones son gratuitas.",
        "FM27 Training Assistant only changes things on FM builds it knows (Steam on Windows and Mac, Game Pass / Xbox app on Windows). On other builds (after an FM update, or Epic for example) the app is read-only. Do club, budget and squad in the app match what FM shows? Then you can allow changes here. Every change is checked first, can be undone and only sticks once you save in FM. Back up your save first." => "FM27 Training Assistant solo modifica versiones de FM conocidas (Steam Windows/Mac, Game Pass). En otras versiones (tras actualizaciones o Epic) queda en solo lectura. Si club y plantilla coinciden con FM, puedes permitir cambios aquí. Cada cambio se valida y solo se guarda al guardar en FM. Haz copia de seguridad antes.",
        "FM27 Training Assistant plans your training on your weakest areas and keeps it in FM's calendar." => "FM27 Training Assistant planifica tu entrenamiento según tus puntos débiles y lo mantiene en el calendario de FM.",
        "FM27 is already running. Load your save; the app connects automatically." => "FM27 ya está abierto. Carga tu partida; la app se conecta automáticamente.",
        "FM27 is already running. Save your game, quit FM27 and try again." => "FM27 ya está abierto. Guarda la partida, cierra FM27 e inténtalo de nuevo.",
        "FM27 is starting through Steam. Load your save; the app connects automatically." => "FM27 se está iniciando por Steam. Carga tu partida; la conexión es automática.",
        "FM27 is starting with the module. Load your save; the app connects automatically." => "FM27 se inicia con el módulo. Carga tu partida; la conexión es automática.",
        "FM27 screen" => "Pantalla de FM27",
        "FM27 season plan" => "Plan de temporada FM27",
        "FM27 week plan" => "Plan semanal FM27",
        "Fatigue" => "Cansancio",
        "Final Third" => "Último tercio",
        "First team" => "Primer equipo",
        "Fit" => "Encaje",
        "Fit 12+ only" => "Solo nota 12+",
        "Fitness camp (1 week)" => "Concentración física (1 sem.)",
        "Focus" => "Enfoque",
        "Focuses extra slots on player development matching your chosen tactic style." => "Dedica los huecos extra al desarrollo de jugadores acorde a tu estilo táctico.",
        "Folders and permissions" => "Carpetas y permisos",
        "Forces set pieces and Team Bonding this week for a key derby or final (current week only)." => "Fuerza balón parado y cohesión de equipo esta semana para un derbi o final (solo semana actual).",
        "Friday" => "Viernes",
        "From file" => "Desde archivo",
        "Full Week of Rest" => "Semana completa de descanso",
        "Game date" => "Fecha del juego",
        "Gegenpress (high press)" => "Gegenpress (presión alta)",
        "General" => "General",
        "Give minutes in rotation matches; rises through team tactical sessions." => "Dale minutos en rotaciones; sube con sesiones tácticas del equipo.",
        "Go to Overview" => "Ir al Resumen",
        "Goalkeeper Distribution" => "Distribución del portero",
        "Goalkeeper Handling" => "Blocaje del portero",
        "Goalkeeper Shot Stopping" => "Paradas del portero",
        "Grows by scouting the opponent and their players. Scout unknown (European) opponents in advance." => "Crece ojeando al rival y a sus jugadores. Ojea rivales desconocidos con antelación.",
        "Half" => "Mitad",
        "Half Intensity" => "Media intensidad",
        "Happiness" => "Moral",
        "Have your analysts/scouts report on the next opponent, and hire good analysts." => "Pide a tus analistas/ojeadores un informe del próximo rival.",
        "How well your players fit the roles of your tactic" => "Qué tan bien encajan tus jugadores en los roles de tu táctica",
        "INJURED" => "LESIONADO",
        "In Possession" => "Con balón",
        "In Possession Role" => "Rol con balón",
        "Individual" => "Individual",
        "Individual & Bottlenecks" => "Individual y Carencias",
        "Injured → half intensity for safe recovery." => "Lesionado → media intensidad para recuperación segura.",
        "Injury & Critical Risk" => "Lesiones y Riesgo crítico",
        "Injury risk" => "Riesgo de lesión",
        "Install & restart" => "Instalar y reiniciar",
        "Intensity profile:" => "Perfil de intensidad:",
        "International break" => "Parón internacional",
        "Keep" => "Mantener",
        "Keep training on plan automatically" => "Mantener el plan automáticamente",
        "Language" => "Idioma",
        "Last check" => "Última rev.",
        "League on" => "Liga el",
        "Leave the Breakdown open in FM and the app re-reads it by itself." => "Deja abierta la pantalla de Eficacia Táctica en FM y la app la releerá sola.",
        "Less rest" => "Menos descanso",
        "Less suited to current role; check Tactics tab for a better-fitting role." => "Menos apto para el rol actual; revisa la pestaña Tácticas para un rol mejor.",
        "Live features are not available on this system." => "Las funciones en vivo no están disponibles en este sistema.",
        "Live in FM" => "En vivo en FM",
        "Live in FM is not available on this system" => "En vivo en FM no disponible en este sistema",
        "Live: macOS only" => "En vivo: solo macOS",
        "Load your save." => "Carga tu partida.",
        "Load your save. The app connects automatically." => "Carga tu partida. La app se conecta automáticamente.",
        "Loading squad attributes…" => "Cargando atributos de la plantilla…",
        "Looks strictly at each player's own natural position and weakest core attributes — never forces backups into unsuitable tactic roles." => "Analiza solo la posición natural y los atributos clave más bajos de cada jugador — nunca fuerza a suplentes a entrenar roles incompatibles.",
        "Low block & counter" => "Bloque bajo y contra",
        "Main Drag" => "Freno principal",
        "Manager of" => "Mánager de",
        "Match and travel days stay with FM." => "Los días de partido y viaje siguen en manos de FM.",
        "Match day" => "Día de partido",
        "Match on" => "Partido el",
        "Matches" => "Partidos",
        "Measure" => "Medir",
        "Menu" => "Menú",
        "More Double intensity for fit & young players with lower rest threshold." => "Más Doble intensidad para jóvenes en forma con menor umbral de descanso.",
        "More rest" => "Más descanso",
        "Move one Team Bonding to Sunday rest day" => "Mover una Cohesión de equipo al domingo de descanso",
        "No BepInEx data found. Open Tactical Effectiveness once in FM27." => "No hay datos BepInEx. Abre Eficacia Táctica una vez en FM27.",
        "No game plugin data found. Open Tactical Effectiveness once in FM27." => "No hay datos del módulo. Abre Eficacia Táctica una vez en FM27.",
        "No Football Manager window found. Is FM27 running?" => "No se encontró ventana de Football Manager. ¿Está FM27 abierto?",
        "No Screen Recording permission. Switch on FM27 Training Assistant under System Settings → Privacy & Security → Screen Recording, then restart the app." => "Sin permiso de Grabación de pantalla. Activa FM27 Training Assistant en Ajustes del Sistema → Privacidad y seguridad → Grabación de pantalla y reinicia.",
        "No changes yet. When FM re-plans a week, you'll see here what was put back." => "Sin cambios aún. Cuando FM altere una semana, aquí verás lo restaurado.",
        "No per-player BepInEx data yet. Open Tactical Effectiveness once in FM27 (with FM27Trainer.Bridge v0.6+ active) to capture individual tactical scores." => "Aún no hay datos por jugador. Abre Eficacia Táctica en FM27 (con Bridge activo) para capturar las puntuaciones.",
        "No per-player game plugin data yet. Open Tactical Effectiveness once in FM27 to capture individual tactical scores." => "Aún no hay datos por jugador del módulo. Abre Eficacia Táctica una vez en FM27 para capturar las puntuaciones.",
        "No players below 65 — the entire squad is at a solid tactical level!" => "Sin jugadores por debajo de 65 — ¡toda la plantilla tiene un nivel sólido!",
        "No tactical sessions" => "Sin sesiones tácticas (Modo desarrollo)",
        "No valid players found to apply." => "No se encontraron jugadores válidos para aplicar.",
        "None: every player is in a role that suits him." => "Ninguno: cada jugador está en un rol adecuado.",
        "Normal" => "Normal",
        "Normal Intensity" => "Intensidad normal",
        "Normal schedule based on season phase, fixtures and weakest areas." => "Plan estándar según la fase de temporada, partidos y áreas débiles.",
        "Not connected" => "Sin conexión",
        "Not read yet." => "Aún sin leer.",
        "Not right? Attach last-breakdown.png and last-analysis.txt from the app folder (Settings → Open app folder) to your report." => "¿Algo falla? Adjunta last-breakdown.png y last-analysis.txt de la carpeta de la app (Ajustes → Abrir carpeta).",
        "OFF" => "APAGADO",
        "ON" => "ACTIVO",
        "OOP Role" => "Rol sin balón",
        "Off-season" => "Fuera de temporada",
        "Off: FM overwrites your training with its own plan every week. Turn this on, or click 'Apply training now' every few game weeks." => "Apagado: FM sobrescribe tu entrenamiento cada semana. Actívalo o pulsa « Aplicar entrenamiento ahora » cada pocas semanas.",
        "Once at least two readings from different dates are recorded, a progress line chart will appear here." => "Cuando se registren al menos dos lecturas en fechas distintas, aquí aparecerá el gráfico de evolución.",
        "Only bottlenecks (<65)" => "Solo bajos (<65)",
        "Only days after today are changed." => "Solo se modifican los días posteriores a hoy.",
        "Open Individual Training once in FM so focus option IDs are captured." => "Abre Entrenamiento Individual una vez en FM para capturar los IDs de enfoque.",
        "Open Individual Training once in FM so position/role option IDs are captured." => "Abre Entrenamiento Individual una vez en FM para capturar los IDs de posición/rol.",
        "Open app folder (last screenshot and reading)" => "Abrir carpeta de la app",
        "Open export folder" => "Abrir carpeta de exportación",
        "Open the Breakdown in FM and read it; do this every 2–3 game weeks." => "Abre Eficacia Táctica en FM y léelo cada 2–3 semanas de juego.",
        "Open the Tactical Effectiveness Breakdown in FM27 and read it." => "Abre el desglose de Eficacia Táctica en FM27 y pulsa leer.",
        "Out Of Possession" => "Sin balón",
        "Out of Possession Role" => "Rol sin balón",
        "Outlined = set by FM27 Training Assistant · struck through = what FM wanted ·" => "Recuadro = fijado por el Asistente · tachado = plan de FM ·",
        "Overall" => "General",
        "Overview" => "Resumen",
        "Partially applied" => "Parcialmente aplicado",
        "Passing" => "Pases",
        "Period" => "Periodo",
        "Place" => "Posición",
        "Plan" => "Plan",
        "Plan ready. Green = will change." => "Plan listo. Verde = cambiará.",
        "Planner" => "Planificador",
        "Planning" => "Planificación",
        "Play people in their natural position and role. Use Player View to see who drags the rating down." => "Alinea a los jugadores en su posición y rol natural. Usa la vista individual para ver quién baja la media.",
        "Player" => "Jugador",
        "Player Capability" => "Capacidad del jugador",
        "Player development" => "Desarrollo de jugadores",
        "Players expect the style of your manager profile. The further your tactic strays from it, the lower this is." => "Los jugadores esperan el estilo de tu perfil de mánager. Cuanto más se aleje tu táctica, menor será.",
        "Players struggle to carry out their roles. Pick roles that suit their attributes or strengthen the squad." => "Los jugadores tienen dificultades en sus roles. Elige roles acordes a sus atributos o refuerza la plantilla.",
        "Pos" => "Pos",
        "Position" => "Posición",
        "Position / IP Role" => "Posición / Rol CB",
        "Position Fit" => "Encaje posición",
        "Possession / Tiki-Taka" => "Posesión / Tiki-Taka",
        "Pre-match injury radar, bottleneck report, smart intensity and role-free Additional Focus advisor." => "Radar de lesiones prepartido, informe de carencias, intensidad inteligente y enfoque adicional sin forzar roles.",
        "Pre-season (early)" => "Pretemporada (inicio)",
        "Pre-season (late)" => "Pretemporada (final)",
        "Pre-season (mid)" => "Pretemporada (media)",
        "Preview" => "Vista previa",
        "Preview & Approve" => "Vista previa y Aprobar",
        "Previews each player's tactical position and In/Out of Possession role first. Nothing is written into FM27 until you review, tick players, and click Approve & Write." => "Previsualiza primero la posición táctica y el rol con/sin balón de cada jugador. No se escribe nada en FM27 hasta que marques los jugadores y pulses Aprobar y escribir.",
        "Priorities" => "Prioridades",
        "Progress Over Time" => "Progreso en el tiempo",
        "Protective" => "Protector",
        "Quick set" => "Ajuste rápido",
        "Quickness" => "Velocidad",
        "Quiet week" => "Semana tranquila",
        "REST" => "DESCANSO",
        "Rate € per £" => "Tasa € por £",
        "Re-target every (weeks)" => "Recalcular objetivos cada (sem.)",
        "Read" => "Leer",
        "Read FM screen" => "Leer pantalla FM",
        "Read again" => "Leer de nuevo",
        "Read all 17 exact factors from FM27 (BepInEx)." => "Lee los 17 factores exactos desde FM27 (BepInEx).",
        "Read all 17 exact factors from game plugin." => "Lee los 17 factores exactos desde el módulo del juego.",
        "Read from BepInEx" => "Leer desde BepInEx",
        "Read from file" => "Leer desde archivo",
        "Read from game plugin" => "Leer desde módulo",
        "Read from screen" => "Leer desde pantalla",
        "Read options" => "Opciones de lectura",
        "Read this week (game time)." => "Leído esta semana (tiempo del juego).",
        "Reading your tactic…" => "Leyendo tu táctica…",
        "Reading, planning and export do work." => "La lectura, planificación y exportación funcionan.",
        "Reading…" => "Leyendo…",
        "Ready" => "Listo",
        "Reason" => "Motivo",
        "Recommended" => "Recomendado",
        "Recommended balance between development and freshness." => "Equilibrio recomendado entre desarrollo y frescura.",
        "Reconnect" => "Reconectar",
        "Recovery" => "Recuperación",
        "Refresh squad" => "Actualizar plantilla",
        "Refresh data" => "Actualizar datos",
        "Regular week" => "Semana normal",
        "Relationships or support are low; Team Bonding and playing time help." => "Relaciones o respaldo bajos; Cohesión de equipo y minutos ayudan.",
        "Rest" => "Descanso",
        "Rest week: kept from FM" => "Semana de descanso: conservada",
        "Rest weeks (holidays) stay with FM." => "Las semanas de vacaciones las gestiona FM.",
        "Restored (this session)" => "Restaurados (esta sesión)",
        "Results, good team talks and positive player interactions raise support." => "Resultados, buenas charlas e interacciones positivas aumentan el apoyo.",
        "Role Fit" => "Encaje rol",
        "Role Suitability" => "Idoneidad en el rol",
        "Role in / out of possession" => "Rol con / sin balón",
        "Roles that fit better" => "Roles que mejor encajan",
        "Roles, positions and attributes come from FM. Which attributes belong to a role is our own estimate, and left or right is derived from the order of the places: treat the score as a guide, not as a number from FM. Nothing is changed in FM." => "Los roles, posiciones y atributos vienen de FM. Toma las puntuaciones como guía orientativa. No se modifica nada en FM.",
        "Rotate / rest needed" => "Necesita rotación / descanso",
        "Saturday" => "Sábado",
        "Save your game and quit FM27." => "Guarda tu partida y sal de FM27.",
        "Saving in FM keeps it in your save." => "Al guardar en FM se conservará en tu partida.",
        "Schedule Team Bonding (Extra-Curricular). Keep the core of the squad together and avoid unrest." => "Programa Cohesión de equipo. Mantén unido el vestuario y evita malestar.",
        "Screen Recording permission" => "Permiso de Grabación de pantalla",
        "Screenshot (PNG):" => "Captura (PNG):",
        "Season" => "Temporada",
        "Select all" => "Seleccionar todo",
        "Select a Breakdown screenshot (PNG) below to read from file." => "Selecciona abajo una captura (PNG) para leer desde archivo.",
        "Selling…" => "Vendiendo…",
        "Session names, limits and presets come from FM27's own game data. The module only reads and writes after checks, with undo. Single-player only. Font: Inter (SIL OFL). Icons: Phosphor (MIT)." => "Los nombres, límites y plantillas proceden de los datos de FM27. El módulo verifica antes de escribir y permite deshacer. Solo un jugador. Fuente: Inter (SIL OFL). Iconos: Phosphor (MIT).",
        "Set pieces every (weeks, 0 = off)" => "Balón parado cada (semanas, 0 = no)",
        "Settings" => "Ajustes",
        "Sharpness" => "Ritmo",
        "Shooting" => "Definición / Tiro",
        "Smart Additional Focus (role-free)" => "Enfoque adicional inteligente (sin forzar rol)",
        "Smart Injury-Safe Individual Intensity" => "Intensidad individual anti-lesiones",
        "Special camp" => "Concentración especial",
        "Squad Readiness" => "Preparación de la plantilla",
        "Squad looks fresh" => "La plantilla está fresca",
        "Standard (auto)" => "Estándar (auto)",
        "Start FM with module" => "Iniciar FM con módulo",
        "Start FM27" => "Iniciar FM27",
        "Start FM27 on your PC: Steam (or the button below), the Xbox app / Game Pass or Epic." => "Inicia FM27 en tu PC: Steam (o el botón inferior), Xbox / Game Pass o Epic.",
        "Status" => "Estado",
        "Still not working? Attach last-capture.png and last-analysis.txt from the app folder (Settings → Open app folder) to your report." => "¿Sigue sin funcionar? Adjunta last-capture.png y last-analysis.txt de la carpeta de la app.",
        "Strength" => "Fuerza",
        "Stricter injury guard: Half intensity earlier, 30+ veterans protected." => "Protección estricta: media intensidad antes y veteranos 30+ protegidos.",
        "Suggested Focus" => "Enfoque sugerido",
        "Sunday" => "Domingo",
        "Support development" => "Apoyar el desarrollo",
        "Support development (buymeacoffee.com/cilek14)" => "Apoyar el desarrollo (buymeacoffee.com/cilek14)",
        "Support on Buy Me a Coffee" => "Apoyar en Buy Me a Coffee",
        "Tactic" => "Táctica",
        "Tactic Role Individual Training" => "Entrenamiento individual por rol táctico",
        "Tactic style" => "Estilo táctico",
        "Tactic style:" => "Estilo táctico:",
        "Tactical" => "Táctica",
        "Tactical Effectiveness" => "Eficacia Táctica",
        "Tactical Understanding" => "Comprensión táctica",
        "Tactics" => "Tácticas",
        "Team Bonding below Relationships (%)" => "Cohesión de equipo bajo Relaciones (%)",
        "The FM window is not visible. Bring FM into view (with the Breakdown open) and read again." => "La ventana de FM no es visible. Trae FM al frente y vuelve a leer.",
        "The app connects automatically." => "La app se conecta automáticamente.",
        "The app reads FM27 directly while it runs. Nothing to install." => "La app lee FM27 directamente mientras se ejecuta. Sin instalación.",
        "The app talks to FM27 directly through a small module that starts with FM." => "La app se comunica con FM27 mediante un pequeño módulo que arranca con el juego.",
        "The day after a match gets a tactical session after Recovery." => "El día después del partido añade una sesión táctica tras Recuperación.",
        "The regular plan." => "El plan equilibrado estándar.",
        "The rhythm around matches follows SI's presets; the focus shifts week by week." => "El ritmo en torno a los partidos sigue las plantillas de SI; el enfoque cambia cada semana.",
        "The way to spread your available players over the eleven places with the highest total fit. Injured and suspended players are left out." => "Distribuye a los jugadores disponibles en los once puestos con mayor encaje total (excluye lesionados y sancionados).",
        "This FM build has not been verified with FM27 Trainer yet: read-only. Check that club and squad match FM; you can allow changes in Settings." => "Esta versión de FM27 aún no está verificada: solo lectura. Si club y plantilla coinciden con FM, permite los cambios en Ajustes.",
        "This FM build is tested." => "Esta versión de FM está probada.",
        "This is not the tactic in use: FM keeps your starting eleven for that tactic only. So below you only see the best eleven for this tactic." => "No es la táctica activa: FM guarda el once solo para la táctica en uso. Abajo solo ves el mejor once teórico.",
        "This is not your own team. The app can only plan its training if FM keeps a calendar of its own for it (you run this team's training). The plan uses the values you read last." => "No es tu primer equipo. La app solo planifica si FM mantiene un calendario propio para ese equipo.",
        "Those slots go to player development; match preparation stays. Meant for use with \"Keep at 100 %\" for tactical understanding (Squad)." => "Esos huecos pasan al desarrollo de jugadores; la preparación de partido se mantiene.",
        "Thu (EL/ECL)" => "Jue (EL/UECL)",
        "Tip: open Individual Training once in FM27 to capture live focus IDs and enable one-click apply." => "Consejo: abre Entrenamiento Individual una vez en FM27 para activar el envío en un clic.",
        "Tip: open Individual Training once in FM27 so live Position & Role option IDs are captured by BepInEx." => "Consejo: abre Entrenamiento Individual una vez en FM27 para que BepInEx capture los IDs de posición y rol.",
        "Tip: open Individual Training once in FM27 so the game plugin can capture Position & Role IDs." => "Consejo: abre Entrenamiento Individual una vez en FM27 para que el módulo capture los IDs de posición y rol.",
        "Too many team, player and opposition instructions. Drop the ones that add little and bring them back later." => "Demasiadas instrucciones de equipo, jugador y rival. Quita las menos útiles por ahora.",
        "Train the session with the same name (Tactical Understanding group). Set your Primary Trained Tactic to the tactic you play." => "Entrena la sesión con el mismo nombre (grupo Comprensión Táctica). Fija tu táctica principal entrenada en la que juegas.",
        "Training" => "Entrenamiento",
        "Training day" => "Día de entrenamiento",
        "Training of" => "Entrenamiento de",
        "Training-only build: only the training calendar is written; all editor/cheat tools are removed." => "Edición sin trucos: solo se escribe el calendario de entrenamiento; todas las funciones de editor/trampas están eliminadas.",
        "Tue (CL)" => "Mar (CL)",
        "Understanding" => "Comprensión",
        "Untested FM build: changes allowed." => "Versión FM no probada: cambios permitidos.",
        "Untested FM build: read-only." => "Versión FM no probada: solo lectura.",
        "Up to date" => "Actualizado",
        "Version & Updates" => "Versión y actualizaciones",
        "Veteran Guard" => "Protección veteranos",
        "Weakest Attributes" => "Atributos más bajos",
        "Weakest place" => "Puesto más flojo",
        "Wed (CL)" => "Mié (CL)",
        "Week" => "Semana",
        "Week from" => "Semana del",
        "Week of" => "Semana del",
        "Weekly Fixture" => "Partidos de la semana",
        "Weekly and season plan as overview and export (works without the module)." => "Plan semanal y de temporada con vista previa y exportación (funciona sin módulo).",
        "Weekly schedules" => "Planes semanales",
        "When all 8 tactical areas are above this threshold, tactical slots automatically switch to player development (0 = off)." => "Cuando las 8 áreas tácticas superan este umbral, los huecos tácticos cambian automáticamente a desarrollo (0 = apagado).",
        "Who drags the tactic down?" => "¿Quién lastra la táctica?",
        "Windows has no text recognition (OCR) installed. Add English: Settings → Time & language → Language & region → Add a language → English, with 'Optical character recognition'." => "Windows no tiene reconocimiento de texto (OCR) instalado. Añade Inglés con OCR en Configuración → Hora e idioma.",
        "Wing & fast attack" => "Bandas y ataque rápido",
        "Winter break" => "Parón invernal",
        "Works once FM runs with the module (see below)." => "Funciona en cuanto FM se ejecuta con el módulo.",
        "Writing training into FM…" => "Escribiendo entrenamiento en FM…",
        "Your eleven in this tactic" => "Tu once en esta táctica",
        "accomplished" => "eficaz",
        "automatic" => "automático",
        "average fit" => "encaje medio",
        "awkward" => "incómodo",
        "bans" => "sanciones",
        "competent" => "competente",
        "component" => "componente",
        "connect" => "conectar",
        "cup" => "copa",
        "date,day,period,type,session1,session2,extra,schedule\n" => "fecha,dia,periodo,tipo,sesion1,sesion2,extra,plan\n",
        "day" => "día",
        "done" => "listo",
        "empty" => "vacío",
        "en" => "es",
        "every" => "cada",
        "excellent" => "excelente",
        "fair" => "aceptable",
        "good" => "bueno",
        "ineffectual" => "ineficaz",
        "injured" => "lesionado",
        "low" => "bajo",
        "manual" => "manual",
        "match next Monday" => "partido el próximo lunes",
        "match previous Sunday" => "partido el domingo anterior",
        "match(es)" => "partido(s)",
        "module not reachable (is FM27 running with the module?)" => "módulo inaccesible (¿está FM27 abierto?)",
        "natural" => "natural",
        "no eleven picked" => "sin once elegido",
        "none" => "ninguno",
        "on plan" => "en plan",
        "players" => "jugadores",
        "refresh" => "actualizar",
        "role the app does not know yet" => "rol aún desconocido",
        "source" => "origen",
        "suspended" => "sancionado",
        "template" => "plantilla",
        "type" => "tipo",
        "unconvincing" => "poco convincente",
        "very low" => "muy bajo",
        "waiting for the Breakdown in FM" => "esperando datos de FM",
        "weeks" => "semanas",
        "✓ Set" => "✓ Fijado",
        "✓ Written" => "✓ Escrito",
        _ => en,
    }
}

pub fn translate_dynamic(en: String) -> String {
    if en == "your eleven is already the best" {
        return "tu once ya es el mejor".into();
    }
    if let Some(n) = en.strip_suffix(" role options captured") {
        return format!("{n} opciones de rol capturadas");
    }
    if let Some(rest) = en.strip_prefix("Selected: ").and_then(|r| r.strip_suffix(" players")) {
        return format!("Seleccionados: {rest} jugadores");
    }
    if let Some(n) = en.strip_prefix("Sent approved tactic role training (").and_then(|r| r.strip_suffix(" actions) to FM27…")) {
        return format!("Entrenamiento por rol aprobado ({n} acciones) enviado a FM27…");
    }
    if let Some(rest) = en.strip_prefix("Downloading update v") {
        return format!("Descargando actualización v{rest}");
    }
    if let Some(rest) = en.strip_prefix("Applying update v") {
        if let Some(ver) = rest.strip_suffix(", restarting…") {
            return format!("Instalando v{ver}, reiniciando…");
        }
        return format!("Aplicando v{rest}");
    }
    if let Some(ver) = en.strip_prefix("New version available: v") {
        return format!("Nueva versión disponible: v{ver}");
    }
    if let Some(ver) = en.strip_prefix("Available: v") {
        return format!("Disponible: v{ver}");
    }
    if let Some(rest) = en.strip_prefix("Downloading v") {
        return format!("Descargando v{rest}");
    }
    if let Some(rest) = en.strip_prefix("Installing v") {
        return format!("Instalando v{rest}");
    }
    if let Some(rest) = en.strip_prefix("Install v") {
        if let Some(ver) = rest.strip_suffix(" & restart") {
            return format!("Instalar v{ver} y reiniciar");
        }
    }
    if let Some(rest) = en.strip_prefix("Last read: ") {
        return format!("Última lectura: {rest}");
    }
    if let Some(rest) = en.strip_prefix("Read: ") {
        return format!("Leído: {rest}");
    }
    if let Some(rest) = en.strip_prefix("no change (") {
        return format!("sin cambios ({rest}");
    }
    if let Some(rest) = en.strip_prefix("updated (") {
        return format!("actualizado ({rest}");
    }
    if let Some(n) = en.strip_prefix("Read ").and_then(|r| r.strip_suffix(" of 17 gauges.")) {
        return format!("Leídos {n} de 17 indicadores.");
    }
    if let Some(w) = en.strip_prefix("Read ").and_then(|r| r.strip_suffix(" weeks ago (game time).")) {
        return format!("Leído hace {w} semanas (tiempo de juego).");
    }
    if let Some(rest) = en.strip_prefix("Please check (colour and fill disagree): ") {
        if let Some(mid) = rest.strip_suffix(". Fix them with 'Adjust values manually'.") {
            return format!("Comprueba (color y barra no coinciden): {mid}. Corrígelo con « Ajustar valores manualmente ».");
        }
    }
    if let Some(n) = en.strip_suffix(" days") {
        return format!("{n} días");
    }
    if let Some(n) = en.strip_prefix("Under ") {
        return format!("Menor de {n}");
    }
    if let Some(n) = en.strip_prefix("Team ") {
        return format!("Equipo {n}");
    }
    if let Some(n) = en.strip_suffix(" readings") {
        return format!("{n} lecturas");
    }
    if let Some(p) = en.strip_prefix("Saved: ") {
        return format!("Guardado: {p}");
    }
    if let Some(g) = en.strip_prefix('+').and_then(|r| r.strip_suffix(" to gain")) {
        return format!("+{g} a ganar");
    }
    if let Some(avg) = en.strip_prefix("Squad condition ").and_then(|r| r.strip_suffix("% → 'More rest' automatically active.")) {
        return format!("Condición plantilla {avg}% → « Más descanso » activado automáticamente.");
    }
    if let Some(avg) = en.strip_prefix("Current squad condition: ").and_then(|r| r.strip_suffix("% (0 = off).")) {
        return format!("Condición actual de la plantilla: {avg}% (0 = apagado).");
    }
    if let Some((w, rest)) = en.split_once(" weeks · ") {
        if let Some(u) = rest.strip_suffix(" distinct weekly schedules") {
            return format!("{w} semanas · {u} planes semanales distintos");
        }
    }
    if let Some(d) = en.strip_prefix("Week of ") {
        if let Some((date, tail)) = d.split_once(": ") {
            if let Some(n) = tail.strip_suffix(" days put back on plan (FM had re-planned the week).") {
                return format!("Semana del {date}: {n} días restaurados (FM había alterado la semana).");
            }
            if let Some(n) = tail.strip_suffix(" days written into FM.") {
                return format!("Semana del {date}: {n} días escritos en FM.");
            }
        }
        return format!("Semana del {d}");
    }
    if let Some(n) = en.strip_suffix(" days written into FM.") {
        return format!("{n} días escritos en FM.");
    }
    if let Some(n) = en.strip_suffix(" match(es)") {
        return format!("{n} partido(s)");
    }
    if let Some(n) = en.strip_suffix(" days restored") {
        return format!("{n} días restaurados");
    }
    if let Some(n) = en.strip_suffix(" days differ from your plan") {
        return format!("{n} días difieren de tu plan");
    }
    if let Some(s) = en.strip_prefix("FM had planned: ") {
        return format!("FM había previsto: {s}");
    }
    // Individual / Radar / Bottlenecks
    if let Some(v) = en.strip_suffix("+ yrs guarded") {
        return format!("{v}+ años protegidos");
    }
    if let Some(v) = en.strip_prefix("Sharpness ") {
        return format!("Ritmo {v}");
    }
    if let Some((n, avg)) = en.split_once(" players · avg ") {
        return format!("{n} jugadores · media {avg}");
    }
    if let Some(n) = en.strip_suffix(" match(es) this week") {
        return format!("{n} partido(s) esta semana");
    }
    if let Some(n) = en.strip_suffix(" options captured") {
        return format!("{n} opciones capturadas");
    }
    if let Some(n) = en.strip_prefix("Updated individual training for ").and_then(|r| r.strip_suffix(" players in FM27.")) {
        return format!("Entrenamiento individual actualizado para {n} jugadores en FM27.");
    }
    if let Some(n) = en.strip_prefix("Sent intensity command for ").and_then(|r| r.strip_suffix(" players to FM27 (BepInEx)…")) {
        return format!("Orden de intensidad enviada para {n} jugadores a FM27 (BepInEx)…");
    }
    if let Some(n) = en.strip_prefix("Sent Additional Focus command for ").and_then(|r| r.strip_suffix(" players to FM27…")) {
        return format!("Orden de Enfoque Adicional enviada para {n} jugadores a FM27…");
    }
    if let Some(mid) = en.strip_prefix("Injured → low-strain technical/tactical focus (").and_then(|r| r.strip_suffix(").")) {
        return format!("Lesionado → enfoque técnico/táctico suave ({mid}).");
    }
    if let Some(mid) = en.strip_prefix("32+ → low-strain focus on ").and_then(|r| r.strip_suffix('.')) {
        return format!("32+ → enfoque suave en {mid}.");
    }
    if let Some(rest) = en.strip_prefix("Lowest core attributes for ") {
        if let Some((pos, tail)) = rest.split_once(": ") {
            if let Some((attrs, avg)) = tail.split_once(" (avg ") {
                let avg = avg.trim_end_matches(").");
                return format!("Atributos clave más bajos en {pos}: {attrs} (media {avg}).");
            }
        }
    }
    if let Some(rest) = en.strip_prefix("Condition ") {
        if let Some((pct, thr)) = rest.split_once("% (<") {
            if let Some(thr) = thr.strip_suffix("%) → high injury risk, rest on matchday.") {
                return format!("Condición {pct}% (<{thr}%) → alto riesgo de lesión, descanso en día de partido.");
            }
            if let Some(thr) = thr.strip_suffix("%) → recovery priority.") {
                return format!("Condición {pct}% (<{thr}%) → prioridad recuperación.");
            }
        }
        if let Some(pct) = rest.strip_suffix("% · balanced workload.") {
            return format!("Condición {pct}% · carga equilibrada.");
        }
    }
    if let Some(rest) = en.strip_prefix("Veteran (") {
        if let Some((age, pct)) = rest.split_once("y) + congested week/condition ") {
            if let Some(pct) = pct.strip_suffix("% → limit muscle load.") {
                return format!("Veterano ({age}a) + semana cargada/condición {pct}% → limitar carga.");
            }
        }
        if let Some((age, pct)) = rest.split_once("y) fresh (") {
            if let Some(pct) = pct.strip_suffix("%) → capped at Normal to prevent injuries.") {
                return format!("Veterano ({age}a) fresco ({pct}%) → limitado a Normal por precaución.");
            }
        }
    }
    if let Some((m, pct)) = en.split_once(" matches this week + condition ") {
        if let Some(pct) = pct.strip_suffix("% → prevent overload.") {
            return format!("{m} partidos esta semana + condición {pct}% → evitar sobrecarga.");
        }
    }
    if let Some(m) = en.strip_suffix(" matches this week → Double avoided for injury prevention.") {
        return format!("{m} partidos esta semana → Doble evitada por prevención.");
    }
    if let Some(rest) = en.strip_prefix("Rotation player: condition ") {
        if let Some((c, s)) = rest.split_once("%, sharpness ") {
            if let Some(s) = s.strip_suffix("% → build sharpness.") {
                return format!("Jugador de rotación: condición {c}%, ritmo {s}% → ganar ritmo.");
            }
        }
    }
    if let Some(rest) = en.strip_prefix("Young prospect (") {
        if let Some((age, c)) = rest.split_once("y) & fresh (") {
            if let Some(c) = c.strip_suffix("%) → max development.") {
                return format!("Joven promesa ({age}a) y fresco ({c}%) → máximo desarrollo.");
            }
        }
    }
    if let Some(rest) = en.strip_prefix("Fresh (") {
        if let Some((c, s)) = rest.split_once("%), low sharpness (") {
            if let Some(s) = s.strip_suffix("%) → boost sharpness.") {
                return format!("Fresco ({c}%), poco ritmo ({s}%) → subir ritmo.");
            }
        }
    }
    if let Some(c) = en.strip_prefix("Peak condition (").and_then(|r| r.strip_suffix("%) & light week → safe for Double intensity.")) {
        return format!("Condición óptima ({c}%) y semana ligera → Doble intensidad segura.");
    }
    // Live & Vision
    if let Some(n) = en.strip_prefix("Tactical understanding put back to 100% for ").and_then(|r| r.strip_suffix(" players (FM had let it drop).")) {
        return format!("Comprensión táctica devuelta al 100 % para {n} jugadores.");
    }
    if let Some(rest) = en.strip_prefix("The capture of the FM window has almost no text (") {
        if let Some(n) = rest.split_once(" lines).").map(|x| x.0) {
            return format!("La captura de FM casi no tiene texto ({n} líneas). Asegúrate de que FM sea visible o usa modo ventana sin bordes.");
        }
    }
    if let Some(rest) = en.strip_prefix("No Breakdown labels found (OCR language ") {
        if let Some(lang) = rest.split_once(").").map(|x| x.0) {
            return format!("No se encontraron etiquetas de Eficacia Táctica (OCR {lang}). Abre Entrenamiento → Eficacia Táctica en FM27 con el desglose visible.");
        }
    }
    if let Some(rest) = en.strip_prefix("Breakdown screen not recognised (") {
        if let Some(n) = rest.split_once(" of 17 gauges").map(|x| x.0) {
            return format!("Pantalla no reconocida ({n} de 17 indicadores encontrados). Abre el resumen de Eficacia Táctica en FM27 e inténtalo de nuevo.");
        }
    }
    en
}
