mod flags;
mod memory;
mod opcode;
mod registers;

use flags::Flags;
use memory::Memory;
use opcode::Opcode;
use registers::Registers;

use std::{fs::File, io::BufReader, process::exit};

fn add(a: u8, value: u8, flags: &mut Flags) -> u8 {
    let (a, cy) = a.overflowing_add(value);
    flags.upd(a, Some(cy));
    a
}

fn adc(a: u8, value: u8, flags: &mut Flags) -> u8 {
    let (a, cy1) = a.overflowing_add(value);
    let (a, cy2) = a.overflowing_add(flags.carry as u8);
    flags.upd(a, Some(cy1 | cy2));
    a
}

fn sub(a: u8, value: u8, flags: &mut Flags) -> u8 {
    let (a, cy) = a.overflowing_sub(value);
    flags.upd(a, Some(cy));
    a
}

fn sbb(a: u8, value: u8, flags: &mut Flags) -> u8 {
    let (a, cy1) = a.overflowing_sub(value);
    let (a, cy2) = a.overflowing_sub(flags.carry as u8);
    flags.upd(a, Some(cy1 | cy2));
    a
}

fn and(a: u8, value: u8, flags: &mut Flags) -> u8 {
    let a = a & value;
    flags.upd(a, Some(false));
    a
}

fn xor(a: u8, value: u8, flags: &mut Flags) -> u8 {
    let a = a ^ value;
    flags.upd(a, Some(false));
    a
}

fn ior(a: u8, value: u8, flags: &mut Flags) -> u8 {
    let a = a | value;
    flags.upd(a, Some(false));
    a
}

fn cmp(a: u8, value: u8, flags: &mut Flags) {
    let (a, cy) = a.overflowing_sub(value);
    flags.upd(a, Some(cy));
}

