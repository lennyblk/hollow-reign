use std::io::{self, Write};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};

use crate::catalog::item_by_name;
use crate::player::Player;

// ─── DONNÉES NPC ─────────────────────────────────────────────────────────────

struct NpcData {
    name: &'static str,
    ascii: &'static str,
    // Dialogues selon état
    intro: &'static str,       // Première rencontre (avant quête)
    quest_offer: &'static str, // Proposition de quête
    active: &'static str,      // Quête en cours, item manquant
    complete: &'static str,    // Joueur a l'item requis
    done: &'static str,        // Quête terminée / plus tard
    no_quest: &'static str,    // NPC sans quête
    // Quête
    required_item: Option<&'static str>,
    reward_souls: u32,
    reward_item: Option<&'static str>,
}

fn npc_data(name: &str) -> Option<NpcData> {
    match name {
        "Edric le Creux" => Some(NpcData {
            name: "Edric le Creux",
            ascii: r#"
                       +*.
                     +*+***:
                    ++--==+*+
                  #*==::: ::*#*
                 *=:.: ....::=+
               =**-:..   .:-::-=:
              +#####:=:..:=-*#**%#
            .**#####+===-+*=*####%#
           =###***##-=*+*+==+#######.
           *##*###==:======.==#######=
          *##*#++*==:+*=-+*:===######*
          ###*-=++==++++++*=+==+**##*#.
          ##+:+=++====+==*==+=====####*
          ##*.:=+*+=-:====:++*+*=######
         *%##*+-==-.:-:=:.:-:+--%##%%##.
        +#*####=. ..:.:-:.....:=##****#=
        .#**%#*++--+=***-=+**=++*####*%#
         *%%%#=----+%**==+*#+--++%####%%
          *#%#-==--=*#*#+**=-=:*-##%#%%=
           *%*--++--:::::::==+-+:-##%%*
            *+=:-:--:::==-:-=+**-=#%%.
             -=-::--:::------=+==+%%
             +:-:::-:::----=---====
            :+:-::-:-::=-------=-+=
            ++---:-----+=-------=**
            *===---+=-==-==------**
            *==+--==+========----+*
            +*+++===++====+====--+=
            +*+++==++++========-=*
             *+++++++++======++++*
             *+++****++++++**+*+**
             *****+*+*#**++****##*
             -####****+ +*####**#*
              -=######.  +####%##.
                %%%%#-     %%%%#
               .@@%%%+    :%%*%%.
               %%#*##=     -#%##%+
             -####            +%##%"#,
            intro: "\
Tu es frais... ca se voit. Cet endroit te videra comme il m'a vide.\n\
La malediction se repand depuis le Vide. Le Roi Creux trone la-bas, \
dans l'obscurite absolue.\n\
Personne n'a reussi a l'atteindre. Si tu veux survivre ici, commence \
par t'armer correctement.\n\
J'ai laisse ma vieille lame dans la tour de guet. Va la chercher.",
            quest_offer: "\
Il y a une vieille lame dans la tour de guet. Prouve-moi que tu peux \
y arriver vivant - ramene-la. Elle est a toi si tu y parviens.",
            active: "\
La tour de guet est par la. La lame est dans le coffre, au sommet.",
            complete: "\
Tu l'as trouvee. Garde-la - elle t'appartient desormais.\n\
En echange, prends ca. C'est un fragment de carte que j'avais \
gribouille avant de perdre la tete.\n\
Elle montre le chemin jusqu'au Vide. Utilise-la dans ton inventaire.",
            done: "\
Le Roi Creux attend dans le Vide. Aucun ne revient de la-bas...\n\
Mais toi, peut-etre que tu es different.",
            no_quest: "...",
            required_item: Some("Worn Shortsword"),
            reward_souls: 0,
            reward_item: Some("Fragment de Carte"),
        }),

        "Soeur Vael" => Some(NpcData {
            name: "Soeur Vael",
            ascii: r#"
                   .-=++-
                  .--:..:=-
                  --=-::+==.
                 --=-:::.-+-
                .--=+=:.+#==.
                :*==+*+-**#+*+
                ***++**+*#***+-
              =**+**#***+++****#*.
           =**++++*##**+**+++++++
         :---=+**#--=**++++++++*++**
        =------=%#=--*+*++++++**=----
        ::--+###*##%**++++++*###=----
         .++:-***+#*##*+**##**##===-+
         :##%=*#*****#####==.*#*-=:+
         =######==##*###++#*=  :::
         *###**%###**#%#**###**+
         *#*#*#*%%+**##*###=*%#*
        =*#*###*%###%%%#######**+
        *****#***#%########*####+
        *******#########*##%%#*
        ***#***##******######%%
        *+*****#*#%###*+..**%##.
          *****+##%##-    =###*:
          +++++=####*      ####*
           ===- *###.      :####
           -:   ####         ###*
               -####         ####+
               *####.        :####.
               -####          *###-
                *##+           ###=
                 ***           *###
                =#**#:         *****=
                 +***-          #**:
                 +***:          .*#*
                 .*-*            *==:
                 ****            ****
                 :**=             .."#,
            intro: "\
Ces tombes... elles murmurent. La malediction du Roi Creux s'etend \
jusqu'ici, troublant les morts.\n\
Une creature garde le tombeau ancien au nord - le Gardien du Tombeau. \
Il profane tout ce que j'etudie.\n\
Tue-le. Rapporte-moi sa lame comme preuve.",
            quest_offer: "\
Tue le Gardien du Tombeau et rapporte-moi la Hunter's Blade.\n\
Fais cela, et je te dirai ce que je sais du Vide et du Roi Creux.",
            active: "\
Le Gardien rode encore dans les tombes. Sa lame sera preuve suffisante \
de ta victoire.",
            complete: "\
La Hunter's Blade... tu l'as fait.\n\
Le Vide. C'est un espace entre les mondes, ou les ames vont quand le \
cycle se brise. Le Roi Creux maintient tout ca par la force.\n\
Si tu le tues... le cycle pourrait repartir. Prends ces provisions.",
            done: "\
Va. Les Cendres t'attendent, puis Frostveil... et enfin le Vide.\n\
La fin du roi est ta destinee.",
            no_quest: "...",
            required_item: Some("Hunter's Blade"),
            reward_souls: 200,
            reward_item: Some("Antidote"),
        }),

        "Osryn le Chevalier Braise" => Some(NpcData {
            name: "Osryn le Chevalier Braise",
            ascii: r#"
                     -  .-
                     =::-+-
                    .##**#=
                    -#+++*#
                 .-:=%@*%@+++=
              :+-..=*+=***+##+++:
            :    --.  -+#%%*=+=**#
            --===-       -=-#%=++*
            -==*@=-.  . ..:=+%@@++*
            %%@@%+=======+++*@@%@%*=  .=
            ###%%#*+++++++*#%@%##@@#+=+-
           **%%@@*%*++++++*%@@@@%**%=++:
          .#**%@@@##******#%@@@@%*=#*==--
          =:-#%@@%*#%+:=*#%##@@@*=.====-*+-
          -=+#*@%##%%#*+#@%@##@@#*-===-=***.
          =++*%%%#%@@@%%%%%%%%%#@*+==+==**+#
          -+*#%*##=%%%#*#%%++-#%#%%*==++##*-
          +*%*@**#+@%%#%%%##=#**@+**+++*%#*
       .+ -%+%%*#+%@@@@@@%%##++*%##*+++#*%*.
          -+%@@@%#%%%@@@@%##%%++%#@++++*#.
         :=*@@##%#%@@@@@@%%####%@@@++++#*-
        .=+%#@@###%%@@@@@@%%##%%@@@++++#**=
        -=.**@@%#%%%@@@@@@@%%%%%%%@=++*#***.
       .=: ++%***%%%@@@@@@@%%++=*#%+++##*##-
       =-  ++#++++##@@@@@@@*++*++##+++#***#
      ==   +*#++++*#@@@@@@@#**++*#****#-+#
    .==    *%@#**#%##@@@@@@%+#%%##*#*#*==.
   .==    :*%+*#%#*%@%@%@@@*++*+*%*#+*=
  .-=     +*#@=+***%@#@*#@@%*+#+#%*.#+.
  -=      ##*#=***#%%%@#*#@@%+**#:
 :+       ###*+**#*#%%##**#%@%**#.
 -       +####+**#*##%###@###%****
         #%#*#***#*#####      =*##
        .*####*###*###*@*     #*#**
        +##*+**#%=-   ##@#-   -#+=:*=
        .#****%%%.        *+  :###+++*:"#,
            intro: "\
J'attends ici depuis des mois. Le Seigneur du Feu... il a tue toute \
ma compagnie.\n\
Je suis trop affaibli pour l'affronter seul maintenant. Mais toi...\n\
Je vois quelque chose en toi. Prouve-le. Tue-le. \
Rapporte-moi sa lame.",
            quest_offer: "\
L'Ashfall Katana. C'est ce que le seigneur du feu porte.\n\
Apporte-la moi. C'est tout ce que je demande.",
            active: "\
Le Seigneur du Feu garde la salle de fonte, plus loin dans les cendres. \
Sa flamme ne s'eteint jamais.",
            complete: "\
L'Ashfall Katana... mes compagnons peuvent se reposer maintenant.\n\
Prends ces ames. Tu en auras besoin pour Frostveil et au-dela.\n\
Le Roi Creux... quand tu l'affronteras, souviens-toi : \
il etait autrefois un roi comme un autre. Le Vide l'a brise.",
            done: "\
Au-dela de Frostveil se trouve le Vide. Le pouvoir du Roi Creux \
y est absolu.\n\
Seul quelqu'un qui a conquis le feu et la glace a une chance.",
            no_quest: "...",
            required_item: Some("Ashfall Katana"),
            reward_souls: 500,
            reward_item: None,
        }),

        "Le Pelerin Pale" => Some(NpcData {
            name: "Le Pelerin Pale",
            ascii: r#"
                *#######+ :
                 +##%######-
                -###%##**## :=
                :-%#%**#=*#+-
                 :*#++==+*+*++-
                   +*+=+*+%%##-
                   =#%%+++#**+=#*
                  *#**+++**+###%#:
               .=:-++**##%**#####.
              :==--:=:%##-*%%@@@@@*
               #+##+=::+##**%####=
              +*#=::-:-=+#**#+===-
             *%%%++=+-++*##%=====*
             *%%%#+++++++**    *+=*
          .***#%%#**#####%#+   ++##*
          +++##%%%@%#%%##%#+* .**+##
         #%#%%%@%%##*+*#**%##- -*#%%
         :+%%%%%%#=#%*+#+==+** #*#+-
         %%%%.*#=+*=-+++%#=-=: %%%%
        #%%#-=##=-=--+%#**#+=+####*
        #%%%**%#===-=+**#%%%*%%#**#.
        #%%%%*@#=+===*#%%%#%%%%**#
         %%#%#%#=====###%%%%%%%%%%
          =+#%##++==+###%%%%%%%%%*
           #%%##+++++%##%%%%%@@#*=
           *%%%#+++++@@#%%%%%%%%+++
          =#%%%#*****@%%%%%%%%%%#+*+
          +#%%%#+***#@@%%%%%%%%%#+*+=
          *#%%%#****#@%%%%%%%%%%%*+*+.
         -*#%%%%****%@%%%%%%%%%%%%+*++
         +**++@@***@@%%%%%%%%%@@@++++*+
         +++++*%*%@@%+: #@@@%@@@*+*++*+.
        .++++++*%@@@@+=  :@%@@%%*+*++***
        -+*+===+%@@%%*=: :*%%%%%%=+++***:
        =+++. .=%%%%@*== :+%%%%%%==******
        -*+-    :%%%-       *%%@* =*****#-
         **+ %#*-##+         ##%%%==****#+
          **  %%%%%%=        %%%%# .**#*.
           -  %%%%%#        .%%%%%  ***.
             ###%%%#         %@%%%  **.
           *%%%%#%%#         *#%%%# +
        -%%%%%%#=--.         =@%%%%#
                             #%%%%%%=
                              :+**="#,
            intro: "\
Bienvenue, voyageur. J'ai parcouru ces pics glaces plus longtemps \
que je ne veux l'admettre.\n\
J'etais la, une fois... au seuil du Vide. Le pouvoir du Roi Creux \
irradie de l'interieur. Il gele l'ame plus que n'importe quel givre.",
            quest_offer: "",
            active: "",
            complete: "",
            done: "\
La faiblesse du Roi Creux... ils disent que c'est la meme que celle \
de tout roi : il craint d'etre oublie.\n\
Defais-le. Prends le trone. Brise le cycle.",
            no_quest: "\
Je commerce ce que je trouve sur ma route.\n\
Le Roi Creux attend dans le Vide. Aucun de ceux qui y entrent \
ne revient jamais... sauf peut-etre toi.",
            required_item: None,
            reward_souls: 0,
            reward_item: None,
        }),

        _ => None,
    }
}

