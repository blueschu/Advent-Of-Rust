pub mod puzzle;
pub mod util;
pub mod prelude;

macro_rules! route_days {
    ( $( $day:expr => $sol:ident ),+ ) => {
        use common::puzzle::{PuzzleSelection as Pz, PuzzleResult, SelectionError};
        pub fn route(puzzle: &Pz) -> PuzzleResult {
            match puzzle.day() {
                $( $day => $sol::solve(puzzle), )*
                _ => Err(Box::new(SelectionError::UnimplementedDay))
            }
        }
    };
}

macro_rules! bench_ans {
    ( $ans:expr ) => {{
        use common::puzzle::Answer;
        use std::time::Instant;

        let start = Instant::now();
        Answer::with_bench($ans, Some(Instant::now().duration_since(start)))
    }};
}

macro_rules! solve_parts {
    ( 1 => $part_one:expr ) => { Ok(Solution(Some(bench_ans!($part_one)), None)) };

    ( 1 => $part_one:expr, 2 => $part_two:expr ) => {
        Ok(Solution(
            Some(bench_ans!($part_one)),
            Some(bench_ans!($part_two))
        ))
    };

   ( both => $part_producer:expr ) => {{
        use common::puzzle::Answer;
        use std::time::Instant;

        let start = Instant::now();
        let (part_one, part_two) = $part_producer;
        let bench = start.elapsed();

        Ok(Solution(
            Some(Answer::with_bench(part_one, Some(bench))),
            Some(Answer::with_bench(part_two, None))
        ))
   }}
}

#[cfg(test)]
macro_rules! assert_solution {
    ( $part_one:expr, $puzzle:expr) => {{
        use common::puzzle::{Solution, Answer};
        assert_eq! {
            Solution::new(Some(Answer::new($part_one)), None),
            solve($puzzle.as_ref()).unwrap()
        }
    }};

    ( $part_one:expr, $part_two:expr, $puzzle:expr) => {{
        use common::puzzle::{Solution, Answer};
        assert_eq! {
            Solution::new(Some(Answer::new($part_one)), Some(Answer::new($part_two))),
            solve($puzzle.as_ref()).unwrap()
        }
    }};
}