fn emulate(reg: &mut Registers, flags: &mut Flags, mem: &mut Memory) {
    //let pc = mem.pc;
    let opcode = mem.read_opcode();
    // println!("{:#010x} | {:?}", pc, opcode);
    match opcode {
        //
        // Data transfer group
        //
        // This group of instructions transfers data to and from registers
        // and memory. Condition flags are not affected by any instruction
        // in this group.
        //

        // MOV r1, r2 (Move register)
        //   (r1) ← (r2)
        //   The content of register r2 is moved to register r1.
        //
        // Cycles: 1
        // States: 5
        // Addressing: register
        // Flags: none
        Opcode::MOV_AA => {},
        Opcode::MOV_BB => {},
        Opcode::MOV_CC => {},
        Opcode::MOV_DD => {},
        Opcode::MOV_EE => {},
        Opcode::MOV_HH => {},
        Opcode::MOV_LL => {},

        Opcode::MOV_AB => reg.a = reg.b,
        Opcode::MOV_AC => reg.a = reg.c,
        Opcode::MOV_AD => reg.a = reg.d,
        Opcode::MOV_AE => reg.a = reg.e,
        Opcode::MOV_AH => reg.a = reg.h,
        Opcode::MOV_AL => reg.a = reg.l,

        Opcode::MOV_BA => reg.b = reg.a,
        Opcode::MOV_BC => reg.b = reg.c,
        Opcode::MOV_BD => reg.b = reg.d,
        Opcode::MOV_BE => reg.b = reg.e,
        Opcode::MOV_BH => reg.b = reg.h,
        Opcode::MOV_BL => reg.b = reg.l,

        Opcode::MOV_CA => reg.c = reg.a,
        Opcode::MOV_CB => reg.c = reg.b,
        Opcode::MOV_CD => reg.c = reg.d,
        Opcode::MOV_CE => reg.c = reg.e,
        Opcode::MOV_CH => reg.c = reg.h,
        Opcode::MOV_CL => reg.c = reg.l,

        Opcode::MOV_DA => reg.d = reg.a,
        Opcode::MOV_DB => reg.d = reg.b,
        Opcode::MOV_DC => reg.d = reg.c,
        Opcode::MOV_DE => reg.d = reg.e,
        Opcode::MOV_DH => reg.d = reg.h,
        Opcode::MOV_DL => reg.d = reg.l,

        Opcode::MOV_EA => reg.e = reg.a,
        Opcode::MOV_EB => reg.e = reg.b,
        Opcode::MOV_EC => reg.e = reg.c,
        Opcode::MOV_ED => reg.e = reg.d,
        Opcode::MOV_EH => reg.e = reg.h,
        Opcode::MOV_EL => reg.e = reg.l,

        Opcode::MOV_HA => reg.h = reg.a,
        Opcode::MOV_HB => reg.h = reg.b,
        Opcode::MOV_HC => reg.h = reg.c,
        Opcode::MOV_HD => reg.h = reg.d,
        Opcode::MOV_HE => reg.h = reg.e,
        Opcode::MOV_HL => reg.h = reg.l,

        Opcode::MOV_LA => reg.l = reg.a,
        Opcode::MOV_LB => reg.l = reg.b,
        Opcode::MOV_LC => reg.l = reg.c,
        Opcode::MOV_LD => reg.l = reg.d,
        Opcode::MOV_LE => reg.l = reg.e,
        Opcode::MOV_LH => reg.l = reg.h,

        // MOV r, M (Move from memory)
        //   (r) ← ((H) (L))
        //   The content of the memory location, whose address
        //   is in registers H and L, is moved to register r.
        //
        // Cycles: 2
        // States: 7
        // Addressing: register indirect
        // Flags: none
        Opcode::MOV_AM => reg.a = mem[reg.m()],
        Opcode::MOV_BM => reg.b = mem[reg.m()],
        Opcode::MOV_CM => reg.c = mem[reg.m()],
        Opcode::MOV_DM => reg.d = mem[reg.m()],
        Opcode::MOV_EM => reg.e = mem[reg.m()],
        Opcode::MOV_HM => reg.h = mem[reg.m()],
        Opcode::MOV_LM => reg.l = mem[reg.m()],

        // MOV M, r (Move to memory)
        //   ((H) (L)) ← (r)
        //   The content of register r is moved to the memory
        //   location whose address is in registers H and L.
        //
        // Cycles: 2
        // States: 7
        // Addressing: register indirect
        // Flags: none
        Opcode::MOV_MA => mem[reg.m()] = reg.a,
        Opcode::MOV_MB => mem[reg.m()] = reg.b,
        Opcode::MOV_MC => mem[reg.m()] = reg.c,
        Opcode::MOV_MD => mem[reg.m()] = reg.d,
        Opcode::MOV_ME => mem[reg.m()] = reg.e,
        Opcode::MOV_MH => mem[reg.m()] = reg.h,
        Opcode::MOV_ML => mem[reg.m()] = reg.l,

        // MVI r, data (Move immediate)
        //   (r) ← (byte 2)
        //   The content of byte 2 of the instruction is moved to register r.
        //
        // Cycles: 2
        // States: 7
        // Addressing: immediate
        // Flags: none
        Opcode::MVI_A(d8) => reg.a = d8,
        Opcode::MVI_B(d8) => reg.b = d8,
        Opcode::MVI_C(d8) => reg.c = d8,
        Opcode::MVI_D(d8) => reg.d = d8,
        Opcode::MVI_E(d8) => reg.e = d8,
        Opcode::MVI_H(d8) => reg.h = d8,
        Opcode::MVI_L(d8) => reg.l = d8,

        // MVI M, data (Move to memory immediate)
        //   ((H) (L)) ← (byte 2)
        //   The content of byte 2 of the instruction is moved to the
        //   memory location whose address is in registers H and L.
        //
        // Cycles: 3
        // States: 10
        // Addressing: immediate/register indirect
        // Flags: none
        Opcode::MVI_M(d8) => mem[reg.m()] = d8,

        // LXI rp, data 16 (Load register pair immediate)
        //   (rh) ← (byte 3),
        //   (rl) ← (byte 2)
        //   Byte 3 of the instruction is moved into the high-order register
        //   (rh) of the register pair rp. Byte 2 of the instruction is moved
        //   into the low-order register (rl) of the register pair rp.
        //
        // Cycles: 3
        // States: 10
        // Addressing: immediate
        // Flags: none
        Opcode::LXI_BC(rl, rh) => { reg.b = rh; reg.c = rl; },
        Opcode::LXI_DE(rl, rh) => { reg.d = rh; reg.e = rl; },
        Opcode::LXI_HL(rl, rh) => { reg.h = rh; reg.l = rl; },
        Opcode::LXI_SP(rl, rh) => mem.set_sp(rh, rl),

        // LDA addr (Load accumulator direct)
        //   (A) ← ((byte 3) (byte 2))
        //   The content of the memory location, whose address is specified in
        //   byte 2 and byte 3 of the instruction, is moved to register A.
        //
        // Cycles: 4
        // States: 13
        // Addressing: direct
        // Flags: none
        Opcode::LDA(adr) => mem[adr] = reg.a,

        // STA addr (Store accumulator direct)
        //   ((byte 3) (byte 2)) ← (A)
        //   The content of the accumulator is moved to the memory location
        //   whose address is specified in byte 2 and byte 3 of the instruction.
        //
        // Cycles: 4
        // States: 13
        // Addressing: direct
        // Flags: none
        Opcode::STA(adr) => mem[adr] = reg.a,

        // LHLD addr (Load H and L direct)
        //   (L) ← ((byte 3) (byte 2))
        //   (H) ← ((byte 3) (byte 2) + 1)
        //   The content of the memory location, whose address is specified
        //   in byte 2 and byte 3 of the instruction, is moved to register L.
        //   The content of the memory location at the succeeding address
        //   is moved to register H.
        //
        // Cycles: 5
        // States: 16
        // Addressing: direct
        // Flags: none
        Opcode::LHLD(adr) => {
            reg.l = mem[adr];
            reg.h = mem[adr + 1];
        },

        // SHLD addr (Store H and L direct)
        //   ((byte 3) (byte 2))     ← (L)
        //   ((byte 3) (byte 2) + 1) ← (H)
        //   The content of register L is moved to the memory location whose
        //   address is specified in byte 2 and byte 3. The content of
        //   register H is moved to the succeeding memory location.
        //
        // Cycles: 5
        // States: 16
        // Addressing: direct
        // Flags: none
        Opcode::SHLD(adr) => {
            mem[adr] = reg.l;
            mem[adr + 1] = reg.h;
        },

        // LDAX rp (Load accumulator indirect)
        //   (A) ← ((rp))
        //   The content of the memory location, whose address is
        //   in the register pair rp, is moved to register A.
        //
        // Cycles: 2
        // States: 7
        // Addressing: register direct
        // Flags: none
        Opcode::LDAX_BC => reg.a = mem[reg.bc()],
        Opcode::LDAX_DE => reg.a = mem[reg.de()],

        // STAX rp (Store accumulator indirect)
        //   ((rp)) ← (A)
        //   The content of register A is moved to the memory location
        //   whose address is in the register pair rp.
        //
        // Cycles: 2
        // States: 7
        // Addressing: register direct
        // Flags: none
        Opcode::STAX_BC => mem[reg.bc()] = reg.a,
        Opcode::STAX_DE => mem[reg.de()] = reg.a,

        // XCHG (Exchange H and L with D and E)
        //   (H) ↔ (D)
        //   (L) ↔ (E)
        //   The contents of registers Hand L are exchanged
        //   with the contents of registers D and E.
        //
        // Cycles: 1
        // States: 4
        // Addressing: register
        // Flags: none
        Opcode::XCHG => {
            std::mem::swap(&mut reg.c, &mut reg.d);
            std::mem::swap(&mut reg.l, &mut reg.e);
        },

        //
        // Arithmetic group
        //
        // This group of instructions performs arithmetic operations on
        // data in registers and memory.
        // Unless indicated otherwise, all instructions in this group affect
        // the Zero, Sign, Parity, Carry, and Auxiliary Carry flags according
        // to the standard rules.
        // All subtraction operations are performed via two's complement
        // arithmetic and set the carry flag to one to indicate a borrow
        // and clear it to indicate no borrow.
        //

        // ADD r (Add register)
        //   (A) ← (A) + (r)
        //   The content of register r is added to the content of the
        //   accumulator. The result is placed in the accumulator.
        //
        // Cycles: 1
        // States: 4
        // Addressing: register
        // Flags: Z, S, P, CY, AC
        Opcode::ADD_A => reg.a = add(reg.a, reg.a, flags),
        Opcode::ADD_B => reg.a = add(reg.a, reg.b, flags),
        Opcode::ADD_C => reg.a = add(reg.a, reg.c, flags),
        Opcode::ADD_D => reg.a = add(reg.a, reg.d, flags),
        Opcode::ADD_E => reg.a = add(reg.a, reg.e, flags),
        Opcode::ADD_H => reg.a = add(reg.a, reg.h, flags),
        Opcode::ADD_L => reg.a = add(reg.a, reg.l, flags),

        // ADD M (Add memory)
        //   (A) ← (A) + ((H) (L))
        //   The content of the memory location whose address is contained
        //   in the H and L registers is added to the content of the
        //   accumulator. The result is placed in the accumulator.
        //
        // Cycles: 2
        // States: 7
        // Addressing: register indirect
        // Flags: Z, S, P, CY, AC
        Opcode::ADD_M => reg.a = add(reg.a, mem[reg.m()], flags),

        // ADI data (Add immediate)
        //   (A) ← (A) + (byte 2)
        //   The content of the second byte of the instruction is added
        //   to the content of the accumulator. The result is placed
        //   in the accumulator.
        //
        // Cycles: 2
        // States: 7
        // Addressing: immediate
        // Flags: Z, S, P, CY, AC
        Opcode::ADI(d8) => reg.a = add(reg.a, d8, flags),

        // ADC r (Add register with carry)
        //   (A) ← (A) + (r) + (CY)
        //   The content of register r and the content of the carry bit
        //   are added to the content of the accumulator. The result
        //   is placed in the accumulator.
        //
        // Cycles: 1
        // States: 4
        // Addressing: register
        // Flags: Z, S, P, CY, AC
        Opcode::ADC_A => reg.a = adc(reg.a, reg.a, flags),
        Opcode::ADC_B => reg.a = adc(reg.a, reg.b, flags),
        Opcode::ADC_C => reg.a = adc(reg.a, reg.c, flags),
        Opcode::ADC_D => reg.a = adc(reg.a, reg.d, flags),
        Opcode::ADC_E => reg.a = adc(reg.a, reg.e, flags),
        Opcode::ADC_H => reg.a = adc(reg.a, reg.h, flags),
        Opcode::ADC_L => reg.a = adc(reg.a, reg.l, flags),

        // ADC M (Add memory with carry)
        //   (A) ← (A) + ((H) (L)) + (CY)
        //   The content of the memory location whose address is contained in
        //   the H and L registers and the content of the CY flag are added
        //   to the accumulator. The result is placed in the accumulator.
        //
        // Cycles: 2
        // States: 4
        // Addressing: register indirect
        // Flags: Z, S, P, CY, AC
        Opcode::ADC_M => reg.a = adc(reg.a, mem[reg.m()], flags),

        // ACI data (Add immediate with carry)
        //   (A) ← (A) + (byte 2) + (CY)
        //   The content of the second byte of the instruction and the
        //   content of the CY flag are added to the contents of the
        //   accumulator. The result is placed in the accumulator.
        //
        // Cycles: 2
        // States: 7
        // Addressing: immediate
        // Flags: Z, S, P, CY, AC
        Opcode::ACI(d8) => reg.a = adc(reg.a, d8, flags),

        // SUB r (Subtract register)
        //   (A) ← (A) - (r)
        //   The content of register r is subtracted from the content of
        //   the accumulator. The result is placed in the accumulator.
        //
        // Cycles: 1
        // States: 4
        // Addressing: register
        // Flags: Z, S, P, CY, AC
        Opcode::SUB_A => reg.a = sub(reg.a, reg.a, flags),
        Opcode::SUB_B => reg.a = sub(reg.a, reg.b, flags),
        Opcode::SUB_C => reg.a = sub(reg.a, reg.c, flags),
        Opcode::SUB_D => reg.a = sub(reg.a, reg.d, flags),
        Opcode::SUB_E => reg.a = sub(reg.a, reg.e, flags),
        Opcode::SUB_H => reg.a = sub(reg.a, reg.h, flags),
        Opcode::SUB_L => reg.a = sub(reg.a, reg.l, flags),

        // SUB M (Subtract memory)
        //   (A) ← (A) - ((H) (L))
        //   The content of the memory location whose address is contained
        //   in the H and L registers is subtracted from the content of the
        //   accumulator. The result is placed in the accumulator.
        //
        // Cycles: 2
        // States: 7
        // Addressing: register indirect
        // Flags: Z, S, P, CY, AC
        Opcode::SUB_M => reg.a = sub(reg.a, mem[reg.m()], flags),

        // SUI data (Subtract immediate)
        //   (A) ← (A) - (byte 2)
        //   The content of the second byte of the instruction is subtracted
        //   from the content of the accumulator. The result is placed in
        //   the accumulator.
        //
        // Cycles: 2
        // States: 7
        // Addressing: immediate
        // Flags: Z, S, P, CY, AC
        Opcode::SUI(d8) => reg.a = sub(reg.a, d8, flags),

        // SBB r (Subtract register with borrow)
        //   (A) ← (A) - (r) - (CY)
        //   The content of register r and the content of the CY flag are
        //   both subtracted from the accumulator. The result is placed
        //   in the accumulator.
        //
        // Cycles: 1
        // States: 4
        // Addressing: register
        // Flags: Z, S, P, CY, AC
        Opcode::SBB_A => reg.a = sbb(reg.a, reg.a, flags),
        Opcode::SBB_B => reg.a = sbb(reg.a, reg.b, flags),
        Opcode::SBB_C => reg.a = sbb(reg.a, reg.c, flags),
        Opcode::SBB_D => reg.a = sbb(reg.a, reg.d, flags),
        Opcode::SBB_E => reg.a = sbb(reg.a, reg.e, flags),
        Opcode::SBB_H => reg.a = sbb(reg.a, reg.h, flags),
        Opcode::SBB_L => reg.a = sbb(reg.a, reg.l, flags),

        // SBB M (Subtract memory with borrow)
        //   (A) ← (A) - ((H) (L)) - (CY)
        //   The content of the memory location whose address is contained
        //   in the H and L registers and the content of the CY flag are
        //   both subtracted from the accumulator. The result is placed
        //   in the accumulator.
        //
        // Cycles: 2
        // States: 7
        // Addressing: register indirect
        // Flags: Z, S, P, CY, AC
        Opcode::SBB_M => reg.a = sbb(reg.a, mem[reg.m()], flags),

        // SBI data (Subtract immediate with borrow)
        //   (A) ← (A) - (byte 2) - (CY)
        //   The contents of the second byte of the instruction and the
        //   contents of the CY flag are both subtracted from the accumulator.
        //   The result is placed in the accumulator.
        //
        // Cycles: 2
        // States: 7
        // Addressing: immediate
        // Flags: Z, S, P, CY, AC
        Opcode::SBI(d8) => reg.a = sbb(reg.a, d8, flags),

        // INR r (Increment register)
        //   (r) ← (r) + 1
        //   The content of register r is incremented by one.
        //   Note: All condition flags except CY are affected.
        //
        // Cycles: 1
        // States: 5
        // Addressing: register
        // Flags: Z, S, P, AC
        Opcode::INR_A => { reg.a = reg.a.wrapping_add(1); flags.upd(reg.a, None); },
        Opcode::INR_B => { reg.b = reg.b.wrapping_add(1); flags.upd(reg.b, None); },
        Opcode::INR_C => { reg.c = reg.c.wrapping_add(1); flags.upd(reg.c, None); },
        Opcode::INR_D => { reg.d = reg.d.wrapping_add(1); flags.upd(reg.d, None); },
        Opcode::INR_E => { reg.e = reg.e.wrapping_add(1); flags.upd(reg.e, None); },
        Opcode::INR_H => { reg.h = reg.h.wrapping_add(1); flags.upd(reg.h, None); },
        Opcode::INR_L => { reg.l = reg.l.wrapping_add(1); flags.upd(reg.l, None); },

        // INR M (Increment memory)
        //   ((H) (L)) ← ((H) (L)) + 1
        //   The content of the memory location whose address is
        //   contained in the H and L registers is incremented by one.
        //   Note: All condition flags except CY are affected.
        //
        // Cycles: 3
        // States: 10
        // Addressing: register indirect
        // Flags: Z, S, P, AC
        Opcode::INR_M => { mem[reg.m()] = mem[reg.m()].wrapping_add(1); flags.upd(mem[reg.m()], None); },

        // DCR r (Decrement register)
        //   (r) ← (r)-1
        //   The content of register r is decremented by one.
        //   Note: All condition flags except CY are affected.
        //
        // Cycles: 1
        // States: 5
        // Addressing: register
        // Flags: Z, S, P, AC
        Opcode::DCR_A => { reg.a = reg.a.wrapping_sub(1); flags.upd(reg.a, None); },
        Opcode::DCR_B => { reg.b = reg.b.wrapping_sub(1); flags.upd(reg.b, None); },
        Opcode::DCR_C => { reg.c = reg.c.wrapping_sub(1); flags.upd(reg.c, None); },
        Opcode::DCR_D => { reg.d = reg.d.wrapping_sub(1); flags.upd(reg.d, None); },
        Opcode::DCR_E => { reg.e = reg.e.wrapping_sub(1); flags.upd(reg.e, None); },
        Opcode::DCR_H => { reg.h = reg.h.wrapping_sub(1); flags.upd(reg.h, None); },
        Opcode::DCR_L => { reg.l = reg.l.wrapping_sub(1); flags.upd(reg.l, None); },

        // DCR M (Decrement memory)
        //   ((H) (L)) ← ((H) (L)) - 1
        //   The content of the memory location whose address is
        //   contained in the Hand L registers is decremented by one.
        //   Note: All condition flags except CY are affected.
        //
        // Cycles: 3
        // States: 10
        // Addressing: register indirect
        // Flags: Z, S, P, AC
        Opcode::DCR_M => { mem[reg.m()] = mem[reg.m()].wrapping_sub(1); flags.upd(mem[reg.m()], None); },

        // INX rp (Increment register pair)
        //   (rh) (rl) ← (rh) (rl) + 1
        //   The content of the register pair rp is incremented by one.
        //   Note: No condition flags are affected.
        //
        // Cycles: 1
        // States: 5
        // Addressing: register
        // Flags: none
        Opcode::INX_BC => {
            let (c, cy) = reg.c.overflowing_add(1);
            reg.b = reg.b.wrapping_add(1 + cy as u8);
            reg.c = c;
        },
        Opcode::INX_DE => {
            let (e, cy) = reg.e.overflowing_add(1);
            reg.d = reg.d.wrapping_add(1 + cy as u8);
            reg.e = e;
        },
        Opcode::INX_HL => {
            let (l, cy) = reg.l.overflowing_add(1);
            reg.h = reg.h.wrapping_add(1 + cy as u8);
            reg.l = l;
        },
        Opcode::INX_SP => mem.pc = mem.pc.wrapping_add(1),

        // DCX rp (Decrement register pair)
        //   (rh) (rl) ← (rh) (rl) - 1
        //   The content of the register pair rp is decremented by one.
        //   Note: No condition flags are affected.
        //
        // Cycles: 1
        // States: 5
        // Addressing: register
        // Flags: none
        Opcode::DCX_BC => {
            let (c, cy) = reg.c.overflowing_sub(1);
            reg.b = reg.b.wrapping_add(1 + cy as u8);
            reg.c = c;
        },
        Opcode::DCX_DE => {
            let (e, cy) = reg.e.overflowing_sub(1);
            reg.d = reg.d.wrapping_add(1 + cy as u8);
            reg.e = e;
        },
        Opcode::DCX_HL => {
            let (l, cy) = reg.l.overflowing_sub(1);
            reg.h = reg.h.wrapping_add(1 + cy as u8);
            reg.l = l;
        },
        Opcode::DCX_SP => mem.pc = mem.pc.wrapping_sub(1),

        // DAD rp (Add register pair to H and L)
        //   (H) (L) ← (H) (L) + (rh) (rl)
        //   The content of the register pair rp is added to the content of
        //   the register pair H and L. The result is placed in the register
        //   pair H and L.
        //   Note: Only the CY flag is affected. It is set if there is a
        //   carry out of the double precision add; otherwise it is reset.
        //
        // Cycles: 3
        // States: 10
        // Addressing: register
        // Flags: CY
        Opcode::DAD_BC => {
            let (l, cy) = reg.l.overflowing_add(reg.b);
            let (h, cy1) = reg.h.overflowing_add(cy as u8);
            let (h, cy2) = h.overflowing_add(reg.c);
            flags.carry = cy1 | cy2;
            reg.l = l;
            reg.h = h;
        },
        Opcode::DAD_DE => {
            let (l, cy) = reg.l.overflowing_add(reg.d);
            let (h, cy1) = reg.h.overflowing_add(cy as u8);
            let (h, cy2) = h.overflowing_add(reg.e);
            flags.carry = cy1 | cy2;
            reg.l = l;
            reg.h = h;
        },
        Opcode::DAD_HL => {
            let (l, cy) = reg.l.overflowing_add(reg.h);
            let (h, cy1) = reg.h.overflowing_add(cy as u8);
            let (h, cy2) = h.overflowing_add(reg.l);
            flags.carry = cy1 | cy2;
            reg.l = l;
            reg.h = h;
        },
        Opcode::DAD_SP => {
            let (rh, rl) = mem.get_pc();
            let (l, cy) = reg.l.overflowing_add(rh);
            let (h, cy1) = reg.h.overflowing_add(cy as u8);
            let (h, cy2) = h.overflowing_add(rl);
            flags.carry = cy1 | cy2;
            reg.l = l;
            reg.h = h;
        },

        // DAA (Decimal adjust accumulator)
        //   The eight-bit number in the accumulator is adjusted to form two
        //   four-bit Binary-Coded-Decimal digits by the following process:
        //   1. If the value of the least significant 4 bits of the accumulator
        //      is greater than 9 or if the AC flag is set, 6 is added to the
        //      accumulator.
        //   2. If the value of the most significant 4 bits of the accumulator
        //      is now greater than 9, or if the CY flag is set, 6 is added to
        //      the most significant 4 bits of the accumulator.
        //   Note: All flags are affected.
        //
        // Cycles: 1
        // States: 4
        // Flags: Z, S, P, CY, AC
        Opcode::DAA => todo!(),

        //
        // Logical group
        //
        // This group of instructions performs logical (Boolean) operations
        // on data in registers and memory and on condition flags.
        // Unless indicated otherwise, all instructions in this group affect
        // the Zero, Sign, Parity, Auxiliary Carry, and Carry flags according
        // to the standard rules.
        //

        // ANA r (AND register)
        //   (A) ← (A) & (r)
        //   The content of register r is logically anded with the content of
        //   the accumulator. The result is placed in the accumulator.
        //   The CY flag is cleared.
        //
        // Cycles: 1
        // States: 4
        // Addressing: register
        // Flags: Z, S, P, CY, AC
        Opcode::ANA_A => flags.upd(reg.a, Some(false)),
        Opcode::ANA_B => reg.a = and(reg.a, reg.b, flags),
        Opcode::ANA_C => reg.a = and(reg.a, reg.c, flags),
        Opcode::ANA_D => reg.a = and(reg.a, reg.d, flags),
        Opcode::ANA_E => reg.a = and(reg.a, reg.e, flags),
        Opcode::ANA_H => reg.a = and(reg.a, reg.h, flags),
        Opcode::ANA_L => reg.a = and(reg.a, reg.l, flags),

        // ANA M (AND memory)
        //   (A) ← (A) & ((H) (L))
        //   The contents of the memory location whose address is contained
        //   in the H and L registers is logically anded with the content of
        //   the accumulator. The result is placed in the accumulator.
        //   The CY flag is cleared.
        //
        // Cycles: 2
        // States: 7
        // Addressing: register indirect
        // Flags: Z, S, P, CY, AC
        Opcode::ANA_M => reg.a = and(reg.a, mem[reg.m()], flags),

        // ANI data (AND immediate)
        //   (A) ← (A) & (byte 2)
        //   The content of the second byte of the instruction is logically
        //   anded with the contents of the accumulator. The result is placed
        //   in the accumulator. The CY and AC flags are cleared.
        //
        // Cycles: 2
        // States: 7
        // Addressing: immediate
        // Flags: Z, S, P, CY, AC
        Opcode::ANI(d8) => reg.a = and(reg.a, d8, flags),

        // XRA r (Exclusive OR register)
        //   (A) ← (A) ^ (r)
        //   The content of register r is exclusive-or'd with the content of
        //   the accumulator. The result is placed in the accumulator.
        //   The CY and AC flags are cleared.
        //
        // Cycles: 1
        // States: 4
        // Addressing: register
        // Flags: Z, S, P, CY, AC
        Opcode::XRA_A => {
            flags.upd(0, Some(false));
            reg.a = 0;
        },
        Opcode::XRA_B => reg.a = xor(reg.a, reg.b, flags),
        Opcode::XRA_C => reg.a = xor(reg.a, reg.c, flags),
        Opcode::XRA_D => reg.a = xor(reg.a, reg.d, flags),
        Opcode::XRA_E => reg.a = xor(reg.a, reg.e, flags),
        Opcode::XRA_H => reg.a = xor(reg.a, reg.h, flags),
        Opcode::XRA_L => reg.a = xor(reg.a, reg.l, flags),

        // XRA M (Exclusive OR memory)
        //   (A) ← (A) ^ ((H) (L))
        //   The content of the memory location whose address is contained in
        //   the H and L registers is exclusive-OR'd with the content of the
        //   accumulator. The result is placed in the accumulator.
        //   The CY and AC flags are cleared.
        //
        // Cycles: 2
        // States: 7
        // Addressing: register indirect
        // Flags: Z, S, P, CY, AC
        Opcode::XRA_M => reg.a = xor(reg.a, mem[reg.m()], flags),

        // XRI data (Exclusive OR immediate)
        //   (A) ← (A) ^ (byte 2)
        //   The content of the second byte of the instruction is
        //   exclusive-OR'd with the content of the accumulator.
        //   The result is placed in the accumulator.
        //   The CY and AC flags are cleared.
        //
        // Cycles: 2
        // States: 7
        // Addressing: immediate
        // Flags: Z, S, P, CY, AC
        Opcode::XRI(d8) => reg.a = xor(reg.a, d8, flags),

        // ORA r (OR register)
        //   (A) ← (A) | (r)
        //   The content of register r is inclusive-OR'd with the content of
        //   the accumulator. The result is placed in the accumulator.
        //   The CY and AC flags are cleared.
        //
        // Cycles: 1
        // States: 4
        // Addressing: register
        // Flags: Z, S, P, CY, AC
        Opcode::ORA_A => flags.upd(0, Some(false)),
        Opcode::ORA_B => reg.a = ior(reg.a, reg.b, flags),
        Opcode::ORA_C => reg.a = ior(reg.a, reg.c, flags),
        Opcode::ORA_D => reg.a = ior(reg.a, reg.d, flags),
        Opcode::ORA_E => reg.a = ior(reg.a, reg.e, flags),
        Opcode::ORA_H => reg.a = ior(reg.a, reg.h, flags),
        Opcode::ORA_L => reg.a = ior(reg.a, reg.l, flags),

        // ORA M (OR memory)
        //   (A) ← (A) | ((H) (L))
        //   The content of the memory location whose address is contained in
        //   the H and L registers is inclusive-OR'd with the content of the
        //   accumulator. The result is placed in the accumulator.
        //   The CY and AC flags are cleared.
        //
        // Cycles: 2
        // States: 7
        // Addressing: register indirect
        // Flags: Z, S, P, CY, AC
        Opcode::ORA_M => reg.a = ior(reg.a, mem[reg.m()], flags),

        // ORI data (OR immediate)
        //   (A) ← (A) | (byte 2)
        //   The content of the second byte of the instruction is
        //   inclusive-OR'd with the content of the accumulator.
        //   The result is placed in the accumulator.
        //   The CY and AC flags are cleared.
        //
        // Cycles: 2
        // States: 7
        // Addressing: immediate
        // Flags: Z, S, P, CY, AC
        Opcode::ORI(d8) => reg.a = ior(reg.a, d8, flags),

        // CMP r (Compare register)
        //   (A) - (r)
        //   The content of register r is subtracted from the accumulator.
        //   The accumulator remains unchanged. The condition flags are set as
        //   a result of the subtraction. The Z flag is set to 1 if (A) = (r).
        //   The CY flag is set to 1 if (A) < (r).
        //
        // Cycles: 1
        // States: 4
        // Addressing: register
        // Flags: Z, S, P, CY, AC
        Opcode::CMP_A => flags.upd(0, Some(false)),
        Opcode::CMP_B => cmp(reg.a, reg.b, flags),
        Opcode::CMP_C => cmp(reg.a, reg.c, flags),
        Opcode::CMP_D => cmp(reg.a, reg.d, flags),
        Opcode::CMP_E => cmp(reg.a, reg.e, flags),
        Opcode::CMP_H => cmp(reg.a, reg.h, flags),
        Opcode::CMP_L => cmp(reg.a, reg.l, flags),

        // CMP M (Compare memory)
        //   (A) - ((H) (L))
        //   The content of the memory location whose address is contained in
        //   the H and L registers is subtracted from the accumulator. The
        //   accumulator remains unchanged.
        //   The condition flags are set as a result of the subtraction.
        //   The Z flag is set to 1 if (A) = ((H) (L)).
        //   The CY flag is set to 1 if (A) < ((H) (L)).
        //
        // Cycles: 2
        // States: 7
        // Addressing: register indirect
        // Flags: Z, S, P, CY, AC
        Opcode::CMP_M => cmp(reg.a, mem[reg.m()], flags),

        // CPI data (Compare immediate)
        //   (A) - (byte 2)
        //   The content of the second byte of the instruction is subtracted
        //   from the accumulator. The condition flags are set by the result
        //   of the subtraction. The Z flag is set to 1 if (A) = (byte 2).
        //   The CY flag is set to 1 if (A) < (byte 2).
        Opcode::CPI(d8) => cmp(reg.a, d8, flags),

        // RLC (Rotate left)
        //   (A_{n+1}) ← (A_n)
        //   (A_O) ← (A_7)
        //   (CY) ← (A_7)
        //   The content of the accumulator is rotated left one position.
        //   The low order bit and the CY flag are both set to the value
        //   shifted out of the high order bit position.
        //   Only the CY flag is affected.
        //
        // Cycles: 1
        // States: 4
        // Flags: CY
        Opcode::RLC => {
            const MSB: u8 = 0b1000_0000;
            flags.carry = (reg.a & MSB) == MSB;
            reg.a = (reg.a << 1) | ((reg.a & MSB) >> 7);
        },

        // RRC (Rotate right)
        //   (A_n) ← (A_{n-1})
        //   (A_7) ← (A_O)
        //   (CY) ← (A_O)
        //   The content of the accumulator is rotated right one position.
        //   The high order bit and the CY flag are both set to the value
        //   shifted out of the low order bit position.
        //   Only the CY flag is affected.
        //
        // Cycles: 1
        // States: 4
        // Flags: CY
        Opcode::RRC => {
            const LSB: u8 = 0b0000_0001;
            flags.carry = (reg.a & LSB) == LSB;
            reg.a = (reg.a >> 1) | ((reg.a & LSB) << 7);
        },

        // RAL (Rotate left through carry)
        //   (A_{n+1}) ← (A_n)
        //   (CY) ← (A_7)
        //   (A_O) ← (CY)
        //   The content of the accumu lator is rotated left one position
        //   through the CY flag. The low order bit is set equal to the CY
        //   flag and the CY flag is set to the value shifted out of the
        //   high order bit. Only the CY flag is affected.
        //
        // Cycles: 1
        // States: 4
        // Flags: CY
        Opcode::RAL => {
            const MSB: u8 = 0b1000_0000;
            let cy = flags.carry;
            flags.carry = (reg.a & MSB) == MSB;
            reg.a = (reg.a << 1) | (cy as u8);
        },

        // RAR (Rotate right through carry)
        //   (A_n) ← (A_{n+1})
        //   (CY) ← (A_O)
        //   (A_7) ← (CY)
        //   The content of the accumulator is rotated right one position
        //   through the CY flag. The high order bit is set to the CY flag
        //   and the CY flag is set to the value shifted out of the low
        //   order bit. Only the CY flag is affected.
        //
        // Cycles: 1
        // States: 4
        // Flags: CY
        Opcode::RAR => {
            const LSB: u8 = 0b0000_0001;
            let cy = flags.carry;
            flags.carry = (reg.a & LSB) == LSB;
            reg.a = (reg.a >> 1) | ((cy as u8) << 7);
        },

        // CMA (Complement accumulator)
        //   (A) ← !(A)
        //   The contents of the accumulator are complemented (zero bits
        //   become 1, one bits become 0). No flags are affected.
        //
        // Cycles: 1
        // States: 4
        // Flags: CY
        Opcode::CMA => reg.a = !reg.a,

        // CMC (Complement carry)
        //   (CY) ← !(CY)
        //   The CY flag is complemented. No other flags are affected.
        //
        // Cycles: 1
        // States: 4
        // Flags: CY
        Opcode::CMC => flags.carry = !flags.carry,

        // STC (Set carry)
        //   (CY) ← 1
        //   The CY flag is set to 1. No other flags are affected.
        //
        // Cycles: 1
        // States: 4
        // Flags: CY
        Opcode::STC => flags.carry = true,

        //
        // Branch group
        //
        // This group of instructions alter normal sequential program flow.
        // Condition flags are not affected by any instruction in this group.
        //
        // The two types of branch instructions are unconditional and
        // conditional. Unconditional transfers simply perform the specified
        // operation on register PC (the program counter). Conditional
        // transfers examine the status of one of the four processor flags
        // to determine if the specified branch is to be executed.
        // The conditions that may be specified areas follows:
        //   NZ   not zero    (Z = 0)
        //   Z    zero        (2 = 1)
        //   NC   no carry    (CY = 0)
        //   C    carry       (CY = 1)
        //   PO   parity odd  (P = 0)
        //   PE   parity even (P = 1)
        //   P    plus        (S = 0)
        //   M    minus       (S = 1)
        //

        // JMP addr (Jump)
        //   (PC) ← (byte 3) (byte 2)
        //   Control is transferred to the instruction whose address is
        //   specified in byte 3 and byte 2 of the current instruction.
        //
        // Cycles: 3
        // States: 10
        // Addressing: immediate
        Opcode::JMP(adr) => mem.jump(adr, true),

        // JCOND addr (Conditional jump)
        //   If (COND)
        //     (PC) ← (byte 3) (byte 2)
        //   If the specified condition is true, control is transferred to the
        //   instruction whose address is specified in byte 3 and byte 2 of the
        //   current instruction; otherwise, control continues sequentially.
        //
        // Cycles: 3
        // States: 10
        // Addressing: immediate
        Opcode::JNZ(adr) => mem.jump(adr, !flags.zero),
        Opcode::JZ(adr)  => mem.jump(adr, flags.zero),
        Opcode::JNC(adr) => mem.jump(adr, !flags.carry),
        Opcode::JC(adr)  => mem.jump(adr, flags.carry),
        Opcode::JPO(adr) => mem.jump(adr, !flags.parity),
        Opcode::JPE(adr) => mem.jump(adr, flags.parity),
        Opcode::JP(adr)  => mem.jump(adr, !flags.sign),
        Opcode::JM(adr)  => mem.jump(adr, flags.sign),

        // CALL addr (Call)
        //   ((SP) - 1) ← (PCH)
        //   ((SP) - 2) ← (PCl)
        //   (SP) ← (SP) - 2
        //   (PC) ← (byte 3) (byte 2)
        //   The high-order eight bits of the next instruction address are
        //   moved to the memory location whose address is one less than the
        //   content of register SP. The low-order eight bits of the next
        //   instruction address are moved to the memory location whose
        //   address is two less than the content of register SP.
        //   The content of register SP is decremented by 2.
        //   Control is transferred to the instruction whose address is
        //   specified in byte 3 and byte 2 of the current instruction.
        //
        // Cycles: 5
        // States: 17
        // Addressing: immediate/register indirect
        Opcode::CALL(adr) => mem.call(adr, true),

        // CCOND addr (Condition call)
        //   If (COND)
        //     ((SP) - 1) ← (PCH)
        //     ((SP) - 2) ← (PCl)
        //     (SP) ← (SP) - 2
        //     (PC) ← (byte 3) (byte 2)
        //   If the specified condition is true, the actions specified
        //   in the CAll instruction (see above) are performed;
        //   otherwise, control continues sequentially.
        //
        // Cycles: 3/5
        // States: 11/17
        // Addressing: immediate/register indirect
        Opcode::CNZ(adr) => mem.call(adr, !flags.zero),
        Opcode::CZ(adr)  => mem.call(adr, flags.zero),
        Opcode::CNC(adr) => mem.call(adr, !flags.carry),
        Opcode::CC(adr)  => mem.call(adr, flags.carry),
        Opcode::CPO(adr) => mem.call(adr, !flags.parity),
        Opcode::CPE(adr) => mem.call(adr, flags.parity),
        Opcode::CP(adr)  => mem.call(adr, !flags.sign),
        Opcode::CM(adr)  => mem.call(adr, flags.sign),

        // RET (Return)
        //   (PCl) ← ((SP));
        //   (PCH) ← ((SP) + 1);
        //   (SP) ← (SP) + 2;
        //   The content of the memory location whose address is specified in
        //   register SP is moved to the low-order eight bits of register PC.
        //   The content of the memory location whose address is one more than
        //   the content of register SP is moved to the high-order eight bits
        //   of register PC. The content of register SP is incremented by 2.
        //
        // Cycles: 3
        // States: 10
        // Addressing: register indirect
        Opcode::RET => mem.ret(true),

        // RCOND (Conditional return)
        //   If (COND)
        //     (PCl) ← ((SP))
        //     (PCH) ← ((SP) + 1)
        //     (SP) ← (SP) + 2
        //   If the specified condition is true, the actions specified in
        //   the RET instruction (see above) are performed; otherwise,
        //   control continues sequentially.
        //
        // Cycles: 1/3
        // States: 5/11
        // Addressing: register indirect
        Opcode::RNZ => mem.ret(!flags.zero),
        Opcode::RZ  => mem.ret(flags.zero),
        Opcode::RNC => mem.ret(!flags.carry),
        Opcode::RC  => mem.ret(flags.carry),
        Opcode::RPO => mem.ret(!flags.parity),
        Opcode::RPE => mem.ret(flags.parity),
        Opcode::RP  => mem.ret(!flags.sign),
        Opcode::RM  => mem.ret(flags.sign),

        // RST n (Restart)
        //   ((SP) - 1) ← (PCH)
        //   ((SP) - 2) ← (PCl)
        //   (SP) ← (SP) - 2
        //   (PC) ← 8 * (NNN)
        //   The high-order eight bits of the next instruction address are
        //   moved to the memory location whose address is one less than the
        //   content of register SP. The low-order eight bits of the next
        //   instruction address are moved to the memory location whose address
        //   is two less than the content of register SP. The content of
        //   register SP is decremented by two. Control is transferred to
        //   the instruction whose address is eight times the content of NNN.
        //
        // Program counter after restart:
        //   0 0 0 0 0 0 0 0 0 0 N N N 0 0 0
        //  MSB                           LSB
        //
        // Cycles: 3
        // States: 11
        // Addressing: register indirect
        Opcode::RST_0 => todo!(),
        Opcode::RST_1 => todo!(),
        Opcode::RST_2 => todo!(),
        Opcode::RST_3 => todo!(),
        Opcode::RST_4 => todo!(),
        Opcode::RST_5 => todo!(),
        Opcode::RST_6 => todo!(),
        Opcode::RST_7 => todo!(),

        // PCHL (Jump H and l indirect - move H and L to PC)
        //   (PCH) ← (H)
        //   (PCl) ← (l)
        //   The content of register H is moved to the high-order eight
        //   bits of register PC. The content of register l is moved to
        //   the low-order eight bits of register PC.
        //
        // Cycles: 1
        // States: 5
        // Addressing: register
        Opcode::PCHL => mem.set_pc(reg.h, reg.l),

        //
        // Stack, I/O, and machine control group
        //
        // This group of instructions performs I/O, manipulates the Stack,
        // and alters internal control flags. Unless otherwise specified,
        // condition flags are not affected by any instructions in this group.
        //

        // PUSH rp (Push)
        //   ((SP) - 1) ← (rh)
        //   ((SP) - 2) ← (rl)
        //   (SP) ← (SP) - 2
        //   The content of the high-order register of register pair rp is
        //   moved to the memory location whose address is one less than the
        //   content of register SP. The content of the low-order register of
        //   register pair rp is moved to the memory location whose address
        //   is two less than the content of register SP. The content of
        //   register SP is decremented by 2.
        //   Note: Register pair rp = SP may not be specified.
        //
        // Cycles: 3
        // States: 11
        // Addressing: register indirect
        // Flags: none
        Opcode::PUSH_BC => mem.push(reg.b, reg.c),
        Opcode::PUSH_DE => mem.push(reg.d, reg.e),
        Opcode::PUSH_HL => mem.push(reg.h, reg.l),

        // PUSH PSW (Push processor status word)
        //   ((SP) - 1) ← (A)
        //   ((SP) - 2)_0 ← (CY), ((SP) - 2)_1 ← 1
        //   ((SP) - 2)_2 ← (P),  ((SP) - 2)_3 ← 0
        //   ((SP) - 2)_4 ← (AC), ((SP) - 2)_5 ← 0
        //   ((SP) - 2)_6 ← (Z),  ((SP) - 2)_7 ← (S)
        //   (SP) ← (SP) - 2
        //   The content of register A is moved to the memory location
        //   whose address is one less than register SP. The contents of
        //   the condition flags are assembled into a processor status word
        //   and the word is moved to the memory location whose address is
        //   two less than the content of register SP. The content of
        //   register SP is decremented by two.
        //
        // Flag word:
        //   S Z 0 AC 0 P 1 CY
        //  MSB             LSB
        //
        // Cycles: 3
        // States: 11
        // Addressing: register indirect
        // Flags: none
        Opcode::PUSH_PSW => {
            let sp = mem.sp;
            mem[sp - 1] = reg.a;

            let mut psw = 0b0100_0000;
            psw |= 0b1000_0000 * (flags.sign as u8);
            psw |= 0b0100_0000 * (flags.zero as u8);
            psw |= 0b0001_0000 * (flags.carry_aux as u8);
            psw |= 0b0000_0100 * (flags.parity as u8);
            psw |= 0b0000_0001 * (flags.carry as u8);
            mem[sp - 2] = psw;
            mem.sp -= 2;
        },

        // POP rp (Pop)
        //   (rl) ← ((SP))
        //   (rh) ← ((SP) + 1)
        //   (SP) ← (SP) + 2
        //   The content of the memory location, whose address is specified by
        //   the content of register SP, is moved to the low-order register
        //   of register pair rp. The content of the memory location, whose
        //   address is one more than the content of register SP, is moved
        //   to the high-order register of register pair rp. The content
        //   of register SP is incremented by 2.
        //   Note: Register pair rp = SP may not be specified.
        //
        // Cycles: 3
        // States: 10
        // Addressing: register indirect
        // Flags: none
        Opcode::POP_BC => {
            reg.c = mem.read_sp8();
            reg.b = mem.read_sp8();
        },
        Opcode::POP_DE => {
            reg.e = mem.read_sp8();
            reg.d = mem.read_sp8();
        },
        Opcode::POP_HL => {
            reg.l = mem.read_sp8();
            reg.h = mem.read_sp8();
        },

        // POP PSW (Pop processor status word)
        //   (CY) ← ((SP))_O
        //   (P)  ← ((SP))_2
        //   (AC) ← ((SP))_4
        //   (Z)  ← ((SP))_6
        //   (S)  ← ((SP))_7
        //   (A)  ← ((SP) + 1)
        //   (SP) ← (SP) + 2
        //   The content of the memory location whose address is specified
        //   by the content of register SP is used to restore the condition
        //   flags. The content of the memory location whose address is one
        //   more than the content of register SP is moved to register A.
        //   The content of register SP is incremented by 2.
        //
        // Cycles: 3
        // States: 10
        // Addressing: register indirect
        // Flags: Z, S, P, CY, AC
        Opcode::POP_PSW => {
            let psw = mem.read_sp8();
            flags.sign      = (0b1000_0000 & psw) > 0;
            flags.zero      = (0b0100_0000 & psw) > 0;
            flags.carry_aux = (0b0001_0000 & psw) > 0;
            flags.parity    = (0b0000_0100 & psw) > 0;
            flags.carry     = (0b0000_0001 & psw) > 0;
            reg.a = mem.read_sp8();
        },

        // XTHL (Exchange stack top with H and L)
        //   (L) ← ((SP))
        //   (H) ← ((SP) + 1)
        //   The content of the L register is exchanged with the content of
        //   the memory location whose address is specified by the content of
        //   register SP. The content of the H register is exchanged with the
        //   content of the memory location whose address is one more than
        //   the content of register SP.
        //
        // Cycles: 5
        // States: 18
        // Addressing: register indirect
        // Flags: none
        Opcode::XTHL => todo!(),

        // SPHL (Move HL to SP)
        //   (SP) ← (H) (L)
        //   The contents of registers H and L are moved to register SP.
        //
        // Cycles: 1
        // States: 5
        // Addressing: register
        // Flags: none
        Opcode::SPHL => todo!(),

        // IN port (Input)
        //   (A) ← (data)
        //   The data placed on the eight bit bi-directional data bus
        //   by the specified port is moved to register A.
        //
        // Cycles: 3
        // States: 10
        // Addressing: direct
        // Flags: none
        Opcode::IN(d8) => println!("IN {}", d8),

        // OUT port (Output)
        //   (data) ← (A)
        //   The content of register A is placed on the eight bit bi-directional
        //   data bus for transmission to the specified port.
        //
        // Cycles: 3
        // States: 10
        // Addressing: direct
        // Flags: none
        Opcode::OUT(d8) => println!("OUT {}", d8),

        // EI (Enable interrupts)
        //   The interrupt system is enabled following the
        //   execution of the next instruction.
        //
        // Cycles: 1
        // States: 4
        // Flags: none
        Opcode::EI => flags.interrupt_enabled = true,

        // DI (Disable interrupts)
        //   The interrupt system is disabled immediately
        //   following the execution of the DI instruction.
        //
        // Cycles: 1
        // States: 4
        // Flags: none
        Opcode::DI => flags.interrupt_enabled = false,

        // HLT (Halt)
        //   The processor is stopped. The registers and flags are unaffected.
        //
        // Cycles: 1
        // States: 7
        // Flags: none
        Opcode::HLT => exit(0),

        // NOP (No op)
        //   No operation is performed. The registers and flags are unaffected.
        //
        // Cycles: 1
        // States: 4
        // Flags: none
        Opcode::NOP => {},
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let file = File::open(&args[1])?;
    let mut reader = BufReader::new(file);
    let mut mem = Memory::from_reader(&mut reader)?;

    let mut reg = Registers::default();
    let mut flags = Flags::default();

    loop {
        emulate(&mut reg, &mut flags, &mut mem);
    }
}
