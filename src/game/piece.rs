use serde::Deserialize;
use crate::game::player::Player;

#[derive(Debug, Clone, PartialEq)]
pub struct PieceType {
    pub name: String,
    pub symbol: String,
    pub moves: Vec<MovePattern>,
    pub promotion: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Piece {
    pub position: (usize, usize),
    pub piece_type: PieceType,
    pub owner: Player,
    pub move_count: u32,
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum MovePattern {
    Adjacent,
    BackwardOne,
    Diagonal,
    DiagonalBackward,
    DiagonalCapture,
    DiagonalForward,
    DiagonalForwardOne,
    DiagonalJumpOne,
    DiagonalOne,
    ElJump,
    ElJumpForward,
    EnPassant,
    ForwardOne,
    ForwardOneNoCapture,
    ForwardTwoOnFirstMove,
    FirstForwardTwoNoCapture,
    ForwardUnlimited,
    Horizontal,
    HorizontalOne,
    IguiDiagonalForward,
    IguiForward,
    JittoDiagonalForward,
    JittoForward,
    LeftOne,
    Orthogonal,
    OrthogonalJumpOne,
    OrthogonalOne,
    RightOne,
    StepOrJumpDiagonalForward,
    StepOrJumpForward,
    StepTwiceWithJumping,
    Straight,
    StraightOne,
    Vertical,
    VerticalOne,
}

impl MovePattern {
    pub fn from_str(name: &str) -> Option<Self> {
        match name {
            "adjacent" => Some(MovePattern::Adjacent),
            "backward_one" => Some(MovePattern::BackwardOne),
            "diagonal" => Some(MovePattern::Diagonal),
            "diagonal_capture" => Some(MovePattern::DiagonalCapture),
            "diagonal_backward" => Some(MovePattern::DiagonalBackward),
            "diagonal_forward" => Some(MovePattern::DiagonalForward),
            "diagonal_forward_one" => Some(MovePattern::DiagonalForwardOne),
            "diagonal_jump_one" => Some(MovePattern::DiagonalJumpOne),
            "diagonal_one" => Some(MovePattern::DiagonalOne),
            "L_jump" => Some(MovePattern::ElJump),
            "L_jump_forward" => Some(MovePattern::ElJumpForward),
            "en_passant" => Some(MovePattern::EnPassant),
            "forward_one" => Some(MovePattern::ForwardOne),
            "forward_one_no_capture" => Some(MovePattern::ForwardOneNoCapture),
            "first_forward_two_no_capture" => Some(MovePattern::FirstForwardTwoNoCapture),
            "forward_unlimited" => Some(MovePattern::ForwardUnlimited),
            "horizontal" => Some(MovePattern::Horizontal),
            "horizontal_one" => Some(MovePattern::HorizontalOne),
            "igui_diagonal_forward" => Some(MovePattern::IguiDiagonalForward),
            "igui_forward" => Some(MovePattern::IguiForward),
            "jitto_diagonal_forward" => Some(MovePattern::JittoDiagonalForward),
            "jitto_forward" => Some(MovePattern::JittoForward),
            "left_one" => Some(MovePattern::LeftOne),
            "orthogonal" => Some(MovePattern::Orthogonal),
            "orthogonal_jump_one" => Some(MovePattern::OrthogonalJumpOne),
            "orthogonal_one" => Some(MovePattern::OrthogonalOne),
            "right_one" => Some(MovePattern::RightOne),
            "step_or_jump_diagonal_forward" => Some(MovePattern::StepOrJumpDiagonalForward),
            "step_or_jump_forward" => Some(MovePattern::StepOrJumpForward),
            "step_twice_with_jumping" => Some(MovePattern::StepTwiceWithJumping),
            "straight" => Some(MovePattern::Straight),
            "straight_one" => Some(MovePattern::StraightOne),
            "vertical" => Some(MovePattern::Vertical),
            "vertical_one" => Some(MovePattern::VerticalOne),
            _ => None,
        }
    }
}

