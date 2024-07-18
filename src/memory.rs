use std::io::{BufReader, Read};

use crate::opcode::Opcode;

pub struct Memory {
    pub bytes_read: usize,
    /// ### Program Counter
    ///
    /// 16-bit program counter register (PCH and PCl are used to refer
    /// to the high-order and low-order 8 bits respectively).
    pub pc: u16,
    /// ### Stack Pointer
    ///
    /// 16-bit stack pointer register (SPH and SPL are used to refer
    /// to the high-order and low-order 8 bits respectively).
    pub sp: u16,
    mem: Vec<u8>,
}

impl Memory {
    pub fn from_reader<T: Read>(reader: &mut BufReader<T>) -> std::io::Result<Self> {
        let mut buf = Vec::with_capacity(0xFFFF + 1);
        let bytes_read = reader.read_to_end(&mut buf)?;
        buf.resize(0xFFFF + 1, 0);
        Ok(Memory { bytes_read, pc: 0, sp: 0, mem: buf })
    }

    pub fn set_sp(&mut self, rh: u8, rl: u8) {
        self.sp = ((rh as u16) << 8) | (rl as u16)
    }

    pub fn set_pc(&mut self, rh: u8, rl: u8) {
        self.pc = ((rh as u16) << 8) | (rl as u16)
    }

    pub fn get_pc(&self) -> (u8, u8) {
        let rh = (self.pc >> 8) as u8;
        let rl = (self.pc >> 0) as u8;
        (rh, rl)
    }

    pub fn read_pc8(&mut self) -> u8 {
        let res = self.mem[self.pc as usize];
        self.pc = self.pc.wrapping_add(1);
        res
    }

    pub fn read_pc16(&mut self) -> u16 {
        let rl = self.read_pc8();
        let rh = self.read_pc8();
        ((rh as u16) << 8) | (rl as u16)
    }

    pub fn read_sp8(&mut self) -> u8 {
        let res = self.mem[self.sp as usize];
        self.sp = self.sp.wrapping_add(1);
        res
    }

    pub fn jump(&mut self, adr: u16, cond: bool) {
        if cond {
            self.pc = adr;
        }
    }

    pub fn call(&mut self, adr: u16, cond: bool) {
        if cond {
            self.mem[self.sp.wrapping_sub(1) as usize] = (self.pc >> 8) as u8;
            self.mem[self.sp.wrapping_sub(2) as usize] = (self.pc >> 0) as u8;
            self.sp = self.sp.wrapping_sub(2);
            self.pc = adr;
        }
    }

    pub fn ret(&mut self, cond: bool) {
        if cond {
            let rl = self.mem[self.sp as usize];
            let rh = self.mem[self.sp as usize + 1];
            let adr = ((rh as u16) << 8) | (rl as u16);
            self.pc = adr;
            self.sp += 2;
        }
    }

    pub fn push(&mut self, rh: u8, rl: u8) {
        self.mem[self.sp.wrapping_sub(1) as usize] = rh;
        self.mem[self.sp.wrapping_sub(2) as usize] = rl;
        self.sp = self.sp.wrapping_sub(2);
    }

