use ghost_cell::{GhostCell, GhostToken};
use typed_arena::Arena;
use crate::Bird;
use crate::XY;

enum Diagonal {
    Northeast,
    Southeast,
    Southwest,
    Northwest,
    Unaligned
}
#[derive(PartialEq,Debug,Copy,Clone)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32, //half width
    pub h: f32, //half height
}
impl Rect {
    fn collides(&self, Rect{x, y, w, h}: Rect) -> Diagonal {
        let north = self.y + self.h >= y - h;
        let east = self.x + self.w >= x - w;
        let south = self.y - self.h <= y + h;
        let west = self.x - self.w <= x + w;
        use Diagonal::*;
        match (north, east, south, west) {
            (true, true, _, _) => Northeast,
            (_, true, true, _) => Southeast,
            (_, _, true, true) => Southwest,
            (true, _, _, true) => Northwest,
            _ => Unaligned
        }
    }
}

type NodeRef<'arena, 'id, T> = &'arena GhostCell<'id, QGTree<'arena, 'id, T>>;
struct QGTree<'arena, 'id, T: Copy + XY> {
    local: [Option<T>; 4],
    pub bound: Rect,
    divided: bool,
    ne: Option<NodeRef<'arena, 'id, T>>,
    se: Option<NodeRef<'arena, 'id, T>>,
    sw: Option<NodeRef<'arena, 'id, T>>,
    nw: Option<NodeRef<'arena, 'id, T>>,
    arena: &'arena Arena<Self>
}

impl<'arena, 'id, T: Copy + XY> QGTree<'arena, 'id, T> {
    pub fn new(bound: Rect, arena: &'arena Arena<QGTree<'arena, 'id, T>>)
    -> NodeRef<'arena, 'id, T> {
        GhostCell::from_mut(arena.alloc(QGTree {
            local: [None; 4],
            bound,
            divided: false,
            ne: None,
            se: None,
            sw: None,
            nw: None,
            arena
        }))
    }
    pub fn insert(&mut self, bird: T) -> bool {
        let QGTree{mut local, ..} = *self;

        for i in 0..local.len() {
            if local[i].is_none() {
                local[i] = Some(bird);
                return true;
            }
        }//if full, then subdivide
        self.subdivide(bird);
        false
    }
    fn subdivide(&mut self, bird: T) {
        //TODO
    }
}

