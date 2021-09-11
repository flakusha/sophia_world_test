use std::collections::HashSet;

use crate::RectObj;

pub fn display_rect_areas(rects: &Vec<RectObj>) {
    for rect in rects {
        println!("Rectangle: {} area: {}", rect.name,
        calculate_rect_area(rect.width, rect.height));
    }
}

/// Detects overlapping pairs of rectangles and returns Vector with them.
pub fn detect_overlapping_pairs(rects: &Vec<RectObj>) ->
Vec<(RectObj, RectObj, f32)> {
    let rlen = rects.len();
    // Construct HashSet with indices of intersecting rectangles' pairs,
    // double the indices because they go in pairs like (i, j) and (j, i)
    let mut intersecting_indices = HashSet::<(usize, usize)>
    ::with_capacity(rlen * 2);
    let mut intersecting_rects = Vec::<(RectObj, RectObj, f32)>
    ::with_capacity(rlen);

    // Use index for outer loop, because of additional code complexity for
    // nested for loops
    let mut i = 0;

    while i < rlen {
        let r0 = &rects[i].clone();
        for (j, rect) in rects.iter().enumerate() {
            if i != j {
                if !intersecting_indices.contains(&(i, j))
                && !intersecting_indices.contains(&(j, i)) {
                    // Check this unique pair is not added and area
                    let overlap = compute_overlap(r0, rect);
                    intersecting_indices.insert((i, j));
                    intersecting_indices.insert((j, i));
                    if overlap > 0f32 {
                        intersecting_rects.push(
                            // Not so effective to clone, but it's made for
                            // full access to data structure later on, can be
                            // replaced just by rectangles' names
                            (r0.clone(), rect.clone(), overlap)
                        );
                    }
                }
            }
        }
        i += 1;
    }

    return intersecting_rects;
}

/// Rectangles' vertices have coordinates:
/// left = x, right = x + width, top = y, bottom = y - height.
///
/// Function returns any value > 0 if intersection is found.
pub fn compute_overlap(rc0: &RectObj, rc1: &RectObj) -> f32 {
    if rc0.width == 0f32 || rc0.height == 0f32 ||
    rc1.width == 0f32 || rc1.height == 0f32 { return 0f32; }

    let l0 = rc0.x; // r0 left
    let r0 = rc0.x + rc0.width; // r0 right
    let t0 = rc0.y; // r0 top
    let b0 = rc0.y - rc0.height; // r0 bottom
    let l1 = rc1.x;
    let r1 = rc1.x + rc1.width;
    let t1 = rc1.y;
    let b1 = rc1.y - rc1.height;
    let l = l0.max(l1);
    let r = r0.min(r1);
    let b = b0.max(b1);
    let t = t0.min(t1);

    // Check that overlap exists
    if l < r && b < t {
        let x_overlap = r - l;
        let y_overlap = t - b;
        return x_overlap * y_overlap;
    } else { return 0f32; }
}

