use quill_prototype::BlockPosition;
use std::f32::consts::PI;
use crate::util::blockpos;
use std::thread::yield_now;

/*
This circle uses Bresenham's Circle Algorithm for drawing the circle. It works based on starting at
(0, radius) to plot the first point. Then increment the x value. A "decision parameter" is held and
used for determining when to decrement the y value. Using 8-point symmetry, we can draw 8 points in
one cycle. If the circle is to be filled, a line is drawn between two points across from each other,
filling at most 4 rows/columns per cycle making this algorithm very efficient.
 */
pub fn circle(radius: u32, filled: bool, origin: &BlockPosition) -> Vec<BlockPosition> {
    let mut vecs: Vec<BlockPosition> = Vec::new();

    let mut x_diff = 0;
    let mut z_diff = radius as i32;
    let mut decision = 3 - (2 * radius) as i32;

    while z_diff >= x_diff {

        vecs.push(blockpos(origin.x + x_diff, origin.y, origin.z + z_diff));
        vecs.push(blockpos(origin.x + x_diff, origin.y, origin.z - z_diff));

        vecs.push(blockpos(origin.x + z_diff, origin.y, origin.z + x_diff));
        vecs.push(blockpos(origin.x + z_diff, origin.y, origin.z - x_diff));

        vecs.push(blockpos(origin.x - z_diff, origin.y, origin.z + x_diff));
        vecs.push(blockpos(origin.x - z_diff, origin.y, origin.z - x_diff));

        vecs.push(blockpos(origin.x - x_diff, origin.y, origin.z + z_diff));
        vecs.push(blockpos(origin.x - x_diff, origin.y, origin.z - z_diff));

        if filled {
            for i in 1..(2 * z_diff - 1) {
                vecs.push(blockpos(origin.x + x_diff, origin.y, origin.z + z_diff - i));
                vecs.push(blockpos(origin.x - x_diff, origin.y, origin.z + z_diff - i));
            }

            for i in 1..(2 * x_diff - 1) {
                vecs.push(blockpos(origin.x + z_diff, origin.y, origin.z + x_diff - i));
                vecs.push(blockpos(origin.x - z_diff, origin.y, origin.z + x_diff - i));
            }
        }

        x_diff += 1;
        if decision > 0 {
            z_diff -= 1;
            decision = decision + 4 * (x_diff - z_diff) + 10;
        } else {
            decision = decision + 4 * x_diff + 6;
        }
    }

    vecs
}

pub fn cyl(radius: u32, height: u32, filled: bool, origin: &BlockPosition) -> Vec<BlockPosition> {
    let mut vecs: Vec<BlockPosition> = Vec::new();
    for i in 0..height {
        vecs.extend(circle(radius, filled, &blockpos(origin.x, origin.y + i as i32, origin.z)));
    }
    vecs
}

pub fn rec_prism(x: i32, y: i32, z: i32, origin: &BlockPosition) -> Vec<BlockPosition> {
    let mut vecs: Vec<BlockPosition> = Vec::new();

    for a in 0..(x + 1) {
        for b in 0..(z + 1) {
            for c in 0..y {
                vecs.push(BlockPosition {
                    x: (a + origin.x),
                    y: (c + origin.y),
                    z: (b + origin.z)
                });
            }
        }
    }

    vecs
}

pub fn hrec_prism(x: i32, y: i32, z: i32, floor: bool, ceiling: bool, origin: &BlockPosition) -> Vec<BlockPosition> {
    let mut vecs: Vec<BlockPosition> = Vec::new();

    // this function draws the walls
    for y_inc in 0..y {
        // walls iterating over x with z constant
        for i in 0..(x + 1) {
            vecs.push(BlockPosition { x: (i + origin.x), y: (y_inc + origin.y), z: origin.z });
            vecs.push(BlockPosition { x: (i + origin.x), y: (y_inc + origin.y), z: (z + origin.z) });
        }

        // walls iterating over z with x constant
        for i in 0..(z + 1) {
            vecs.push(BlockPosition { x: origin.x, y: (y_inc + origin.y), z: (i + origin.z) });
            vecs.push(BlockPosition { x: (x + origin.x), y: (y_inc + origin.y), z: (i + origin.z) });
        }
    }

    // don't want to run some loops if both are false
    if floor || ceiling {
        // this draws the top and bottom plates (floor and ceiling)
        for a in 0..(z + 1) {
            for b in 0..(x + 1) {
                if floor {
                    vecs.push(BlockPosition { x: (b + origin.x), y: origin.y, z: (a + origin.z) });
                }

                if ceiling {
                    vecs.push(BlockPosition { x: (b + origin.x), y: (y + origin.y - 1), z: (a + origin.z) });
                }
            }
        }
    }

    vecs
}

pub fn sphere(radius: u32, filled: bool, origin: &BlockPosition) -> Vec<BlockPosition> {
    let mut vecs: Vec<BlockPosition> = Vec::new();

    //todo this is still kinda hard, need a 3d version of Bresenham's Circle Algorithm

    vecs
}