// ─── ÉTAT DE QUÊTE ───────────────────────────────────────────────────────────

#[derive(PartialEq)]
enum DialoguePhase {
    Intro,
    Active,
    CanComplete,
    Done,
    NoQuest,
}

fn current_phase(player: &Player, data: &NpcData) -> DialoguePhase {
    if data.required_item.is_none() {
        // Le Pelerin Pale - pas de quête
        if player.completed_quests.contains(data.name) {
            return DialoguePhase::Done;
        }
        return DialoguePhase::NoQuest;
    }

    if player.completed_quests.contains(data.name) {
        return DialoguePhase::Done;
    }

    if player.active_quests.contains(data.name) {
        // Quête active — est-ce que le joueur a l'item ?
        let has_item = data
            .required_item
            .map(|req| player.inventory.iter().any(|i| i.name() == req))
            .unwrap_or(false);
        if has_item {
            DialoguePhase::CanComplete
        } else {
            DialoguePhase::Active
        }
    } else {
        DialoguePhase::Intro
    }
}

// ─── POINT D'ENTRÉE ──────────────────────────────────────────────────────────

pub fn run_npc_dialogue(player: &mut Player, npc_name: &'static str) {
    let data = match npc_data(npc_name) {
        Some(d) => d,
        None => return,
    };

    let mut out = io::stdout();
    terminal::enable_raw_mode().ok();
    execute!(out, Hide).ok();

    'outer: loop {
        let phase = current_phase(player, &data);

        match phase {
            DialoguePhase::Intro => {
                // Page 1 : intro
                show_page(&mut out, &data, data.intro, "[Entree] Continuer", false);
                if !wait_enter_or_esc(&mut out) {
                    break 'outer;
                }

                // Page 2 : offre de quête
                show_page(
                    &mut out,
                    &data,
                    data.quest_offer,
                    "[Entree] Accepter la quete   [Esc] Refuser",
                    false,
                );
                match wait_choice(&mut out) {
                    Choice::Accept => {
                        player.active_quests.insert(data.name.to_string());
                        show_page(&mut out, &data, "Quete acceptee.", "[Esc] Fermer", false);
                        wait_any(&mut out);
                        break 'outer;
                    }
                    Choice::Decline => break 'outer,
                }
            }

            DialoguePhase::Active => {
                show_page(&mut out, &data, data.active, "[Esc] Fermer", false);
                wait_any(&mut out);
                break 'outer;
            }

            DialoguePhase::CanComplete => {
                show_page(
                    &mut out,
                    &data,
                    data.complete,
                    "[Entree] Recevoir la recompense   [Esc] Plus tard",
                    false,
                );
                match wait_choice(&mut out) {
                    Choice::Accept => {
                        // L'item reste dans l'inventaire du joueur (preuve seulement)
                        // Donner la récompense
                        player.souls += data.reward_souls;
                        if let Some(item_name) = data.reward_item {
                            if let Some(item) = item_by_name(item_name) {
                                player.pick_up(item);
                            }
                        }
                        // Marquer complétée
                        player.active_quests.remove(data.name);
                        player.completed_quests.insert(data.name.to_string());

                        let reward_msg = {
                            let souls_part = if data.reward_souls > 0 {
                                format!("+{} ames", data.reward_souls)
                            } else {
                                String::new()
                            };
                            let item_part = data.reward_item
                                .map(|n| format!("{}", n))
                                .unwrap_or_default();
                            match (souls_part.is_empty(), item_part.is_empty()) {
                                (false, false) => format!("Recompense recue : {} + {}", souls_part, item_part),
                                (false, true)  => format!("Recompense recue : {}", souls_part),
                                (true, false)  => format!("Recompense recue : {}", item_part),
                                (true, true)   => "Merci.".to_string(),
                            }
                        };
                        show_page(&mut out, &data, &reward_msg, "[Esc] Fermer", true);
                        wait_any(&mut out);
                        break 'outer;
                    }
                    Choice::Decline => break 'outer,
                }
            }

            DialoguePhase::Done => {
                show_page(&mut out, &data, data.done, "[Esc] Fermer", false);
                wait_any(&mut out);
                break 'outer;
            }

            DialoguePhase::NoQuest => {
                show_page(&mut out, &data, data.no_quest, "[Esc] Fermer", false);
                wait_any(&mut out);
                break 'outer;
            }
        }
    }

    execute!(out, Show).ok();
    terminal::disable_raw_mode().ok();
}