    pub fn read_opcode(&mut self) -> Opcode {
        match self.read_pc8() {
            0x00 => Opcode::NOP,
            0x01 => Opcode::LXI_BC(self.read_pc8(), self.read_pc8()),
            0x02 => Opcode::STAX_BC,
            0x03 => Opcode::INX_BC,
            0x04 => Opcode::INR_B,
            0x05 => Opcode::DCR_B,
            0x06 => Opcode::MVI_B(self.read_pc8()),
            0x07 => Opcode::RLC,
            0x08 => Opcode::NOP,
            0x09 => Opcode::DAD_BC,
            0x0a => Opcode::LDAX_BC,
            0x0b => Opcode::DCX_BC,
            0x0c => Opcode::INR_C,
            0x0d => Opcode::DCR_C,
            0x0e => Opcode::MVI_C(self.read_pc8()),
            0x0f => Opcode::RRC,
            0x10 => Opcode::NOP,
            0x11 => Opcode::LXI_DE(self.read_pc8(), self.read_pc8()),
            0x12 => Opcode::STAX_DE,
            0x13 => Opcode::INX_DE,
            0x14 => Opcode::INR_D,
            0x15 => Opcode::DCR_D,
            0x16 => Opcode::MVI_D(self.read_pc8()),
            0x17 => Opcode::RAL,
            0x18 => Opcode::NOP,
            0x19 => Opcode::DAD_DE,
            0x1a => Opcode::LDAX_DE,
            0x1b => Opcode::DCX_DE,
            0x1c => Opcode::INR_E,
            0x1d => Opcode::DCR_E,
            0x1e => Opcode::MVI_E(self.read_pc8()),
            0x1f => Opcode::RAR,
            0x20 => Opcode::NOP,
            0x21 => Opcode::LXI_HL(self.read_pc8(), self.read_pc8()),
            0x22 => Opcode::SHLD(self.read_pc16()),
            0x23 => Opcode::INX_HL,
            0x24 => Opcode::INR_H,
            0x25 => Opcode::DCR_H,
            0x26 => Opcode::MVI_H(self.read_pc8()),
            0x27 => Opcode::DAA,
            0x28 => Opcode::NOP,
            0x29 => Opcode::DAD_HL,
            0x2a => Opcode::LHLD(self.read_pc16()),
            0x2b => Opcode::DCX_HL,
            0x2c => Opcode::INR_L,
            0x2d => Opcode::DCR_L,
            0x2e => Opcode::MVI_L(self.read_pc8()),
            0x2f => Opcode::CMA,
            0x30 => Opcode::NOP,
            0x31 => Opcode::LXI_SP(self.read_pc8(), self.read_pc8()),
            0x32 => Opcode::STA(self.read_pc16()),
            0x33 => Opcode::INX_SP,
            0x34 => Opcode::INR_M,
            0x35 => Opcode::DCR_M,
            0x36 => Opcode::MVI_M(self.read_pc8()),
            0x37 => Opcode::STC,
            0x38 => Opcode::NOP,
            0x39 => Opcode::DAD_SP,
            0x3a => Opcode::LDA(self.read_pc16()),
            0x3b => Opcode::DCX_SP,
            0x3c => Opcode::INR_A,
            0x3d => Opcode::DCR_A,
            0x3e => Opcode::MVI_A(self.read_pc8()),
            0x3f => Opcode::CMC,
            0x40 => Opcode::MOV_BB,
            0x41 => Opcode::MOV_BC,
            0x42 => Opcode::MOV_BD,
            0x43 => Opcode::MOV_BE,
            0x44 => Opcode::MOV_BH,
            0x45 => Opcode::MOV_BL,
            0x46 => Opcode::MOV_BM,
            0x47 => Opcode::MOV_BA,
            0x48 => Opcode::MOV_CB,
            0x49 => Opcode::MOV_CC,
            0x4a => Opcode::MOV_CD,
            0x4b => Opcode::MOV_CE,
            0x4c => Opcode::MOV_CH,
            0x4d => Opcode::MOV_CL,
            0x4e => Opcode::MOV_CM,
            0x4f => Opcode::MOV_CA,
            0x50 => Opcode::MOV_DB,
            0x51 => Opcode::MOV_DC,
            0x52 => Opcode::MOV_DD,
            0x53 => Opcode::MOV_DE,
            0x54 => Opcode::MOV_DH,
            0x55 => Opcode::MOV_DL,
            0x56 => Opcode::MOV_DM,
            0x57 => Opcode::MOV_DA,
            0x58 => Opcode::MOV_EB,
            0x59 => Opcode::MOV_EC,
            0x5a => Opcode::MOV_ED,
            0x5b => Opcode::MOV_EE,
            0x5c => Opcode::MOV_EH,
            0x5d => Opcode::MOV_EL,
            0x5e => Opcode::MOV_EM,
            0x5f => Opcode::MOV_EA,
            0x60 => Opcode::MOV_HB,
            0x61 => Opcode::MOV_HC,
            0x62 => Opcode::MOV_HD,
            0x63 => Opcode::MOV_HE,
            0x64 => Opcode::MOV_HH,
            0x65 => Opcode::MOV_HL,
            0x66 => Opcode::MOV_HM,
            0x67 => Opcode::MOV_HA,
            0x68 => Opcode::MOV_LB,
            0x69 => Opcode::MOV_LC,
            0x6a => Opcode::MOV_LD,
            0x6b => Opcode::MOV_LE,
            0x6c => Opcode::MOV_LH,
            0x6d => Opcode::MOV_LL,
            0x6e => Opcode::MOV_LM,
            0x6f => Opcode::MOV_LA,
            0x70 => Opcode::MOV_MB,
            0x71 => Opcode::MOV_MC,
            0x72 => Opcode::MOV_MD,
            0x73 => Opcode::MOV_ME,
            0x74 => Opcode::MOV_MH,
            0x75 => Opcode::MOV_ML,
            0x76 => Opcode::HLT,
            0x77 => Opcode::MOV_MA,
            0x78 => Opcode::MOV_AB,
            0x79 => Opcode::MOV_AC,
            0x7a => Opcode::MOV_AD,
            0x7b => Opcode::MOV_AE,
            0x7c => Opcode::MOV_AH,
            0x7d => Opcode::MOV_AL,
            0x7e => Opcode::MOV_AM,
            0x7f => Opcode::MOV_AA,
            0x80 => Opcode::ADD_B,
            0x81 => Opcode::ADD_C,
            0x82 => Opcode::ADD_D,
            0x83 => Opcode::ADD_E,
            0x84 => Opcode::ADD_H,
            0x85 => Opcode::ADD_L,
            0x86 => Opcode::ADD_M,
            0x87 => Opcode::ADD_A,
            0x88 => Opcode::ADC_B,
            0x89 => Opcode::ADC_C,
            0x8a => Opcode::ADC_D,
            0x8b => Opcode::ADC_E,
            0x8c => Opcode::ADC_H,
            0x8d => Opcode::ADC_L,
            0x8e => Opcode::ADC_M,
            0x8f => Opcode::ADC_A,
            0x90 => Opcode::SUB_B,
            0x91 => Opcode::SUB_C,
            0x92 => Opcode::SUB_D,
            0x93 => Opcode::SUB_E,
            0x94 => Opcode::SUB_H,
            0x95 => Opcode::SUB_L,
            0x96 => Opcode::SUB_M,
            0x97 => Opcode::SUB_A,
            0x98 => Opcode::SBB_B,
            0x99 => Opcode::SBB_C,
            0x9a => Opcode::SBB_D,
            0x9b => Opcode::SBB_E,
            0x9c => Opcode::SBB_H,
            0x9d => Opcode::SBB_L,
            0x9e => Opcode::SBB_M,
            0x9f => Opcode::SBB_A,
            0xa0 => Opcode::ANA_B,
            0xa1 => Opcode::ANA_C,
            0xa2 => Opcode::ANA_D,
            0xa3 => Opcode::ANA_E,
            0xa4 => Opcode::ANA_H,
            0xa5 => Opcode::ANA_L,
            0xa6 => Opcode::ANA_M,
            0xa7 => Opcode::ANA_A,
            0xa8 => Opcode::XRA_B,
            0xa9 => Opcode::XRA_C,
            0xaa => Opcode::XRA_D,
            0xab => Opcode::XRA_E,
            0xac => Opcode::XRA_H,
            0xad => Opcode::XRA_L,
            0xae => Opcode::XRA_M,
            0xaf => Opcode::XRA_A,
            0xb0 => Opcode::ORA_B,
            0xb1 => Opcode::ORA_C,
            0xb2 => Opcode::ORA_D,
            0xb3 => Opcode::ORA_E,
            0xb4 => Opcode::ORA_H,
            0xb5 => Opcode::ORA_L,
            0xb6 => Opcode::ORA_M,
            0xb7 => Opcode::ORA_A,
            0xb8 => Opcode::CMP_B,
            0xb9 => Opcode::CMP_C,
            0xba => Opcode::CMP_D,
            0xbb => Opcode::CMP_E,
            0xbc => Opcode::CMP_H,
            0xbd => Opcode::CMP_L,
            0xbe => Opcode::CMP_M,
            0xbf => Opcode::CMP_A,
            0xc0 => Opcode::RNZ,
            0xc1 => Opcode::POP_BC,
            0xc2 => Opcode::JNZ(self.read_pc16()),
            0xc3 => Opcode::JMP(self.read_pc16()),
            0xc4 => Opcode::CNZ(self.read_pc16()),
            0xc5 => Opcode::PUSH_BC,
            0xc6 => Opcode::ADI(self.read_pc8()),
            0xc7 => Opcode::RST_0,
            0xc8 => Opcode::RZ,
            0xc9 => Opcode::RET,
            0xca => Opcode::JZ(self.read_pc16()),
            0xcb => Opcode::NOP,
            0xcc => Opcode::CZ(self.read_pc16()),
            0xcd => Opcode::CALL(self.read_pc16()),
            0xce => Opcode::ACI(self.read_pc8()),
            0xcf => Opcode::RST_1,
            0xd0 => Opcode::RNC,
            0xd1 => Opcode::POP_DE,
            0xd2 => Opcode::JNC(self.read_pc16()),
            0xd3 => Opcode::OUT(self.read_pc8()),
            0xd4 => Opcode::CNC(self.read_pc16()),
            0xd5 => Opcode::PUSH_DE,
            0xd6 => Opcode::SUI(self.read_pc8()),
            0xd7 => Opcode::RST_2,
            0xd8 => Opcode::RC,
            0xd9 => Opcode::NOP,
            0xda => Opcode::JC(self.read_pc16()),
            0xdb => Opcode::IN(self.read_pc8()),
            0xdc => Opcode::CC(self.read_pc16()),
            0xdd => Opcode::NOP,
            0xde => Opcode::SBI(self.read_pc8()),
            0xdf => Opcode::RST_3,
            0xe0 => Opcode::RPO,
            0xe1 => Opcode::POP_HL,
            0xe2 => Opcode::JPO(self.read_pc16()),
            0xe3 => Opcode::XTHL,
            0xe4 => Opcode::CPO(self.read_pc16()),
            0xe5 => Opcode::PUSH_HL,
            0xe6 => Opcode::ANI(self.read_pc8()),
            0xe7 => Opcode::RST_4,
            0xe8 => Opcode::RPE,
            0xe9 => Opcode::PCHL,
            0xea => Opcode::JPE(self.read_pc16()),
            0xeb => Opcode::XCHG,
            0xec => Opcode::CPE(self.read_pc16()),
            0xed => Opcode::NOP,
            0xee => Opcode::XRI(self.read_pc8()),
            0xef => Opcode::RST_5,
            0xf0 => Opcode::RP,
            0xf1 => Opcode::POP_PSW,
            0xf2 => Opcode::JP(self.read_pc16()),
            0xf3 => Opcode::DI,
            0xf4 => Opcode::CP(self.read_pc16()),
            0xf5 => Opcode::PUSH_PSW,
            0xf6 => Opcode::ORI(self.read_pc8()),
            0xf7 => Opcode::RST_6,
            0xf8 => Opcode::RM,
            0xf9 => Opcode::SPHL,
            0xfa => Opcode::JM(self.read_pc16()),
            0xfb => Opcode::EI,
            0xfc => Opcode::CM(self.read_pc16()),
            0xfd => Opcode::NOP,
            0xfe => Opcode::CPI(self.read_pc8()),
            0xff => Opcode::RST_7,
        }
    }
}

impl std::ops::Index<u16> for Memory {
    type Output = u8;

    fn index(&self, index: u16) -> &u8 {
        &self.mem[index as usize]
    }
}

impl std::ops::IndexMut<u16> for Memory {
    fn index_mut(&mut self, index: u16) -> &mut u8 {
        &mut self.mem[index as usize]
    }
}
