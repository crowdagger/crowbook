pub mod html {
    pub static TEMPLATE:&'static str = include_str!("../../templates/template.html");
    pub static CSS:&'static str = include_str!("../../templates/epub/stylesheet.css");
}

pub mod latex {
    pub static TEMPLATE:&'static str = include_str!("../../templates/template.tex");
}

pub mod epub {
    pub static TEMPLATE:&'static str = include_str!("../../templates/epub/template.xhtml");
    pub static CSS:&'static str = include_str!("../../templates/epub/stylesheet.css");
    pub static CONTAINER:&'static str = include_str!("../../templates/epub/container.xml");
    pub static OPF:&'static str = include_str!("../../templates/epub/content.opf");
    pub static COVER:&'static str = include_str!("../../templates/epub/cover.xhtml");
    pub static IBOOK:&'static str = include_str!("../../templates/epub/ibookstuff.xml");
    pub static NAV:&'static str = include_str!("../../templates/epub/nav.xhtml");
    pub static TITLE:&'static str = include_str!("../../templates/epub/titlepage.xhtml");
    pub static TOC:&'static str = include_str!("../../templates/epub/toc.ncx");
}
