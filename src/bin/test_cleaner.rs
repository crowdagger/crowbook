extern crate crowbook;
use crowbook::cleaner::Cleaner;
use crowbook::cleaner::French;
use std::borrow::Cow;

// Code to end shell colouring
pub const SHELL_COLOUR_OFF: &'static str = "\x1B[0m";
pub const SHELL_COLOUR_RED: &'static str = "\x1B[1m\x1B[31m";
pub const SHELL_COLOUR_BLUE: &'static str = "\x1B[1m\x1B[36m";
pub const SHELL_COLOUR_ORANGE: &'static str = "\x1B[1m\x1B[33m";
pub const SHELL_COLOUR_GREEN: &'static str = "\x1B[1m\x1B[32m";


fn display_spaces(s: &str) {
    let mut res = String::with_capacity(s.len());
    let nb_char = ' ';
    let nb_char_narrow = '\u{202F}'; // narrow non breaking space
    let nb_char_em = '\u{2002}'; // demi em space

    for c in s.chars() {
        if c == nb_char {
            res.push_str(&format!("{}{}{}",
                                  SHELL_COLOUR_ORANGE,
                                  '_',
                                  SHELL_COLOUR_OFF));
        } else if c == nb_char_narrow {
            res.push_str(&format!("{}{}{}",
                                  SHELL_COLOUR_BLUE,
                                  '_',
                                  SHELL_COLOUR_OFF));
        } else if c == nb_char_em {
            res.push_str(&format!("{}{}{}",
                                  SHELL_COLOUR_RED,
                                  '_',
                                  SHELL_COLOUR_OFF));
        } else {
            res.push(c);
        }
    }
    println!("{}\n\n", res);
}

fn main() {
    let s = Cow::Borrowed("10 000 €");
    display_spaces(French.clean(s, false).as_ref());

    let s = Cow::Borrowed("10 000 EUR");
    display_spaces(French.clean(s, false).as_ref());

    let s = Cow::Borrowed("10 000 EUROS");
    display_spaces(French.clean(s, false).as_ref());

    let s = Cow::Borrowed("10 000 fr");
    display_spaces(French.clean(s, false).as_ref());

    let s = Cow::Borrowed("tiret cadratin en début : espace insécable non justifiante, pour que les dialogues restent alignés");
    display_spaces(French.clean(s, false).as_ref());

    let s = Cow::Borrowed(r#"– tiret non cadratin en début : question délicate, est-ce l'ouverture
d'un dialogue ? une liste ? cela dit ce n'est pas a priori l'usage
normal d'un tiret non cadratin suivi d'espace, qui est..."#);
    display_spaces(French.clean(s, false).as_ref());

    let s = Cow::Borrowed(r#"l'angoisse : l'incise. J'ai bien aimé ce film — pas vous ? Là, il semblerait logique de mettre une espace insécable après le tiret. Mais il faut en mettre une avant lorsque ça se referme — comme ça —, sauf
qu'on ne peut pas juste compter les tirets parce que des fois une
incise n'est pas refermée par un tiret — comme en fin de phrase. Le
problème c'est qu'il est dur de trouver des règles simples — repérer
un point n'est pas suffisant, M. le petit malin ! — et qu'au final
avoir une règle qui marche à cent pour cent nécessiterait de l'analyse
syntaxique assez poussée – ce qui va au-delà des prétentions de mon logiciel.."#);
    display_spaces(French.clean(s, false).as_ref());

    let s = Cow::Borrowed(r#"« en début : on pourrait aussi avoir une espace insécable non justifiante, mais de fait l'alignement sera de toute façon rompu car
suivi en général par tiret cadratin. Il faut juste éviter l'espace
fine, qui colle un peu trop le guillemet au dialogue..."#);
    display_spaces(French.clean(s, false).as_ref());

    let s = Cow::Borrowed(r#"— par contre », objecta-t-elle, « il serait bien que si le premier guillemet fermant apparaît sans ouvrant, on en conclut qu'il s'agissait d'un dialogue »"#);
    display_spaces(French.clean(s, false).as_ref());

    let s = Cow::Borrowed(r#"Un « guillemet » pas en début : là, par contre, l'espace insécable fine est bien,
parce que c'est pas terrible d'avoir de grand espaces quand « on » met
un mot entre guillemets"#);
    display_spaces(French.clean(s, false).as_ref());
}
