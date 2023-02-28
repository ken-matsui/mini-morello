pub mod utils;
// pub mod actions;
// pub mod matmuls;

// use crate::specs::tensor_spec::TensorSpec;
use crate::specs::Spec;
use crate::tiling;
// use crate::tensor::Tile;
//
// use std::collections::HashSet;

use anyhow::{bail, Result};

pub trait Impl {
    fn children(&self) -> Vec<Box<dyn Impl>>;

    fn replace_children(&self, replacements: Vec<Box<dyn Impl>>) -> Box<dyn Impl>;

    fn spec(&self) -> Box<dyn Spec>;

    fn tile_out(&self, output_shape: (i32, i32)) -> Result<Box<dyn Impl>> {
        // If this is an Impl with a single child, just forward.
        if self.children().len() == 1 {
            return Ok(
                self.replace_children(vec![Box::new(self.children()[0].tile_out(output_shape))])
            );
        }

        if output_shape.len() != self.spec().output().dim_sizes.len() {
            bail!(
                "Expected {} dimensions; got {}",
                self.spec().output().dim_sizes.len(),
                output_shape.len()
            );
        }
        for (dim, dim_size) in output_shape.iter().enumerate() {
            if dim_size <= 0 {
                bail!("All dimensions must be size 1 or greater, but given output shape {output_shape}");
            } else if dim_size > self.spec().output().dim_sizes[dim] {
                bail!(
                    "Dimensions {dim} was larger than {0} ({dim_size} > {0})",
                    self.spec().output().dim_sizes[dim]
                );
            }
        }

        // A no-op if the given shape is already the output shape.
        if self.spec().output().dim_sizes == output_shape {
            return Ok(Box::new(self.clone()));
        }

        // Tile the output and inputs.
        // smaller_output = self.spec.output.simple_tile(
        //             OperandIdx(len(self.spec.inputs)), output_shape
        //         )
        //         smaller_inputs = [
        //             partial_tile.tile(OperandIdx(input_idx), inp)
        //             for input_idx, (inp, partial_tile) in enumerate(
        //                 zip(
        //                     self.spec.inputs,
        //                     self._calculate_partial_inputs_for_tile_out(
        //                         tiling.tile_to_partial(smaller_output)
        //                     ),
        //                 )
        //             )
        //         ]
        let smaller_output = self
            .spec()
            .output()
            .simple_tile(OperandIdx(self.spec().inputs().len()), output_shape);
        let mut smaller_inputs = vec![];
        for (input_idx, ()) in self
            .spec()
            .inputs()
            .iter()
            .zip(
                self._calculate_partial_inputs_for_tile_out(tiling::tile_to_partial(
                    smaller_output,
                ))
                .iter(),
            )
            .enumerate()
        {}

        // Make an inner hole for the now-smaller Spec.
        //         inner = spec_to_hole(
        //             self.spec.replace_io(
        //                 tuple(x.spec for x in smaller_inputs),
        //                 smaller_output.spec,
        //                 serial_only=inner_serial,
        //             )
        //         )

        // Construct the list of tiles, which is every tile_out result that just
        // returned itself because the tile would have been the same shape as the
        // original. (Note that the output, unlike the inputs, is known to have
        // been tiled or this method would have short-circuited.)
        // TODO: Can we share this code with same block in ComposeHole.tile_out?

        Box::new()
    }
}

// pub struct Impl {
//     spec: Spec,
//     children: Vec<Box<Impl>>,
//     // Loop {
//     //     spec: Box<Impl>,
//     //     subscripts: Vec<i32>,
//     //     _operands_subscripts: Vec<Vec<i32>>,
//     //     tiles: HashSet<Tile>,
//     //     inner_args: HashSet<Tile>,
//     //     inner: Box<Impl>,
//     //     parallel: bool,
//     // },
//     // Move {
//     //     src: TensorSpec,
//     //     dst: TensorSpec,
//     // },
// // }
