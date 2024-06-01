use std::cell::RefCell;
use std::rc::Rc;

use nalgebra::Vector2;

use isotope::constraints::angle_between_points::AngleBetweenPoints;
use isotope::constraints::distance::euclidian_distance_between_points::EuclidianDistanceBetweenPoints;
use isotope::constraints::fix_point::FixPoint;
use isotope::constraints::ConstraintCell;
use isotope::primitives::line::Line;
use isotope::primitives::point2::Point2;
use isotope::primitives::PrimitiveCell;
use isotope::sketch::Sketch;

fn circle(n: usize) -> Vec<Vector2<f64>> {
    let mut points = Vec::new();
    for i in 0..n {
        let x = ((i + 1) / 2) as f64 * 0.8;
        let y = ((i + 0) / 2) as f64 * 0.8;
        points.push(Vector2::new(x, y));
    }
    points
}

pub(crate) fn new_benchmark(n: usize) -> Sketch {
    let reference_points = circle(n);

    let mut sketch = Sketch::new();

    let mut point_references = Vec::new();
    for i in 0..n {
        let point = Rc::new(RefCell::new(Point2::new(0.0, (i as f64) / (n as f64))));
        sketch
            .add_primitive(PrimitiveCell::Point2(point.clone()))
            .unwrap();
        point_references.push(point);
    }

    for i in 0..n {
        sketch
            .add_constraint(ConstraintCell::FixPoint(Rc::new(RefCell::new(
                FixPoint::new(
                    point_references[i].clone(),
                    Vector2::new(reference_points[i].x, reference_points[i].y),
                ),
            ))))
            .unwrap();
    }

    for i in 0..n - 1 {
        let line = Rc::new(RefCell::new(Line::new(
            point_references[i].clone(),
            point_references[i + 1].clone(),
        )));
        sketch
            .add_primitive(PrimitiveCell::Line(line.clone()))
            .unwrap();

        let distance = (reference_points[i + 1] - reference_points[i]).norm();
        sketch
            .add_constraint(ConstraintCell::EuclideanDistance(Rc::new(RefCell::new(
                EuclidianDistanceBetweenPoints::new(
                    point_references[i].clone(),
                    point_references[i + 1].clone(),
                    distance,
                ),
            ))))
            .unwrap();

        let angle = (&reference_points[i + 1] - &reference_points[i])
            .angle(&(&reference_points[(i + n - 1) % n] - &reference_points[i]));
        sketch
            .add_constraint(ConstraintCell::AngleBetweenPoints(Rc::new(RefCell::new(
                AngleBetweenPoints::new(
                    point_references[i + 1].clone(),
                    point_references[(i + n - 1) % n].clone(),
                    point_references[i].clone(),
                    angle,
                ),
            ))))
            .unwrap();
    }

    sketch
}
