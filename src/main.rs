// Things this ALU can do
    // Add,
    // NegateA,
    // Eql,
    // And,
    // Or,
    // ShiftL,
    // ShiftR,
    // AMoreThanB,

struct AddressSpace {
    ram: [u16; 65536],
}
impl AddressSpace {
    fn get(&self, index: u16) -> u16 {
        self.ram[index as usize]
    }
    fn set(&mut self, index: u16, value: u16) {
        self.ram[index as usize] = value;
    }
}

struct CPU {
    /*
     * Registers and flags
     */
    // gp registers
    a: u16,
    b: u16,
    c: u16,
    d: u16,
    // "volatile" registers
    program_counter: u16,
    address_register: u16,
    // Flag Registers
    alu_opcode: u16,

    /*
     * External
     */
    address_space: AddressSpace
}
impl CPU {
    fn execute(&mut self, instruction: &Instruction) {
        //
        let mut bus = instruction.put;

        // should run all the time if we are not using if or if the value of the c register is true
        if !instruction.useif || self.c > 0 {
            // gp reg stuff
            if instruction.ra { bus = bus | self.a; }
            if instruction.rb { bus = bus | self.b; }
            if instruction.rc { bus = bus | self.d; }
            if instruction.rd { bus = bus | self.d; }
            if instruction.wa { self.a = bus; }
            if instruction.wb { self.b = bus; }
            if instruction.wc { self.d = bus; }
            if instruction.wd { self.d = bus; }

            // ALU Stuff // TODO: finish this
            if instruction.rprocout {}

            // flag stuff
            if instruction.wprogram_counter { self.program_counter = bus; }
            if instruction.waddress_register { self.address_register = bus; }
            if instruction.walu_opcode { self.alu_opcode = bus; }

            // RAM stuff // TODO: finish this
            if instruction.raddr { self.alu_opcode = bus; }
            if instruction.waddr { self.alu_opcode = bus; }
        }
    }
}

struct Instruction {
    // 8
    ra: bool,
    rb: bool,
    rc: bool,
    rd: bool,
    wa: bool,
    wb: bool,
    wc: bool,
    wd: bool,
    // 8
    rprocout: bool,
    wprogram_counter: bool,
    waddress_register: bool,
    walu_opcode: bool,
    raddr: bool,
    waddr: bool,
    useif: bool,

    // last 16
    put: u16,
}
impl Instruction {
    fn new_noop() -> Self {
        Self {
            ra: false,
            rb: false,
            rc: false,
            rd: false,
            wa: false,
            wb: false,
            wc: false,
            wd: false,
            rprocout: false,
            wprogram_counter: false,
            waddress_register: false,
            walu_opcode: false,
            raddr: false,
            waddr: false,
            useif: false,
            // there is one flag here called has put
            put: 0,
        }
    }
    // returns (Self, needs to use an arg)
    fn from_raw_flags(a: u16) -> (Self, bool) {
        let mut noop = Instruction::new_noop();
        noop.ra =                (a & 0b1) > 0;
        noop.rb =                (a & 0b10) > 0;
        noop.rc =                (a & 0b100) > 0;
        noop.rd =                (a & 0b1000) > 0;
        noop.wa =                (a & 0b10000) > 0;
        noop.wb =                (a & 0b100000) > 0;
        noop.wc =                (a & 0b1000000) > 0;
        noop.wd =                (a & 0b10000000) > 0;
        noop.rprocout =          (a & 0b100000000) > 0;
        noop.wprogram_counter =  (a & 0b1000000000) > 0;
        noop.waddress_register = (a & 0b10000000000) > 0;
        noop.walu_opcode =       (a & 0b100000000000) > 0;
        noop.raddr =             (a & 0b1000000000000) > 0;
        noop.waddr =             (a & 0b10000000000000) > 0;
        noop.useif =             (a & 0b100000000000000) > 0;
        let has_put =            (a & 0b1000000000000000) > 0;
        (noop, has_put)
    }
}


fn main() {
    println!("Hello, world!");
}
