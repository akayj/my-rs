use bitflags::bitflags;

bitflags! {
    struct Flags: u32 {
        const A = 0b00000001;
        const B = 0b00000010;
        const C = 0b00000100;
        const ABC = Self::A.bits | Self::B.bits | Self::C.bits;
    }
}

pub fn mybits() {
    let e1 = Flags::A | Flags::C;
    let e2 = Flags::B | Flags::C;

    assert_eq!((e1 | e2), Flags::ABC);
}
