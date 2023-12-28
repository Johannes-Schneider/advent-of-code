use std::fs;

use crate::day12::challenge1::solve_challenge1;
use crate::day12::record_row::{RecordRow, UnfoldedRecordRowView};

mod challenge1;
mod record_row;
mod spring;

pub fn solve_day12(file_path: &str) {
    let text = fs::read_to_string(file_path).expect("given challenge file cannot be read");
    let record_rows = RecordRow::parse_all(&text).expect("the given input should be valid");

    let mut challenge1 = 0usize;
    for record_row in &record_rows {
        let view = UnfoldedRecordRowView::new(record_row, 1);
        let combinations = solve_challenge1(&view);
        // println!("{combinations}");
        challenge1 += combinations;
    }

    println!("Day12 - Challenge1: {challenge1}");

    let challenge2 = solve_challenge2_multi_threaded(&record_rows, 8);

    println!("Day12 - Challenge2: {challenge2}");
}

fn solve_challenge2_multi_threaded(rows: &Vec<RecordRow>, threads: usize) -> usize {
    crossbeam::scope(|scope| {
        let mut thread_handles = Vec::new();
        let mut next_rows_start_index = 0usize;
        let chunk_size = (rows.len() as f64 / threads as f64).ceil() as usize;

        for i in 0..threads {
            let my_chunk_size = if i + 1 == threads {
                rows.len() - next_rows_start_index
            } else {
                chunk_size
            };

            let mut my_rows = Vec::new();
            for i in next_rows_start_index..next_rows_start_index + my_chunk_size {
                my_rows.push(&rows[i]);
            }

            next_rows_start_index += my_chunk_size;

            let handle = scope.spawn(move |_| {
                let mut result = 0usize;
                for row in my_rows {
                    let view = UnfoldedRecordRowView::new(row, 5);
                    let combinations = solve_challenge1(&view);
                    result += combinations;
                }

                return result;
            });

            thread_handles.push(handle);
        }

        let mut result = 0usize;
        for handle in thread_handles {
            result += handle.join().unwrap();
        }

        return result;
    })
    .unwrap()
}
