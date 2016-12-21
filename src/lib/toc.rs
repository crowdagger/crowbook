// Copyright (C) 2016 Élisabeth HENRY.
//
// This file is part of Crowbook.
//
// Crowbook is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published
// by the Free Software Foundation, either version 2.1 of the License, or
// (at your option) any later version.
//
// Caribon is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received ba copy of the GNU Lesser General Public License
// along with Crowbook.  If not, see <http://www.gnu.org/licenses/>.

/// A structure for manipulating Table Of Content
#[derive(Debug)]
pub struct Toc {
    elements: Vec<TocElement>,
    numbered: bool,
}

impl Toc {
    /// Create a new, empty, Toc
    pub fn new() -> Toc {
        Toc {
            elements: vec![],
            numbered: false,
        }
    }

    /// Returns `true` if the toc is empty, `false` else.
    ///
    /// Note that `empty` here means that the the toc has zero *or one*
    /// element, since it's still useless in this case.
    pub fn is_empty(&self) -> bool {
        self.elements.len() <= 1
    }

    /// Sets numbering of the Toc
    ///
    /// Only affects whether the generated HTML code should be <ul> or <ol> (epub)
    pub fn numbered(&mut self, numbered: bool) {
        self.numbered = numbered;
    }

    /// Adds an element
    pub fn add(&mut self, level: i32, url: String, title: String) {
        let element = TocElement::new(level, url, title);
        let mut inserted = false;
        if let Some(ref mut last_elem) = self.elements.last_mut() {
            if level > last_elem.level {
                last_elem.add(element.clone());
                inserted = true;
            }
        }
        if !inserted {
            self.elements.push(element);
        }
    }

    /// Render the Toc in a toc.ncx compatible way, for EPUB.
    pub fn render_epub(&mut self) -> String {
        let mut output = String::new();
        let mut offset = 0;
        for elem in &self.elements {
            let (n, s) = elem.render_epub(offset);
            offset = n;
            output.push_str(&s);
        }
        output
    }

    /// Render the Toc in either <ul> or <ol> form (according to Self::numbered
    pub fn render(&mut self) -> String {
        let mut output = String::new();
        for elem in &self.elements {
            output.push_str(&elem.render(self.numbered));
        }
        format!("<{oul}>\n{output}\n</{oul}>\n",
                output = output,
                oul = if self.numbered { "ol" } else { "ul" })
    }
}


#[derive(Debug, Clone)]
struct TocElement {
    level: i32,
    url: String,
    title: String,
    children: Vec<TocElement>,
}

impl TocElement {
    pub fn new(level: i32, url: String, title: String) -> TocElement {
        TocElement {
            level: level,
            url: url,
            title: title,
            children: vec!(),
        }
    }

    // Add element to self or to children, according to level
    pub fn add(&mut self, element: TocElement) {
        let mut inserted = false;
        if let Some(ref mut last_elem) = self.children.last_mut() {
            if element.level > last_elem.level {
                last_elem.add(element.clone());
                inserted = true;
            }
        }
        if !inserted {
            self.children.push(element);
        }
    }

    // Render element for Epub's toc.ncx format
    pub fn render_epub(&self, mut offset: u32) -> (u32, String) {
        offset += 1;
        let id = offset;
        let children = if self.children.is_empty() {
            String::new()
        } else {
            let mut output = String::new();
            for child in &self.children {
                let (n, s) = child.render_epub(offset);
                offset = n;
                output.push_str(&s);
            }
            output
        };
        (offset,
         format!("
<navPoint id = \"navPoint-{id}\">
  <navLabel>
   <text>{title}</text>
  </navLabel>
  <content src = \"{url}\" />
{children}
</navPoint>",
                id = id,
                title = self.title,
                url = self.url,
                children = children))
                
    }

    // Render element as a list element
    pub fn render(&self, numbered: bool) -> String {
        let children = if self.children.is_empty() {
            String::new()
        } else {
            let mut output = String::new();
            for child in &self.children {
                output.push_str(&child.render(numbered));
            }
            format!("\n<{oul}>{children}\n</{oul}>\n",
                    oul = if numbered { "ol" } else { "ul" },
                    children = output)
        };
        format!("<li><a href = \"{link}\">{title}</a>{children}</li>\n",
                link = self.url,
                title = self.title,
                children = children)

    }
}