// ─── RENDU ────────────────────────────────────────────────────────────────────

fn show_page(out: &mut io::Stdout, data: &NpcData, text: &str, hint: &str, is_reward: bool) {
    let (tw, th) = terminal::size().unwrap_or((120, 30));
    execute!(out, Clear(ClearType::All), MoveTo(0, 0)).ok();

    // En-tête
    let title = format!("  {}  ", data.name);
    let bar = "═".repeat((tw as usize).saturating_sub(title.len() + 4));
    execute!(
        out,
        SetForegroundColor(Color::DarkYellow),
        SetAttribute(Attribute::Bold),
        Print(format!("══{}{}══\r\n", title, bar)),
        ResetColor,
    )
    .ok();

    // Séparateur vertical (1/3 - 10 pour ASCII, reste pour texte)
    let split = ((tw as usize) / 3).saturating_sub(10);
    for r in 1..th.saturating_sub(2) {
        execute!(
            out,
            MoveTo(split as u16, r),
            SetForegroundColor(Color::DarkGrey),
            Print("│"),
            ResetColor,
        )
        .ok();
    }

    // ── ASCII NPC (colonne gauche) ────────────────────────────────────────────
    let ascii_max_w = split.saturating_sub(2);
    let ascii_lines: Vec<&str> = data.ascii.lines().collect();
    let min_indent = ascii_lines.iter()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.len() - l.trim_start().len())
        .min()
        .unwrap_or(0);
    for (i, line) in ascii_lines.iter().enumerate() {
        let row = 2 + i as u16;
        if row >= th.saturating_sub(3) {
            break;
        }
        let stripped = if line.len() >= min_indent { &line[min_indent..] } else { line };
        let clipped: String = stripped.chars().take(ascii_max_w).collect();
        execute!(
            out,
            MoveTo(1, row),
            SetForegroundColor(Color::Cyan),
            Print(clipped),
            ResetColor,
        )
        .ok();
    }

    // ── Dialogue (colonne droite) ─────────────────────────────────────────────
    let text_x = (split + 3) as u16;
    let text_w = (tw as usize).saturating_sub(split + 5);
    let text_color = if is_reward { Color::Green } else { Color::Grey };
    let mut row: u16 = 2;

    'text: for paragraph in text.lines() {
        for line in wrap_str(paragraph, text_w) {
            if row >= th.saturating_sub(4) {
                break 'text;
            }
            execute!(
                out,
                MoveTo(text_x, row),
                SetForegroundColor(text_color),
                Print(line),
                ResetColor,
            )
            .ok();
            row += 1;
        }
    }

    // ── Pied de page ─────────────────────────────────────────────────────────
    let footer = th.saturating_sub(2);
    execute!(
        out,
        MoveTo(0, footer),
        SetForegroundColor(Color::DarkGrey),
        Print(format!("  {}", "─".repeat((tw as usize).saturating_sub(4)))),
        ResetColor,
    )
    .ok();
    let hx = (tw.saturating_sub(hint.len() as u16)) / 2;
    execute!(
        out,
        MoveTo(hx, footer + 1),
        SetForegroundColor(Color::DarkGrey),
        Print(hint),
        ResetColor,
    )
    .ok();

    out.flush().ok();
}

