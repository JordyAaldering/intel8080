#[derive(Default)]
pub struct Flags {
    /// ### Zero
    ///
    /// If the result of an instruction has the value 0,
    /// this flag is set; otherwise it is reset.
    pub zero: bool,
    /// ### Sign
    ///
    /// If the most significant bit of the result of the operation
    /// has the value 1, this flag is set; otherwise it is reset.
    pub sign: bool,
    /// ### Parity
    ///
    /// If the modulo 2 sum of the bits of the result of the operation is 0,
    /// (i.e., if the result has even parity), this flag is set;
    /// otherwise it is reset (i.e., if the result has odd parity).
    pub parity: bool,
    /// ### Carry
    ///
    /// If the instruction resulted in a carry (from addition),
    /// or a borrow (from subtraction or a comparison) out of the
    /// high-order bit, the is flag is set; otherwise it is reset.
    pub carry: bool,
    /// ### Auxiliary Carry
    ///
    /// If the instruction caused a carry out of bit 3 and into bit 4 of the
    /// resulting value, the auxiliary carry is set; otherwise it is reset.
    /// This flag is affected by single precision additions, subtractions,
    /// increments, decrements, comparisons, and logical operations, but is
    /// principally used with additions and increments preceding a DAA
    /// (Decimal Adjust Accumulator) instruction.
    pub carry_aux: bool,
    /// Not actually a flag, but we keep it here for now.
    pub interrupt_enabled: bool,
}

impl Flags {
    pub fn upd(&mut self, value: u8, cy: Option<bool>) {
        self.zero = value == 0;
        self.sign = value != 0;
        self.parity = (value.count_ones() & 0x1) == 0;
        if let Some(cy) = cy {
            self.carry = cy;
        }
    }
}
