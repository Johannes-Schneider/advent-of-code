use crate::day12::record_row::RecordRowView;
use crate::day12::spring::Spring;

pub fn solve_challenge1(record_row: &dyn RecordRowView) -> usize {
    return count_valid_combinations(record_row, 0, 0, 0);
}

fn count_valid_combinations(
    row: &dyn RecordRowView,
    current_spring_index: usize,
    current_group_index: usize,
    current_group_length: usize,
) -> usize {
    let number_of_groups = row.number_of_groups();
    let number_of_springs = row.number_of_springs();

    if current_spring_index >= number_of_springs {
        if current_group_index >= number_of_groups && current_group_length == 0 {
            // we closed the last group before we reached the end of our sequence
            return 1;
        }

        if current_group_index + 1 == number_of_groups
            && current_group_length == row.get_group(current_group_index)
        {
            // we closed the last group right at the end of our sequence
            return 1;
        }

        return 0;
    }

    let current_spring = row.get_spring(current_spring_index);
    let springs = if let Spring::Unknown = current_spring {
        vec![&Spring::Broken, &Spring::Functioning]
    } else {
        vec![current_spring]
    };

    let mut count = 0usize;
    for spring in springs {
        if let Spring::Broken = spring {
            if current_group_index >= number_of_groups {
                // too many groups
                continue;
            }

            let next_current_group_length = current_group_length + 1;
            if next_current_group_length > row.get_group(current_group_index) {
                // current group is too large
                continue;
            }

            count += count_valid_combinations(
                row,
                current_spring_index + 1,
                current_group_index,
                next_current_group_length,
            );
        } else {
            if current_group_length > 0 {
                // our current group is finished

                if current_group_length != row.get_group(current_group_index) {
                    // our group didn't match the expectation
                    continue;
                }

                count += count_valid_combinations(
                    row,
                    current_spring_index + 1,
                    current_group_index + 1,
                    0,
                );
            } else {
                // we are in no group
                count +=
                    count_valid_combinations(row, current_spring_index + 1, current_group_index, 0);
            }
        }
    }

    return count;
}

#[cfg(test)]
mod tests {
    use crate::day12::challenge1::solve_challenge1;
    use crate::day12::record_row::{RecordRow, UnfoldedRecordRowView};

    #[test]
    fn test_challenge1_001() {
        let input = "???.### 1,1,3";
        assert_challenge1(input, 1);
    }

    #[test]
    fn test_challenge1_002() {
        let input = ".??..??...?##. 1,1,3";
        assert_challenge1(input, 4);
    }

    #[test]
    fn test_challenge1_003() {
        let input = "?#?#?#?#?#?#?#? 1,3,1,6";
        assert_challenge1(input, 1);
    }

    #[test]
    fn test_challenge1_004() {
        let input = "????.#...#... 4,1,1";
        assert_challenge1(input, 1);
    }

    #[test]
    fn test_challenge1_005() {
        let input = "????.######..#####. 1,6,5";
        assert_challenge1(input, 4);
    }

    #[test]
    fn test_challenge1_006() {
        let input = "?###???????? 3,2,1";
        assert_challenge1(input, 10);
    }

    #[test]
    fn test_challenge1_007() {
        let input = "?????????? 1";
        assert_challenge1(input, 10);
    }

    #[test]
    fn test_challenge1_008() {
        let input = "?#??????#?????#?? 10,2";
        assert_challenge1(input, 4);
    }

    #[test]
    fn test_challenge1_009() {
        let input = "?.?.????.?# 1,2";
        assert_challenge1(input, 6);
    }

    #[test]
    fn test_challenge1_010() {
        let input = "#?#???????#?.? 3,1,2,2";
        assert_challenge1(input, 4);
    }

    #[test]
    fn test_challenge1_011() {
        let input = "??????????#.. 6,1";
        assert_challenge1(input, 4);
    }

    #[test]
    fn test_challenge1_012() {
        let input = "???.??#.????? 1,1,1,2";
        assert_challenge1(input, 28);
    }

    #[test]
    fn test_challenge2_001() {
        let input = "???.### 1,1,3";
        assert_challenge2(input, 1);
    }

    #[test]
    fn test_challenge2_002() {
        let input = ".??..??...?##. 1,1,3";
        assert_challenge2(input, 16_384);
    }

    #[test]
    fn test_challenge2_003() {
        let input = "?#?#?#?#?#?#?#? 1,3,1,6";
        assert_challenge2(input, 1);
    }

    #[test]
    fn test_challenge2_004() {
        let input = "????.#...#... 4,1,1";
        assert_challenge2(input, 16);
    }

    #[test]
    fn test_challenge2_005() {
        let input = "????.######..#####. 1,6,5";
        assert_challenge2(input, 2_500);
    }

    #[test]
    fn test_challenge2_006() {
        let input = "?###???????? 3,2,1";
        assert_challenge2(input, 506_250);
    }

    fn assert_challenge1(input: &str, expected: usize) {
        let record_row = RecordRow::parse(input).unwrap();
        let view = UnfoldedRecordRowView::new(&record_row, 1);

        assert_eq!(solve_challenge1(&view), expected);
    }

    fn assert_challenge2(input: &str, expected: usize) {
        let record_row = RecordRow::parse(input).unwrap();
        let view = UnfoldedRecordRowView::new(&record_row, 5);

        assert_eq!(solve_challenge1(&view), expected);
    }
}
