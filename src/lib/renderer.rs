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

use token::Token;
use error::Result;

/// Renderer trait.
pub trait Renderer {
    /// Render an individual token
    fn render_token(&mut self, token: &Token) -> Result<String>;

    /// Renders a vector of tokens
    fn render_vec(&mut self, tokens: &[Token]) -> Result<String> {
        tokens.iter()
            .map(|token| self.render_token(token))
            .collect::<Result<Vec<_>>>()
            .map(|vec| vec.join(""))

        //     let mut res = String::new();
        
        // for token in tokens {
        //     res.push_str(&try!(self.parse_token(&token)));
        // }
        // Ok(res)
    }
}