/// Will return infinity if result > f32::MAX
pub fn calculate_rect_area(w: f32, h: f32) -> f32 {
    w * h
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlapping_rectangles_same_origin() {
        let r0 = RectObj{
            x: 0f32, y: 0f32,
            width: 2f32, height: 2f32,
            name: "Rect0".to_string(), properties: None,
        };
        let r1 = RectObj{
            x: 0f32, y: 0f32,
            width: 2f32, height: 2f32,
            name: "Rect1".to_string(), properties: None,
        };

        debug_assert_eq!(compute_overlap(&r0, &r1), 4f32);
    }

    #[test]
    fn overlapping_rectangles_offset_00() {
        let r0 = RectObj{
            x: 0f32, y: 0f32,
            width: 2f32, height: 2f32,
            name: "Rect0".to_string(), properties: None,
        };
        let r1 = RectObj{
            x: -1f32, y: 1f32,
            width: 2f32, height: 2f32,
            name: "Rect1".to_string(), properties: None,
        };

        debug_assert_eq!(compute_overlap(&r0, &r1), 1f32);
    }

    #[test]
    fn overlapping_rectangles_offset_01() {
        let r0 = RectObj{
            x: 0f32, y: 0f32,
            width: 2f32, height: 2f32,
            name: "Rect0".to_string(), properties: None,
        };
        let r1 = RectObj{
            x: -1f32, y: -1f32,
            width: 2f32, height: 2f32,
            name: "Rect1".to_string(), properties: None,
        };

        debug_assert_eq!(compute_overlap(&r0, &r1), 1f32);
    }

    #[test]
    fn overlapping_rectangles_offset_02() {
        let r0 = RectObj{
            x: 0f32, y: 0f32,
            width: 2f32, height: 2f32,
            name: "Rect0".to_string(), properties: None,
        };
        let r1 = RectObj{
            x: -1f32, y: 0f32,
            width: 2f32, height: 2f32,
            name: "Rect1".to_string(), properties: None,
        };

        debug_assert_eq!(compute_overlap(&r0, &r1), 2f32);
    }

    #[test]
    fn overlapping_rectangles_offset_03() {
        let r0 = RectObj{
            x: -1f32, y: 0f32,
            width: 2f32, height: 2f32,
            name: "Rect0".to_string(), properties: None,
        };
        let r1 = RectObj{
            x: 0f32, y: 0f32,
            width: 2f32, height: 2f32,
            name: "Rect1".to_string(), properties: None,
        };

        debug_assert_eq!(compute_overlap(&r0, &r1), 2f32);
    }

    #[test]
    fn non_overlapping_rectangles_touching() {
        let r0 = RectObj{
            x: 0f32, y: 0f32,
            width: 2f32, height: 2f32,
            name: "Rect0".to_string(), properties: None,
        };
        let r1 = RectObj{
            x: -2f32, y: -2f32,
            width: 2f32, height: 2f32,
            name: "Rect1".to_string(), properties: None,
        };

        debug_assert_eq!(compute_overlap(&r0, &r1), 0f32);
    }

    #[test]
    fn non_overlapping_rectangles_not_touching() {
        let r0 = RectObj{
            x: 0f32, y: 0f32,
            width: 2f32, height: 2f32,
            name: "Rect0".to_string(), properties: None,
        };
        let r1 = RectObj{
            x: -3f32, y: -2f32,
            width: 2f32, height: 2f32,
            name: "Rect1".to_string(), properties: None,
        };

        debug_assert_eq!(compute_overlap(&r0, &r1), 0f32);
    }

    #[test]
    fn find_overlapping_rectangles() {
        let r0 = RectObj{
            x: -1f32, y: 0f32,
            width: 2f32, height: 2f32,
            name: "Rect0".to_string(), properties: None,
        };
        let r1 = RectObj{
            x: 0f32, y: 0f32,
            width: 2f32, height: 2f32,
            name: "Rect1".to_string(), properties: None,
        };

        let rects = vec![r0, r1];
        let overlap = detect_overlapping_pairs(&rects);

        debug_assert_eq!(overlap[0].0.name, rects[0].name);
        debug_assert_eq!(overlap[0].0.width, rects[0].width);
        debug_assert_eq!(overlap[0].0.height, rects[0].height);
        debug_assert_eq!(overlap[0].0.x, rects[0].x);
        debug_assert_eq!(overlap[0].0.y, rects[0].y);
        debug_assert_eq!(overlap[0].1.name, rects[1].name);
        debug_assert_eq!(overlap[0].1.width, rects[1].width);
        debug_assert_eq!(overlap[0].1.height, rects[1].height);
        debug_assert_eq!(overlap[0].1.x, rects[1].x);
        debug_assert_eq!(overlap[0].1.y, rects[1].y);
        debug_assert_eq!(overlap[0].2, 2f32);
    }
}