pub mod d_unit;
pub mod l_unit;
pub mod lsd;
pub mod m_unit;
pub mod no_unit;
pub mod s_unit;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum FormatSymbol {
    /// Symbol *s*.
    ///
    /// Specifies the side for the destination.
    /// Side A = 0, Side B = 1.
    ///
    /// Please note that the definition of this parameter varies for compact instructions.
    Side,
    /// Symbol *p*.
    ///
    /// Specifies whether the next instruction is executed in parallel (= 1) or not (= 0).
    Parallel,
    /// Symbol *x*.
    ///
    /// Specifies the cross path for [Source2]
    Crosspath,
    /// Symbol *op*.
    ///
    /// Field within the opcode which specifies a unique instruction
    Opfield,
    /// Symbol *crlo*.
    ControlRegisterLow,
    /// Symbol *crhi*.
    ControlRegisterHigh,
    /// Symbol *h*.
    ///
    /// Specifies if the instruction is MVK (= 0) or MVKH (= 1).
    MoveHigh,
    /// Symbol *dst*.
    Destination,
    /// Symbol *src*.
    Source,
    /// Symbol *src/dst*.
    SourceOrDestination,
    /// Symbol *src1*.
    Source1,
    /// Symbol *src1/dst*.
    Source1OrDestination,
    /// Symbol *src2*.
    Source2,
    /// Symbol *src1/dst*.
    Source2OrDestination,
    /// Symbol *cstn* (i.e. *cst5*).
    Constant(u8),
    /// Symbol *ucstn* (i.e. *ucst5*).
    UnsignedConstant(u8),
    /// Symbol *scstn* (i.e. *scst5*).
    SignedConstant(u8),
    /// Symbol *N3*.
    ///
    /// 3-bit field.
    N3,
    /// Symbols *creg* and 8z* coupled together.
    ///
    /// *creg*: 3-bit field specifying a conditional register. \
    /// *z*: test for equality with zero or nonzero.
    ConditionalOperation,
    /// Symbol *z*.
    /// Use [ConditionalOperation] instead whenever possible.
    ///
    /// Specifies the test for equality with zero (= 1) or nonzero (= 0).
    Zero,
    /// Symbol for the *p-bits* field.
    FPHeadPBits,
    /// Symbol *SAT* in the *Expansion* field.
    ///
    /// Specifies if the compact instructions saturate (= 1) or not (= 0).
    FPHeadSaturate,
    /// Symbol *BR* in the *Expansion* field.
    ///
    /// Specifies if the compact instructions in the S unit are decoded as branches (= 1) or not (= 0).
    FPHeadBranches,
    /// Symbol *DSZ* in the *Expansion* field.
    ///
    /// Specifies the primary and secondary data size.
    FPHeadDataSizes,
    /// Symbol *RS* in the *Expansion* field.
    ///
    /// Specifies if instructions use the low (= 0) or high (= 1) register set.
    FPHeadRegisterSet,
    /// Symbol *PROT* in the *Expansion* field.
    ///
    /// Specifies if loads are protected (= 1) or not (= 0).
    FPHeadLoadsProtected,
    /// Symbol for the *Layout* field.
    FPHeadLayout,
    /// Symbol *n* for BDEC/BPOS instruction format.
    ///
    /// Specifies if the instruction is BDEC (= 1) or BPOS (= 0).
    IsBDec,
    /// Symbol *unit*.
    Unit,
    /// Symbol *CC*.
    CC,
    /// Symbol *offsetR*.
    RegisterOffset,
    /// Symbol *baseR*.
    BaseRegister,
    /// Symbol *mode*.
    AddressingMode,
    /// Symbol *y*.
    ///
    /// Specifies if the unit is .D1 (= 0) or .D2 (= 1).
    DUnitSide,
    /// Symbol *r*.
    ///
    /// Specifies if the instruction is an DW/NDW/NW instruction (= 1) or not (= 0).
    LoadStoreR,
    /// Symbol *sc*.
    ///
    /// Specifies the scaling mode: if it is non-scaled (= 0) or scaled (= 1). \
    /// When scaled, *offsetR*/*ucst5* are shifted.
    ScalingMode,
    /// Symbol *ld/st*.
    ///
    /// Specifies if the instruction is a load instruction (= 1) or a store instruction (= 0).
    IsLoad,
    /// Symbol *sz*.
    ///
    /// Specifies if the data size used is primary size (= 0) or secondary size (= 1).
    DataSize,
    /// Symbol *ptr*.
    ///
    /// Specifies the offset from A4-A7 (or B4-B7), or, to be more precise, the 2 least-significant bits
    /// of the [BaseRegister], with bit 2 (3rd least-significant bit) being forced to 1.
    Pointer,
    /// Symbol *t*.
    ///
    /// Specifies the side of the [SourceOrDestination] register.
    Side2,
    /// Symbol *na*.
    ///
    /// Specifies if the double word or word is aligned (= 0) or non-aligned (= 1).
    NonAligned,
    /// Symbol *dw*.
    ///
    /// Specifies if the load/store instruction is the double word variant (= 1) or not (= 0).
    IsDoubleWord,
}
