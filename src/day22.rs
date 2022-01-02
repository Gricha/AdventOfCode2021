use crate::utils::read_input;
use itertools::Itertools;
use std::cmp::{max, min};

pub fn day22_1() {
    let input = read_input("./day22/input")
        .into_iter()
        .map(|x| {
            let (op, x1, x2, y1, y2, z1, z2) = scan_fmt!(
                &x,
                "{} x={}..{},y={}..{},z={}..{}",
                String,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32
            )
            .unwrap();
            let on_off = if op == "on" { true } else { false };
            (on_off, (x1, x2), (y1, y2), (z1, z2))
        })
        .collect_vec();

    let mut values = [[[0; 101]; 101]; 101];

    for (op, (xmin, xmax), (ymin, ymax), (zmin, zmax)) in input {
        if xmin > 50 || xmin < -50 {
            continue;
            // this data is not containing anything within the block
        }

        for x in xmin..=xmax {
            for y in ymin..=ymax {
                for z in zmin..=zmax {
                    values[(x + 50) as usize][(y + 50) as usize][(z + 50) as usize] = op as i32;
                }
            }
        }
    }

    let mut count = 0;
    for x in 0..=100 {
        for y in 0..=100 {
            for z in 0..=100 {
                count += values[x as usize][y as usize][z as usize];
            }
        }
    }

    println!("{:?}", count);
}

#[derive(Debug)]
struct Cube {
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32,
    zmin: i32,
    zmax: i32,

    on_or_off: bool,
}

impl Cube {
    fn size(&self) -> i64 {
        (self.xmax - self.xmin) as i64
            * (self.ymax - self.ymin) as i64
            * (self.zmax - self.zmin) as i64
    }

    fn intersect(&self, other: &Cube) -> Option<Vec<Cube>> {
        if (self.xmin > other.xmax) || other.xmin > self.xmax {
            return None;
        }
        if (self.ymin > other.ymax) || other.ymin > self.ymax {
            return None;
        }
        if (self.zmin > other.zmax) || other.zmin > self.zmax {
            return None;
        }

        let intersecting_cube = Cube {
            xmin: max(self.xmin, other.xmin),
            xmax: min(self.xmax, other.xmax),
            ymin: max(self.ymin, other.ymin),
            ymax: min(self.ymax, other.ymax),
            zmin: max(self.zmin, other.zmin),
            zmax: min(self.zmax, other.zmax),
            on_or_off: other.on_or_off,
        };

        if (intersecting_cube.xmax <= intersecting_cube.xmin)
            || (intersecting_cube.ymax <= intersecting_cube.ymin)
            || (intersecting_cube.zmax <= intersecting_cube.zmin)
        {
            return None;
        }

        let x_ranges = vec![
            (self.xmin, intersecting_cube.xmin),
            (intersecting_cube.xmin, intersecting_cube.xmax),
            (intersecting_cube.xmax, self.xmax),
        ];
        let y_ranges = vec![
            (self.ymin, intersecting_cube.ymin),
            (intersecting_cube.ymin, intersecting_cube.ymax),
            (intersecting_cube.ymax, self.ymax),
        ];
        let z_ranges = vec![
            (self.zmin, intersecting_cube.zmin),
            (intersecting_cube.zmin, intersecting_cube.zmax),
            (intersecting_cube.zmax, self.zmax),
        ];

        let mut other_cubes = Vec::new();

        for (i, x) in x_ranges.iter().enumerate() {
            for (j, y) in y_ranges.iter().enumerate() {
                for (k, z) in z_ranges.iter().enumerate() {
                    if i == 1 && j == 1 && k == 1 {
                        // ignore, this is the `intersecting_cube`
                        continue;
                    }
                    if (x.1 - x.0 <= 0) || (y.1 - y.0 <= 0) || (z.1 - z.0 <= 0) {
                        // lol obv
                        continue;
                    }
                    let new_cube = Cube {
                        xmin: x.0,
                        xmax: x.1,
                        ymin: y.0,
                        ymax: y.1,
                        zmin: z.0,
                        zmax: z.1,
                        on_or_off: true,
                    };
                    other_cubes.push(new_cube);
                }
            }
        }

        Some(other_cubes)
    }
}

pub fn day22() {
    let input = read_input("./day22/input")
        .into_iter()
        .map(|x| {
            let (op, x1, x2, y1, y2, z1, z2) = scan_fmt!(
                &x,
                "{} x={}..{},y={}..{},z={}..{}",
                String,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32
            )
            .unwrap();
            let on_off = if op == "on" { true } else { false };
            Cube {
                xmin: x1,
                xmax: x2 + 1,
                ymin: y1,
                ymax: y2 + 1,
                zmin: z1,
                zmax: z2 + 1,
                on_or_off: on_off,
            }
        })
        .collect_vec();

    let mut final_cubes: Vec<Cube> = Vec::new();

    for i in input {
        let mut temp_final = Vec::new();
        for v in final_cubes.into_iter() {
            if let Some(intersections) = v.intersect(&i) {
                // skip first, add new ones
                temp_final.extend(intersections.into_iter());
            } else {
                // push it back
                temp_final.push(v);
            }
        }
        if i.on_or_off {
            temp_final.push(i);
        }
        final_cubes = temp_final;
    }

    let result = final_cubes.into_iter().map(|x| x.size()).sum::<i64>();

    println!("{}", result);
}
