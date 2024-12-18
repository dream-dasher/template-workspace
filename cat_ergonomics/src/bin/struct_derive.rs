//!

use std::{fmt::Display, ops::Deref};

use derive_more::derive::{Add, From, Index, Into};
use filter::LevelFilter;
use layer::SubscriberExt;
use tracing::{Level, event, instrument};
use tracing_subscriber::{EnvFilter, Layer, *};
use util::SubscriberInitExt;

#[derive(Add, Debug, Copy, Clone, derive_more::Display, From, Into)]
#[display("(r{},c{})", x, y)]
#[from(forward)]
// #[into(owned, ref(i32), ref_mut)]
struct Point2D {
        x: usize,
        y: usize,
}

#[derive(Index, Debug, Clone, From, Into)]
struct Maze {
        #[index]
        maze_linear: Vec<usize>,
        max_dims:    Point2D,
}
impl Display for Maze {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                writeln!(f)?;
                for i in 0..self.max_dims.y {
                        for j in 0..self.max_dims.x {
                                write!(f, "{} ", self.maze_linear[i * self.max_dims.x + j])?;
                        }
                        writeln!(f)?;
                }
                Ok(())
        }
}
impl Maze {
        #[instrument]
        fn new(max_dims: Point2D) -> Self {
                let maze_linear = vec![0; max_dims.x * max_dims.y];
                Self { maze_linear, max_dims }
        }

        #[instrument(skip(self))]
        fn get(&self, p: Point2D) -> Option<usize> {
                event![Level::DEBUG, %p];
                if p.x >= self.max_dims.x || p.y >= self.max_dims.y {
                        None
                } else {
                        let index = p.y * self.max_dims.x + p.x;
                        event![Level::TRACE, ?index, "p.y * self.max_dims.x + p.x"];
                        Some(self.maze_linear[p.y * self.max_dims.x + p.x])
                }
        }

        #[instrument(skip(self))]
        fn set(&mut self, p: Point2D, val: usize) -> Option<()> {
                event![Level::DEBUG, %p, %val];

                if p.x >= self.max_dims.x || p.y >= self.max_dims.y {
                        None
                } else {
                        let index = p.y * self.max_dims.x + p.x;
                        event![Level::TRACE, ?index, "index = p.y * self.max_dims.x + p.x;"];

                        self.maze_linear[p.y * self.max_dims.x + p.x] = val;
                        Some(())
                }
        }
}
fn main() {
        tracing_subscriber::Registry::default()
                .with(tracing_subscriber::fmt::Layer::default().with_filter(
                        EnvFilter::builder()
                                .with_default_directive(LevelFilter::TRACE.into())
                                .from_env_lossy(),
                ))
                .init();
        let p1 = Point2D { x: 1, y: 2 };
        let p2 = Point2D { x: 3, y: 4 };
        let p3 = p1 + p2;
        println!("p3: {}", p3);

        let p4 = Point2D { x: 5, y: 6 };
        let x: (usize, usize) = (1, 2);
        let px: Point2D = x.into();
        let px4 = p4 + px;
        let (a, b) = px4.into();
        event![Level::INFO, %px4, ?x, a,b ,"results of adds"];

        let maze_dims = Point2D { x: 6, y: 6 };
        // let maze_points: Vec<usize> = (0..36).map(|i| i % 10).collect();
        let maze_points: Vec<usize> = (0..36).collect();
        let maze = Maze {
                maze_linear: maze_points,
                max_dims:    maze_dims,
        };

        println!("maze: {}", maze);

        let p = Point2D { x: 3, y: 4 };
        let val = maze.get(p).unwrap();
        event![Level::INFO, %maze, %p, %val ,"maze indexing"];
}
