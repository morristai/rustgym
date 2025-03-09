//! https://leetcode.com/problems/meeting-rooms/description/
//! NOTE: required premium to run test

use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct Solution;

impl Solution {
    fn min_meeting_rooms(mut intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.is_empty() {
            return 0;
        }
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut heap = BinaryHeap::new();
        heap.push(Reverse(intervals[0][1]));
        let mut rst = 0;
        for interval in intervals {
            if let Some(Reverse(min_val)) = heap.pop() {
                if interval[0] < min_val {
                    rst += 1;
                    heap.push(Reverse(min_val));
                }
                heap.push(Reverse(interval[1]));
            }
        }
        rst
    }
}

#[test]
fn test() {
    let intervals: Vec<Vec<i32>> = vec_vec_i32![[0, 30], [5, 10], [15, 20]];
    assert_eq!(Solution::min_meeting_rooms(intervals), 2);
    let intervals: Vec<Vec<i32>> = vec_vec_i32![[7, 10], [2, 4]];
    assert_eq!(Solution::min_meeting_rooms(intervals), 1);
    let intervals: Vec<Vec<i32>> = vec_vec_i32![[13, 15], [1, 13]];
    assert_eq!(Solution::min_meeting_rooms(intervals), 1);
}
