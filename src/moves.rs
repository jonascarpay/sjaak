pub struct Move {
    bits: u16,
}

impl Move {
    // pub fn new(from: Square) -> Self {}
}

#[rustfmt::skip]
#[derive(Clone, Copy)]
pub enum MoveType {
    Quiet                ,
    DoublePush           ,
    CastleKingside       ,
    CastleQueenside      ,
    PromoteKnight        ,
    PromoteBishop        ,
    PromoteRook          ,
    PromoteQueen         ,

    Capture              ,
    CaptureEnPassant     ,
    PromoteCaptureKnight ,
    PromoteCaptureBishop ,
    PromoteCaptureRook   ,
    PromoteCaptureQueen  ,
}

impl MoveType {
    #[inline(never)]
    pub fn is_capture(self) -> bool {
        match self {
            MoveType::Capture => true,
            MoveType::CaptureEnPassant => true,
            MoveType::PromoteCaptureKnight => true,
            MoveType::PromoteCaptureBishop => true,
            MoveType::PromoteCaptureRook => true,
            MoveType::PromoteCaptureQueen => true,
            _ => false,
        }
    }
}
