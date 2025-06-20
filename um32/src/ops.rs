use bitmatch::bitmatch;
use arbitrary_int::{u25, u3, Number};

type Register = u3;

#[derive(Debug, PartialEq)]
pub struct StandardOp {
    pub a: Register,
    pub b: Register,
    pub c: Register,
}

#[repr(packed)] // Is this super-slow?
#[derive(Debug, PartialEq)]
pub struct ConstOp {
}

#[derive(Debug, PartialEq)]
pub enum Op {
    /// **Conditional Move**:
    ///
    /// `if (reg[c]) != 0 { reg[a] := reg[b] }`
    ConditionalMove(StandardOp),

    /// **Array Index**:
    ///
    /// `reg[a] := array[reg[b]][c]`
    ArrayLoad(StandardOp),

    /// **Array Amendment**:
    ///
    /// `array[reg[a]][b] = reg[c]`
    ArrayStore(StandardOp),

    /// **Addition**:
    ///
    /// `reg[a] := reg[b] + reg[c]`
    Add(StandardOp),

    /// **Multiplication**:
    ///
    /// `reg[a] := reg[b] * reg[c]`
    Mul(StandardOp),

    /// **Division**:
    ///
    /// `reg[a] := reg[b] + reg[c]`
    Div(StandardOp),

    /// **Not And**:
    ///
    /// `reg[a] := !(reg[b] & reg[c])`
    NAnd(StandardOp),

    // XXX rustdoc doesn'ty allow the grouping of sub-items like Doxygen does
    // These operators don't use all three register numbers
    /// **Halt**:
    ///
    /// We're done here.
    Halt(StandardOp),

    /// **Allocation**:
    ///
    /// Allocate an array of `reg[c]` zeroed platters, placing its identifying number in `reg[a]`.
    Alloc(StandardOp),

    /// **Abandonment**:
    ///
    /// Deallocate the array identified by `reg[c]`.
    Free(StandardOp),

    /// **Output**:
    ///
    /// Write the value of `reg[c]` to the console. (Constrained to unsigned 8-bit values.)
    Out(StandardOp),

    /// **Input**:
    ///
    /// Wait for and read an 8-bit unsigned value from the console into
    /// `reg[c]`; after EOF, instead fill `reg[c]` with 1s.
    In(StandardOp),

    /// **Load Program**:
    ///
    /// Replace the '0' array with a copy of array `reg[b]`, setting the
    /// execution finger according to `reg[c]`.
    ///
    /// The spec says that it's important for this to be fast when `reg[b] ==
    /// 0`, probably because it's the **only** instruction that can transfer control.
    LoadProgram(StandardOp),

    /// **Orthography**:
    ///
    /// `reg[a] := value`.
    Const {
        a: Register,
        value: u25,
    },

    /// Invalid opcode; contains entire instruction for debugging purposes
    Invalid(u32),
}

impl From<u32> for Op {
    // attributes on expressions are experimental
    #[bitmatch]
    fn from(inst: u32) -> Self {
        #[bitmatch]
        match inst {
            "1101aaav_vvvvvvvv_vvvvvvvv_vvvvvvvv" => {
                Op::Const { a: Register::from_(a), value: u25::from_(v), }
            },
            "oooo????_????????_???????a_aabbbccc" => {
                let so = StandardOp { a: Register::from_(a), b: Register::from_(b), c: Register::from_(c) };
                use Op::*;
                match o {
                    0 => ConditionalMove(so),
                    1 => ArrayLoad(so),
                    2 => ArrayStore(so),
                    3 => Add(so),
                    4 => Mul(so),
                    5 => Div(so),
                    6 => NAnd(so),
                    7 => Halt(so),
                    8 => Alloc(so),
                    9 => Free(so),
                    10 => Out(so),
                    11 => In(so),
                    12 => LoadProgram(so),
                    13 => unreachable!("Should have been handled by the previous bitmatch arm... "),
                    14 | 15 => Invalid(inst),
                    16..=u32::MAX => unreachable!("4-bit opcodes can't be over 16, right?")
                }
            },
        }
    }
}
