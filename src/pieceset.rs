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

use cairo::{Context, ImageSurface};
use resvg::tiny_skia::Transform;
use resvg::usvg;
use std::fs;

use shakmaty::{Color, Piece, Role};

pub struct RenderedPiece {
    pub tree: usvg::Tree,
    pub width: u32,
    pub height: u32,
}

impl RenderedPiece {
    pub fn render_to_cairo(&self, cr: &Context) {
        let width = self.width;
        let height = self.height;

        let mut surface = ImageSurface::create(cairo::Format::ARgb32, width as i32, height as i32)
            .expect("Failed to create surface");

        {
            let mut pixmap =
                tiny_skia::Pixmap::new(width, height).expect("Failed to create pixmap");

            resvg::render(&self.tree, Transform::identity(), &mut pixmap.as_mut());

            let data = pixmap.data();
            let stride = surface.stride() as usize;
            let mut surface_data = surface.data().expect("Failed to get surface data");

            for y in 0..height as usize {
                for x in 0..width as usize {
                    let src_idx = (y * width as usize + x) * 4;
                    let dst_idx = y * stride + x * 4;
                    surface_data[dst_idx] = data[src_idx + 2];
                    surface_data[dst_idx + 1] = data[src_idx + 1];
                    surface_data[dst_idx + 2] = data[src_idx];
                    surface_data[dst_idx + 3] = data[src_idx + 3];
                }
            }
        }

        cr.set_source_surface(&surface, 0.0, 0.0);
        cr.paint();
    }
}

struct PieceSetSide {
    pawn: RenderedPiece,
    knight: RenderedPiece,
    bishop: RenderedPiece,
    rook: RenderedPiece,
    queen: RenderedPiece,
    king: RenderedPiece,
}

impl PieceSetSide {
    fn by_role(&self, role: Role) -> &RenderedPiece {
        match role {
            Role::Pawn => &self.pawn,
            Role::Knight => &self.knight,
            Role::Bishop => &self.bishop,
            Role::Rook => &self.rook,
            Role::Queen => &self.queen,
            Role::King => &self.king,
        }
    }
}

pub struct PieceSet {
    black: PieceSetSide,
    white: PieceSetSide,
}

impl PieceSet {
    fn by_color(&self, color: Color) -> &PieceSetSide {
        color.fold(&self.white, &self.black)
    }

    pub fn by_piece(&self, piece: &Piece) -> &RenderedPiece {
        self.by_color(piece.color).by_role(piece.role)
    }

    pub fn scale(&self) -> f64 {
        1.0 / 177.0
    }
}

fn load_svg(path: &str) -> RenderedPiece {
    let svg_data = fs::read(path).expect(path);
    let tree = usvg::Tree::from_data(&svg_data, &usvg::Options::default())
        .expect(&format!("Failed to parse SVG: {}", path));

    let size = tree.size();
    let width = size.width() as u32;
    let height = size.height() as u32;

    RenderedPiece {
        tree,
        width,
        height,
    }
}

impl PieceSet {
    pub fn merida() -> PieceSet {
        PieceSet {
            black: PieceSetSide {
                pawn: load_svg("src/merida/bP.svg"),
                knight: load_svg("src/merida/bN.svg"),
                bishop: load_svg("src/merida/bB.svg"),
                rook: load_svg("src/merida/bR.svg"),
                queen: load_svg("src/merida/bQ.svg"),
                king: load_svg("src/merida/bK.svg"),
            },
            white: PieceSetSide {
                pawn: load_svg("src/merida/wP.svg"),
                knight: load_svg("src/merida/wN.svg"),
                bishop: load_svg("src/merida/wB.svg"),
                rook: load_svg("src/merida/wR.svg"),
                queen: load_svg("src/merida/wQ.svg"),
                king: load_svg("src/merida/wK.svg"),
            },
        }
    }
}
