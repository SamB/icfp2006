use std::io::stdin;
use std::io::stdout;
use std::io::ErrorKind::UnexpectedEof;
use std::io::Read;
use std::io::Write;
use std::process::exit;

use arbitrary_int::*;

use um32::ops::*;
use um32::regs::*;
use um32::memory::*;

struct State {
    mem: Memory,
    regfile: Registers
}


impl State {
    fn step(&mut self) -> std::io::Result<()> {
        macro_rules! R {
            [$r:expr] => {
                self.regfile.regs[$r.as_usize()]
            };
        }

        let inst = self.mem.read(0, self.regfile.finger);
        self.regfile.finger += 1;
        let op = Op::from(inst);

        match op {
            Op::ConditionalMove(sop) => {
                Ok(if 0 != R![sop.c] {
                    R![sop.a] = R![sop.b];
                })
            },
            Op::ArrayLoad(sop) => Ok(R![sop.a] = self.mem.read(R![sop.b], R![sop.c])),
            Op::ArrayStore(sop) => Ok(self.mem.write(R![sop.a], R![sop.b], R![sop.c])),
            Op::Add(sop) => Ok(R![sop.a] = Platter::wrapping_add(R![sop.b], R![sop.c])),
            Op::Mul(sop) => Ok(R![sop.a] = Platter::wrapping_mul(R![sop.b], R![sop.c])),
            Op::Div(sop) => Ok(R![sop.a] = R![sop.b] / R![sop.c]),
            Op::NAnd(sop) => Ok(R![sop.a] = !(R![sop.b] & R![sop.c])),
            Op::Halt(_sop) => {
                // FIXME: do we want cleaner control flow?
                exit(0);
            },
            Op::Alloc(sop) => Ok(R![sop.a] = self.mem.alloc(R![sop.c])),
            Op::Free(sop) => Ok(self.mem.free(R![sop.c])),
            Op::Out(sop) => {
                let code = u8::try_from(R![sop.c]).expect("console output should be 8-bit");
                let mut out = stdout();
                out.write_all(&[code])?;
                out.flush()
            },
            Op::In(sop) => {
                let mut buf: [u8;1] = [0; 1];
                let result = match stdin().read_exact(&mut buf) {
                    Ok(_) => Ok(buf[0] as Platter),
                    Err(e) if UnexpectedEof == e.kind() => Ok(u32::max_value()),
                    Err(e) => Err(e),
                }?;
                R![sop.c] = result;
                Ok(())
            },
            Op::LoadProgram(sop) => {
                if R![sop.b] != 0 {
                    self.mem.load(self.mem[R![sop.b]].clone())
                }
                self.regfile.finger = R![sop.c];
                Ok(())
            },
            Op::Const { a, value } => Ok(R![a] = Platter::from(value)),
            Op::Invalid(_) => todo!(),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::env::args_os().nth(1).expect("a program should be passed on the command line");
    let program = array_from_file(file)?;
    let mut s = State {
        mem: Memory::new(),
        regfile: Registers::default(),
    };
    s.mem.load(program);
    loop {
        s.step()?;
    }
}
