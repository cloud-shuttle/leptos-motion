//! SVG path command definitions

/// SVG path command types
#[derive(Debug, Clone, PartialEq)]
pub enum PathCommand {
    /// Move to absolute position
    MoveTo(f64, f64),
    /// Move to relative position
    MoveToRel(f64, f64),
    /// Line to absolute position
    LineTo(f64, f64),
    /// Line to relative position
    LineToRel(f64, f64),
    /// Horizontal line to absolute position
    HorizontalLineTo(f64),
    /// Horizontal line to relative position
    HorizontalLineToRel(f64),
    /// Vertical line to absolute position
    VerticalLineTo(f64),
    /// Vertical line to relative position
    VerticalLineToRel(f64),
    /// Cubic bezier curve
    CurveTo(f64, f64, f64, f64, f64, f64),
    /// Cubic bezier curve relative
    CurveToRel(f64, f64, f64, f64, f64, f64),
    /// Smooth cubic bezier curve
    SmoothCurveTo(f64, f64, f64, f64),
    /// Smooth cubic bezier curve relative
    SmoothCurveToRel(f64, f64, f64, f64),
    /// Quadratic bezier curve
    QuadraticCurveTo(f64, f64, f64, f64),
    /// Quadratic bezier curve relative
    QuadraticCurveToRel(f64, f64, f64, f64),
    /// Smooth quadratic bezier curve
    SmoothQuadraticCurveTo(f64, f64),
    /// Smooth quadratic bezier curve relative
    SmoothQuadraticCurveToRel(f64, f64),
    /// Arc
    Arc(f64, f64, f64, bool, bool, f64, f64),
    /// Arc relative
    ArcRel(f64, f64, f64, bool, bool, f64, f64),
    /// Close path
    ClosePath,
}

impl PathCommand {
    /// Get command type as string
    pub fn command_type(&self) -> &'static str {
        match self {
            PathCommand::MoveTo(_, _) => "M",
            PathCommand::MoveToRel(_, _) => "m",
            PathCommand::LineTo(_, _) => "L",
            PathCommand::LineToRel(_, _) => "l",
            PathCommand::HorizontalLineTo(_) => "H",
            PathCommand::HorizontalLineToRel(_) => "h",
            PathCommand::VerticalLineTo(_) => "V",
            PathCommand::VerticalLineToRel(_) => "v",
            PathCommand::CurveTo(_, _, _, _, _, _) => "C",
            PathCommand::CurveToRel(_, _, _, _, _, _) => "c",
            PathCommand::SmoothCurveTo(_, _, _, _) => "S",
            PathCommand::SmoothCurveToRel(_, _, _, _) => "s",
            PathCommand::QuadraticCurveTo(_, _, _, _) => "Q",
            PathCommand::QuadraticCurveToRel(_, _, _, _) => "q",
            PathCommand::SmoothQuadraticCurveTo(_, _) => "T",
            PathCommand::SmoothQuadraticCurveToRel(_, _) => "t",
            PathCommand::Arc(_, _, _, _, _, _, _) => "A",
            PathCommand::ArcRel(_, _, _, _, _, _, _) => "a",
            PathCommand::ClosePath => "Z",
        }
    }

    /// Check if command is relative
    pub fn is_relative(&self) -> bool {
        match self {
            PathCommand::MoveToRel(..) | PathCommand::LineToRel(..) | 
            PathCommand::HorizontalLineToRel(_) | PathCommand::VerticalLineToRel(_) |
            PathCommand::CurveToRel(..) | PathCommand::SmoothCurveToRel(..) |
            PathCommand::QuadraticCurveToRel(..) | PathCommand::SmoothQuadraticCurveToRel(..) |
            PathCommand::ArcRel(..) => true,
            _ => false,
        }
    }
}
