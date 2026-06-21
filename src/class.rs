use crate::stats::Stats;

#[derive(Debug)]
pub enum Class {
    Knight,
    Mage,
    Rogue,
}

impl Class {
    pub fn ascii(&self) -> &'static str {
        match self {
            Class::Knight => concat!(
                "               ;.   :xxxx:.               \n",
                "               .+ .:::+;:+x;              \n",
                "                 +;.::;+: +.              \n",
                "                 :+:;;;x; .x:             \n",
                "                  ++;;;x+                 \n",
                "              +: ;$X+;xxxXx:;:            \n",
                "               ;$$$$x;+;;:.. .;:          \n",
                "             .+++Xx+::....:....:+.        \n",
                "             :++XXX;;::.:+:.::...;        \n",
                "             :xxxx+:...  ;;:;;. :::       \n",
                "             :$$xx+:.. ...+X$Xxxx;        \n",
                "             +&$Xx+;;;::::;X$$X:x         \n",
                "            x&&$Xx+;;;+;+xX$xXXxx         \n",
                "           XXxX$$Xx+++++xXX .$xxx;+.      \n",
                "          :XX$$x$Xx++++xxXX  x+x;+x:      \n",
                "          XxX$+X$$Xxxx++xXXX ;XXxx;       \n",
                "          xx+:X$$$$$$$XXXX$$+:x;+;        \n",
                "         ;xX;;+xXxxxxxXxxx+XXXx;x+        \n",
                "         +xX+;x$+;::::+xx+:XXXxxx         \n",
                "        .xx xX$$+;::.::x++:+$xxX          \n",
                "         +: .X$X+;:::..+++++XXXX          \n",
                "            .X$X+;:::..+xx+;xXx+;         \n",
                "             xXx;;::::.;XXXxxXx++:        \n",
                "             +Xx+x;:+::;Xx++XXx.+;        \n",
                "             +XX$x+;;;:+xX+;XX+: .        \n",
                "            ;XxxXxXXxx.xXXx;x$x:          \n",
                "           .;Xx+xxxxxx ;XXx:+XX::         \n",
                "           ;+xxxx+++XX :XXxx+xX+:         \n",
                "          :;XXXx+++xxx .+XXxxx+x::        \n",
                "          x;xxxx;+xx;; .+xXXx;+X++        \n",
                "          ::;  +;+:       :Xx+xx;:        \n",
                "          ..   ;+;.        xx+;+x.        \n",
                "              ;x+;;        ;++.           \n",
                "             .++xx;        ;x+.           \n",
                "            :;++:.         xxxx           \n",
                "          ;;::.            XxX:           \n",
                "                          .x+x:           \n",
                "                          .+;;."
            ),
            Class::Mage => "\
   /|\\\n\
  / * \\\n\
 | ~~~ |\n\
  \\   /\n\
   | |\n\
   |_|",
            Class::Rogue => "\
  .---.\n\
 (`. .`)\n\
  | ^ |\n\
  |___|\n\
  /| |\\\n\
 / |_| \\",
        }
    }

    pub fn base_stats(&self) -> Stats {
        match self {
            Class::Knight => Stats {
                vigor: 14,
                intelligence: 9,
                faith: 9,
                dexterity: 13,
                arcane: 7,
                mind: 9,
                strength: 14,
            },
            Class::Mage => Stats {
                vigor: 9,
                intelligence: 20,
                faith: 7,
                dexterity: 12,
                arcane: 9,
                mind: 14,
                strength: 5,
            },
            Class::Rogue => Stats {
                vigor: 10,
                intelligence: 11,
                faith: 8,
                dexterity: 20,
                arcane: 11,
                mind: 11,
                strength: 10,
            },
        }
    }
}
