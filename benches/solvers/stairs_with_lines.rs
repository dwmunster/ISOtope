use isotope::constraints::distance::horizontal_distance_between_points::HorizontalDistanceBetweenPoints;
use isotope::constraints::distance::vertical_distance_between_points::VerticalDistanceBetweenPoints;
use isotope::constraints::fix_point::FixPoint;
use isotope::constraints::lines::horizontal_line::HorizontalLine;
use isotope::constraints::lines::vertical_line::VerticalLine;
use isotope::constraints::ConstraintCell;
use isotope::primitives::line::Line;
use isotope::primitives::point2::Point2;
use isotope::primitives::PrimitiveCell;
use isotope::sketch::Sketch;
use nalgebra::Vector2;
use std::cell::RefCell;
use std::rc::Rc;

pub(crate) fn new_benchmark(n: usize) -> Sketch {
    let mut sketch = Sketch::new();

    let mut point_references = Vec::new();
    for _i in 0..n {
        let point = Rc::new(RefCell::new(Point2::new(0.0, 0.0)));
        sketch
            .add_primitive(PrimitiveCell::Point2(point.clone()))
            .unwrap();
        point_references.push(point);
    }

    sketch
        .add_constraint(ConstraintCell::FixPoint(Rc::new(RefCell::new(
            FixPoint::new(point_references[0].clone(), Vector2::new(0.0, 0.0)),
        ))))
        .unwrap();

    for i in 0..n - 1 {
        let line = Rc::new(RefCell::new(Line::new(
            point_references[i].clone(),
            point_references[i + 1].clone(),
        )));
        sketch
            .add_primitive(PrimitiveCell::Line(line.clone()))
            .unwrap();

        if i % 2 == 0 {
            sketch
                .add_constraint(ConstraintCell::HorizontalDistance(Rc::new(RefCell::new(
                    HorizontalDistanceBetweenPoints::new(
                        point_references[i].clone(),
                        point_references[i + 1].clone(),
                        0.8,
                    ),
                ))))
                .unwrap();

            sketch
                .add_constraint(ConstraintCell::HorizontalLine(Rc::new(RefCell::new(
                    HorizontalLine::new(line.clone()),
                ))))
                .unwrap();
        } else {
            sketch
                .add_constraint(ConstraintCell::VerticalDistance(Rc::new(RefCell::new(
                    VerticalDistanceBetweenPoints::new(
                        point_references[i].clone(),
                        point_references[i + 1].clone(),
                        0.8,
                    ),
                ))))
                .unwrap();

            sketch
                .add_constraint(ConstraintCell::VerticalLine(Rc::new(RefCell::new(
                    VerticalLine::new(line.clone()),
                ))))
                .unwrap();
        }
    }

    sketch
}
