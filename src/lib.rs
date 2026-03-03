// This file is part of the chessground library.
// Copyright (C) 2017 Niklas Fiekas <niklas.fiekas@backscattering.de>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

//! A chessboard widget for Relm/GTK.

#![doc(html_root_url = "https://docs.rs/chessground/0.9.0")]
#![warn(missing_debug_implementations)]
#![allow(unused_must_use, dead_code)]

extern crate cairo;
extern crate gdk;
extern crate gtk;
extern crate relm;
extern crate shakmaty;
extern crate time;
#[macro_use]
extern crate relm_derive;

mod boardstate;
mod drawable;
mod ground;
mod pieces;
mod pieceset;
mod pockets;
mod promotable;
mod util;

pub use drawable::{DrawBrush, DrawShape};
pub use ground::{Ground, GroundMsg, Pos};
pub use GroundMsg::*;
