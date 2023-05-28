use std::{cell::RefCell, cmp::min, rc::Rc};

use crate::segment::Segment;
//Ensure sorted by id
pub fn segments_dist_shortest_vec(segments: &Vec<Rc<RefCell<Segment>>>) -> Vec<Vec<i32>> {
    let n = segments.len();

    let mut dist: Vec<Vec<i32>> = Vec::new();
    for segment in segments {
        dist.push(segment_to_distance_vec(&segment.borrow(), n));
    }
    floyd_warshall_fast(&mut dist);
    dist
}

pub fn segment_to_distance_vec(segment: &Segment, n: usize) -> Vec<i32> {
    let mut distance = vec![std::i32::MAX / 3; n];
    for pred in segment.precedence().clone() {
        let duration = pred.borrow().duration();
        let access_id = pred.borrow().id();
        distance[access_id as usize] = duration as i32;
    }
    let segment_access_id = segment.id();
    distance[segment_access_id as usize] = 0;
    distance
}
pub fn segment_to_negative_distance_vec(segment: &Segment, n: usize) -> Vec<i32> {
    let mut distance = vec![std::i32::MAX / 3; n];
    for pred in segment.precedence().clone() {
        let duration = pred.borrow().duration();
        let access_id = pred.borrow().id();
        distance[access_id as usize] = -(duration as i32);
    }
    let segment_access_id = segment.id();
    distance[segment_access_id as usize] = 0;
    distance
}

pub fn segments_dist_longest_vec(segments: &Vec<Rc<RefCell<Segment>>>) -> Vec<Vec<i32>> {
    let n = segments.len();

    let mut dist: Vec<Vec<i32>> = Vec::new();
    for segment in segments {
        dist.push(segment_to_negative_distance_vec(&segment.borrow(), n));
    }
    floyd_warshall_fast(&mut dist);
    dist
}

pub fn floyd_warshall_fast(dist: &mut [Vec<i32>]) {
    let n = dist.len();
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let (dist_j, dist_i) = if j < i {
                let (lo, hi) = dist.split_at_mut(i);
                (&mut lo[j][..n], &mut hi[0][..n])
            } else {
                let (lo, hi) = dist.split_at_mut(j);
                (&mut hi[0][..n], &mut lo[i][..n])
            };
            let dist_ji = dist_j[i];
            for k in 0..n {
                dist_j[k] = min(dist_j[k], dist_ji + dist_i[k]);
            }
        }
    }
}
