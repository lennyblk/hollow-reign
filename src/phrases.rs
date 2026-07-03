use rand::seq::SliceRandom;
use rand::thread_rng;

// ─── DIFFICULTY ──────────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
pub enum Difficulty {
    Short,  // Mob
    Medium, // MobLeader / MiniBoss
    Long,   // Boss
}

// ─── PHRASE POOLS ────────────────────────────────────────────────────────────

const SHORT: &[&str] = &[
    "frappe!",
    "tiens bon",
    "esquive",
    "pare maintenant",
    "reste ferme",
    "garde haute",
    "ne tombe pas",
    "yeux ouverts",
    "recule",
    "tiens-toi pret",
    "roule vite",
    "reste alerte",
    "force a travers",
    "pas de panique",
    "fais attention",
    "concentre-toi",
    "reste calme",
    "avance encore",
    "tiens la ligne",
    "pas de retraite",
    "bouge vite",
    "bloque le coup",
    "reste en vie",
    "regarde en haut",
    "riposte",
];

const MEDIUM: &[&str] = &[
    "le sang appelle le sang",
    "la flamme ne mourra pas",
    "frappe avant qu'il ne frappe",
    "le creux ne triomphera pas",
    "tiens ton terrain, guerrier",
    "la mort n'est pas une fin ici",
    "crains l'echec, pas les tenebres",
    "respire et trouve ton centre",
    "un coup a la fois",
    "ta lame connait le chemin",
    "depasse la douleur",
    "le vide a faim de ton ame",
    "ne laisse pas les tenebres gagner",
    "releve-toi et frappe encore",
    "cette blessure ne sera pas fatale",
    "chaque cicatrice est une lecon",
    "la grace se souviendra de toi",
    "brille fort avant de t'eteindre",
    "garde ton bras de bouclier stable",
    "des cendres nait la force",
    "la nuit est longue mais tu tiens",
    "rappelle-toi pourquoi tu es venu",
    "la mort n'est qu'un nouveau debut",
    "tiens la ligne contre les tenebres",
    "ta braise refuse de s'eteindre",
];

const LONG: &[&str] = &[
    "des cendres des tombes, une nouvelle force nait en toi chaque jour",
    "les creux se nourrissent des faibles, mais tu n'es pas encore creux",
    "chaque blessure t'apprend comment survivre au prochain combat qui vient",
    "la grace brule pour toujours pour ceux qui refusent de ceder a la mort",
    "dans le royaume des morts, seules les ames les plus fortes perdurent encore",
    "tes ennemis ont depuis longtemps oublie ce que signifie vraiment se battre",
    "les tenebres devorent ceux qui abandonnent leur but en ces lieux maudits",
    "porte le poids de tes compagnons tombes et continue d'avancer sans relache",
    "un vrai guerrier ne compte pas ses blessures, seulement les victoires a venir",
    "les runes de ta lame chantent avec les voix des anciens guerriers disparus",
    "laisse la douleur te rappeler que tu respires encore et que tu es toujours la",
    "le roi qui a creux ce royaume craignait les guerriers qui se battaient comme toi",
    "chaque pas plus profond dans ces tenebres te rapproche un peu plus de la verite",
    "ta lame a deja gout le sang et elle le goutera de nouveau avant la fin",
    "le feu ancien qui brule en toi ne peut jamais etre completement eteint",
    "ceux qui sont morts avant toi ont laisse leur force dans cette terre maudite",
    "hesiter face a la mort certaine c'est l'accueillir a bras ouverts sans resister",
    "l'armee creuse s'effondre devant ceux qui ont vraiment maitrise leur peur profonde",
    "allume ton flambeau contre la nuit sans fin qui devore lentement ce royaume perdu",
    "meme les creatures les plus puissantes peuvent etre vaincues par une ame determinee",
    "le chemin vers l'avant n'est trace que par ceux qui sont prets a saigner pour lui",
    "ta souffrance dans ce royaume maudit n'est pas vaine, la fin approche enfin pour toi",
    "tiens-toi contre la maree montante des tenebres et refuse d'etre emporte par elle",
    "chaque combat survcu rend le suivant un tout petit peu moins impossible a surmonter",
    "quand tout espoir semble perdu, la grace te rappelle ton but oublie depuis longtemps",
];

// ─── PHRASE POOL ─────────────────────────────────────────────────────────────

pub struct PhrasePool {
    short_order: Vec<usize>,
    medium_order: Vec<usize>,
    long_order: Vec<usize>,
    si: usize,
    mi: usize,
    li: usize,
}

impl PhrasePool {
    pub fn new() -> Self {
        let mut pool = Self {
            short_order: (0..SHORT.len()).collect(),
            medium_order: (0..MEDIUM.len()).collect(),
            long_order: (0..LONG.len()).collect(),
            si: 0,
            mi: 0,
            li: 0,
        };
        let mut rng = thread_rng();
        pool.short_order.shuffle(&mut rng);
        pool.medium_order.shuffle(&mut rng);
        pool.long_order.shuffle(&mut rng);
        pool
    }

    pub fn next(&mut self, difficulty: Difficulty) -> &'static str {
        match difficulty {
            Difficulty::Short => {
                if self.si >= self.short_order.len() {
                    self.short_order.shuffle(&mut thread_rng());
                    self.si = 0;
                }
                let phrase = SHORT[self.short_order[self.si]];
                self.si += 1;
                phrase
            }
            Difficulty::Medium => {
                if self.mi >= self.medium_order.len() {
                    self.medium_order.shuffle(&mut thread_rng());
                    self.mi = 0;
                }
                let phrase = MEDIUM[self.medium_order[self.mi]];
                self.mi += 1;
                phrase
            }
            Difficulty::Long => {
                if self.li >= self.long_order.len() {
                    self.long_order.shuffle(&mut thread_rng());
                    self.li = 0;
                }
                let phrase = LONG[self.long_order[self.li]];
                self.li += 1;
                phrase
            }
        }
    }
}
