use crate::tensor::SimpleTile;

struct PartialSimpleTile {
    dim_sizes: Vec<u32>,
}

impl PartialSimpleTile {
    pub fn new(dim_sizes: Vec<u32>) -> Self {
        PartialSimpleTile { dim_sizes }
    }
}

// FIXME: Assuming SimpleTile, not Tile
pub fn tile_to_partial(tile: SimpleTile) -> PartialSimpleTile {
    let shape = tile.dim_sizes();
    PartialSimpleTile::new(shape)
}
