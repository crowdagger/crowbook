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

use crate::token::Token;
use crate::error::Result;

/// Renderer trait, implemented by various renderer to render a list of `Token`s.
pub trait Renderer {
    /// Render an individual token
    fn render_token(&mut self, token: &Token) -> Result<String>;

    /// Renders a vector of tokens
    fn render_vec(&mut self, tokens: &[Token]) -> Result<String> {
        tokens.iter()
            .map(|token| self.render_token(token))
            .collect::<Result<Vec<_>>>()
            .map(|vec| vec.join(""))
    }
}