// ─── GESTION INPUT ───────────────────────────────────────────────────────────

enum Choice {
    Accept,
    Decline,
}

/// Attend [Entree] (Accept) ou [Esc] (Decline).
fn wait_choice(out: &mut io::Stdout) -> Choice {
    loop {
        if let Ok(Event::Key(KeyEvent {
            code,
            kind: KeyEventKind::Press,
            ..
        })) = event::read()
        {
            match code {
                KeyCode::Enter | KeyCode::Char(' ') => return Choice::Accept,
                KeyCode::Esc => return Choice::Decline,
                _ => {}
            }
        }
        let _ = out;
    }
}

/// Attend [Entree] ou [Esc]. Retourne true si Enter, false si Esc.
fn wait_enter_or_esc(out: &mut io::Stdout) -> bool {
    loop {
        if let Ok(Event::Key(KeyEvent {
            code,
            kind: KeyEventKind::Press,
            ..
        })) = event::read()
        {
            match code {
                KeyCode::Enter | KeyCode::Char(' ') => return true,
                KeyCode::Esc => return false,
                _ => {}
            }
        }
        let _ = out;
    }
}

/// Attend n'importe quelle touche (Esc ou Entree ou autre).
fn wait_any(out: &mut io::Stdout) {
    loop {
        if let Ok(Event::Key(KeyEvent {
            kind: KeyEventKind::Press,
            ..
        })) = event::read()
        {
            break;
        }
        let _ = out;
    }
}

// ─── UTILITAIRE ──────────────────────────────────────────────────────────────

fn wrap_str(text: &str, width: usize) -> Vec<&str> {
    if width == 0 {
        return vec![text];
    }
    let mut lines = Vec::new();
    let mut start = 0;
    while start < text.len() {
        let remaining = &text[start..];
        if remaining.chars().count() <= width {
            lines.push(remaining);
            break;
        }
        let end_byte = remaining
            .char_indices()
            .nth(width)
            .map(|(i, _)| i)
            .unwrap_or(remaining.len());
        let cut_byte = remaining[..end_byte].rfind(' ').unwrap_or(end_byte);
        lines.push(&remaining[..cut_byte]);
        let skip = if remaining[cut_byte..].starts_with(' ') {
            1
        } else {
            0
        };
        start += cut_byte + skip;
    }
    lines
}
