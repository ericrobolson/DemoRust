use crate::math::*;
use core::ops::Sub;

#[allow(dead_code)]
type Aabb<N> = Aabb2d<N>;

/// 2d representation of an AABB
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aabb2d<N> {
    pub min: [N; 2],
    pub max: [N; 2],
}
impl<N> Aabb<N>
where
    N: PartialEq + Copy + PartialOrd + Eq + Sub<Output = N> + From<i64>,
{
    /// Checks the aabb for a collision.
    pub fn check_collision(&self, other: &Self) -> Option<Manifold<N>> {
        check_collision(self, &other)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Manifold<N> {
    pub depth: N,
    pub normal: [N; 2],
}

fn check_collision<N>(a: &Aabb<N>, b: &Aabb<N>) -> Option<Manifold<N>>
where
    N: PartialEq + Copy + PartialOrd + Eq + Sub<Output = N> + From<i64>,
{
    let x_overlap: N = max(0.into(), min(a.max[0], b.max[0]) - max(a.min[0], b.min[0]));
    let y_overlap: N = max(0.into(), min(a.max[1], b.max[1]) - max(a.min[1], b.min[1]));

    if x_overlap > 0.into() && y_overlap > 0.into() {
        let normal = [
            if a.max[0] < b.min[0] {
                1.into()
            } else {
                (-1).into()
            },
            if a.max[1] < b.min[1] {
                1.into()
            } else {
                (-1).into()
            },
        ];
        let depth = if a == b {
            0.into()
        } else {
            min(x_overlap, y_overlap).into()
        };
        Some(Manifold { depth, normal })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_overlapping_aabbs_returns_none() {
        let a = Aabb::<DNum> {
            min: [0.into(), 0.into()],
            max: [4.into(), 4.into()],
        };

        let b = Aabb {
            min: [10.into(), 10.into()],
            max: [14.into(), 14.into()],
        };

        let expected = None;
        assert_eq!(expected, check_collision(&a, &b))
    }
    #[test]
    fn overlapping_returns_a() {
        let a = Aabb::<DNum> {
            min: [0.into(), 0.into()],
            max: [4.into(), 4.into()],
        };

        let b = Aabb {
            min: [2.into(), 2.into()],
            max: [5.into(), 5.into()],
        };

        let expected = Some(Manifold {
            depth: 2.into(),
            normal: [(-1).into(), (-1).into()],
        });
        assert_eq!(expected, check_collision(&a, &b))
    }

    #[test]
    fn same_aabbs_returns_overlapping() {
        let a = Aabb::<DNum> {
            min: [0.into(), 0.into()],
            max: [4.into(), 4.into()],
        };

        let b = a;

        let expected = Some(Manifold {
            depth: 0.into(),
            normal: [(-1).into(), (-1).into()],
        });
        assert_eq!(expected, check_collision(&a, &b))
    }
}
