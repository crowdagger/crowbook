// Copyright (C) 2016-2023 Élisabeth HENRY.
//
// This file is part of Crowbook.
//
// Crowbook is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published
// by the Free Software Foundation, either version 2.1 of the License, or
// (at your option) any later version.
//
// Crowbook is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received ba copy of the GNU Lesser General Public License
// along with Crowbook.  If not, see <http://www.gnu.org/licenses/>.

pub mod highlight {
    pub static CSS: &str = include_str!("../../templates/highlight/default.css");
    pub static JS: &str = include_str!("../../templates/highlight/highlight.pack.js");
}

pub mod html {
    pub static CSS: &str = concat!(
        include_str!("../../templates/epub/stylesheet.css"),
        include_str!("../../templates/html/template.css")
    );
    pub static CSS_COLORS: &str = include_str!("../../templates/html/colors.css");
    pub static PRINT_CSS: &str = include_str!("../../templates/html/print.css");
    pub static JS: &str = include_str!("../../templates/html/script.js");
}

pub mod img {
    pub static MENU_SVG: &[u8] = include_bytes!("../../templates/img/menu.svg");
    pub static BOOK_SVG: &[u8] = include_bytes!("../../templates/img/book.svg");
    pub static PAGES_SVG: &[u8] = include_bytes!("../../templates/img/pages.svg");
}

pub mod html_single {
    pub static HTML: &str = include_str!("../../templates/html_single/template.html");
    pub static JS: &str = include_str!("../../templates/html_single/script.js");
}

pub mod html_dir {
    pub static TEMPLATE: &str = include_str!("../../templates/html_dir/template.html");
}

pub mod html_if {
    pub static JS: &str = include_str!("../../templates/html_if/script.js");
    pub static NEW_GAME: &str = "";
}

pub mod latex {
    pub static TEMPLATE: &str = include_str!("../../templates/latex/template.tex");
}

pub mod epub {
    pub static TEMPLATE: &str = include_str!("../../templates/epub/template.xhtml");
    pub static CSS: &str = include_str!("../../templates/epub/stylesheet.css");
    pub static COVER: &str = include_str!("../../templates/epub/cover.xhtml");
    pub static TITLE: &str = include_str!("../../templates/epub/titlepage.xhtml");
}

pub mod epub3 {
    pub static TEMPLATE: &str = include_str!("../../templates/epub3/template.xhtml");
    pub static COVER: &str = include_str!("../../templates/epub3/cover.xhtml");
    pub static TITLE: &str = include_str!("../../templates/epub3/titlepage.xhtml");
}

#[cfg(feature = "odt")]
pub mod odt {
    pub static CONTENT: &str = include_str!("../../templates/odt/content.xml");
    pub static ODT: &[u8] = include_bytes!("../../templates/odt/template.odt");
}
