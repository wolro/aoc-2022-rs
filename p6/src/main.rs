/// Advent of Code day 6
/// https://adventofcode.com/2022/day/6

fn main() {
    let datastream = include_str!("../input.txt");
    println!(
        "First start-of-packet marker is detected after {} characters.",
        find_block_index(datastream, 4).unwrap()
    );
    println!(
        "First start-of-packet marker is detected after {} characters.",
        find_block_index(datastream, 14).unwrap()
    );
}

/// Finds index of first start-of-packet marker. The solution I found requires checking if
/// a partial slice of a N character sequence contains the previous character. While
/// Rust allows to slice a String using a range, it does not allow to directly index a character
/// in a string. So my solution was to just work with a vector of characters as "ring buffer", and
/// run the actual uniqueness check on this.
fn find_block_index(test_str1: &str, block_len: usize) -> Option<usize> {
    let mut first_marker_idx: Option<usize> = None; // Store the first marker index here, when found
    for idx in 0..(test_str1.len() - (block_len - 1)) {
        // build "buffer"
        let c1 = idx;
        let c2 = idx + block_len;
        let cbuf = test_str1[c1..c2].chars().collect::<Vec<_>>();
        // Here we check if the contents of the buffer is unique, and return the
        // index of the last (newest) character relative to the full input string.
        if !(1..cbuf.len()).any(|i| cbuf[i..].contains(&cbuf[i - 1])) {
            first_marker_idx = Some(idx + block_len);
            break;
        }
    }
    first_marker_idx
}

/// Test algorithm on test strings provided in the puzzle description (part 1).
#[test]
fn find_marker_indices_in_test_strings() {
    let test_str1 = String::from("bvwbjplbgvbhsrlpgdmjqwftvncz");
    let test_str2 = String::from("nppdvjthqldpwncqszvftbrmjlhg");
    let test_str3 = String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
    let test_str4 = String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");

    assert_eq!(5, find_block_index(&test_str1, 4).unwrap());
    assert_eq!(6, find_block_index(&test_str2, 4).unwrap());
    assert_eq!(10, find_block_index(&test_str3, 4).unwrap());
    assert_eq!(11, find_block_index(&test_str4, 4).unwrap());
}

/// Test algorithm on test strings provided in the puzzle description (part 2).
#[test]
fn find_message_indices_in_test_strings() {
    let test_str1 = String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
    let test_str2 = String::from("bvwbjplbgvbhsrlpgdmjqwftvncz");
    let test_str3 = String::from("nppdvjthqldpwncqszvftbrmjlhg");
    let test_str4 = String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
    let test_str5 = String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");

    assert_eq!(19, find_block_index(&test_str1, 14).unwrap());
    assert_eq!(23, find_block_index(&test_str2, 14).unwrap());
    assert_eq!(23, find_block_index(&test_str3, 14).unwrap());
    assert_eq!(29, find_block_index(&test_str4, 14).unwrap());
    assert_eq!(26, find_block_index(&test_str5, 14).unwrap());
}
