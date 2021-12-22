use limelight::attribute;

#[attribute]
pub struct RelativePosition {
    relative_position: [f32; 2],
}

pub fn identity_quad() -> Vec<RelativePosition> {
    vec![
        RelativePosition {
            relative_position: [-1., 1.],
        },
        RelativePosition {
            relative_position: [1., 1.],
        },
        RelativePosition {
            relative_position: [-1., -1.],
        },
        RelativePosition {
            relative_position: [1., -1.],
        },
    ]
}

#[attribute]
pub struct RectPosition {
    rect_position: [f32; 2],
}

pub fn identity_rect() -> Vec<RectPosition> {
    vec![
        RectPosition {
            rect_position: [0., 0.]
        },
        RectPosition {
            rect_position: [0., 1.]
        },
        RectPosition {
            rect_position: [1., 0.]
        },
        RectPosition {
            rect_position: [1., 1.]
        },
    ]
}


#[attribute]
pub struct LinePosition {
    // position along line (0=start, 1=end); position perpendicular to line
    line_position: [f32; 2],
    line_edge: [f32; 2],
}

pub fn identity_line() -> Vec<LinePosition> {
    vec![
        LinePosition {
            line_position: [0., -1.],
            line_edge: [0., 0.]
        },
        LinePosition {
            line_position: [0., 1.],
            line_edge: [1., 0.]
        },
        LinePosition {
            line_position: [1., -1.],
            line_edge: [0., 1.]
        },
        LinePosition {
            line_position: [1., 1.],
            line_edge: [0., 0.]
        },
    ]
}
