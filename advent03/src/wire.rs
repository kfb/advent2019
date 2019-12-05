use super::point::Point;

#[derive(Debug, PartialEq)]
pub struct WireSegment {
    src: Point,
    dst: Point,
}

impl WireSegment {
    pub fn new(sx: f32, sy: f32, dx: f32, dy: f32) -> WireSegment {
        let src = Point {x: sx, y: sy};
        let dst = Point {x: sx + dx, y: sy + dy};

        WireSegment {src: src, dst: dst}
    }

    pub fn contains_point(&self, p: &Point) -> bool {
        let ab = self.offset();
        let ac = p.subtract(&self.src);

        if ab.cross(&ac) != 0.0 {
            return false;
        }

        let ab_dot_ab = ab.dot(&ab);
        let ab_dot_ac = ab.dot(&ac);

        if (ab_dot_ac == 0.0) || (ab_dot_ac == ab_dot_ab) {
            return true;
        } else if (ab_dot_ac > 0.0) && (ab_dot_ac < ab_dot_ab) {
            return true;
        }

        false
    }

    pub fn intersect(&self, other: &WireSegment) -> Option<Point> {
        let p = &self.src;
        let q = &other.src;
        let r = self.offset();
        let s = other.offset();

        let q_minus_p = q.subtract(p);
        let r_cross_s = r.cross(&s);

        let t = q_minus_p.cross(&s) / r_cross_s;
        let u = q_minus_p.cross(&r) / r_cross_s;

        if (t >= 0.0) && (t <= 1.0) && (u >= 0.0) && (u <= 1.0) {
            Some(p.add(&r.scale(t)))
        } else {
            None
        }
    }

    pub fn len(&self) -> f32 {
        self.dst.manhattan(&self.src)
    }

    pub fn manhattan(&self, p: &Point) -> f32 {
        self.src.manhattan(&p)
    }

    pub fn offset(&self) -> Point {
        Point {
            x: (self.dst.x - self.src.x),
            y: (self.dst.y - self.src.y),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Wire {
    segments: Vec<WireSegment>
}

impl Wire {
    pub fn new() -> Wire {
        Wire {segments: vec![]}
    }

    pub fn intersection_points(&self, other: &Wire) -> Vec<Point> {
        let mut result = vec![];

        for self_segment in &self.segments {
            for other_segment in &other.segments {
                match self_segment.intersect(&other_segment) {
                    Some(p) => {
                        if (p.x != 0.0) || (p.y != 0.0) {
                            result.push(p);
                        }
                    },
                    None => (),
                }
            }
        }

        result
    }

    pub fn push(&mut self, segment: WireSegment) {
        self.segments.push(segment);
    }

    pub fn segments(&self) -> &Vec<WireSegment> {
        &self.segments
    }
}
