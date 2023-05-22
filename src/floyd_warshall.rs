use std::cmp::min;

use crate::segment::Segment;
//Ensure sorted by id
pub(crate) fn segments_dist_shortest_vec(segments: &mut Vec<&Segment>) -> Vec<Vec<u32>> {
    let n = segments.len();

    let mut dist: Vec<Vec<u32>> = Vec::new();
    segments.sort_by_key(|a| a.id());
    for segment in segments {
        dist.push(segment_distance_vec(segment, n));
    }
    floyd_warshall_fast(&mut dist);
    dist
}

pub(crate) fn segment_distance_vec(segment: &Segment, n: usize) -> Vec<u32> {
    let mut distance = vec![std::u32::MAX / 3; n];
    for pred in segment.precedence().borrow().clone() {
        let duration = pred.duration();
        let access_id = pred.id();
        distance[access_id as usize] = duration;
    }
    let segment_access_id = segment.id();
    distance[segment_access_id as usize] = 0;
    distance
}

pub(crate) fn floyd_warshall_fast(dist: &mut [Vec<u32>]) {
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
