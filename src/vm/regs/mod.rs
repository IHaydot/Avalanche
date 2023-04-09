
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum VMRegisterType{
    Normal,
    SR0,
    SR1,
}

#[derive(Clone, Debug,  PartialEq, Eq)]
pub struct VMRegister{
    pub(super) register: VMRegisterE,
    pub(super) value: Option<i64>,
    pub(super) typ: VMRegisterType,
    pub(super) sr0_val: Option<u8>,
    pub(super) sr1_val: Option<Vec<i64>>
}

impl VMRegister{
    pub fn new(reg: VMRegisterE, regs: VMRegisters) -> Option<Self>{
        match reg{
            VMRegisterE::GUR0 => {
                return Some(Self{
                                    register: reg,
                                    value: Some(regs.GUR0), 
                                    typ: VMRegisterType::Normal, 
                                    sr0_val: None, 
                                    sr1_val: None
                                })
            }

            VMRegisterE::GUR1 => {
                return Some(Self{
                                    register: reg,
                                    value: Some(regs.GUR1), 
                                    typ: VMRegisterType::Normal, 
                                    sr0_val: None, 
                                    sr1_val: None
                                })
            }

            VMRegisterE::GUR2 => {
                return Some(Self{
                                    register: reg,
                                    value: Some(regs.GUR2), 
                                    typ: VMRegisterType::Normal, 
                                    sr0_val: None, 
                                    sr1_val: None
                                })
            }

            VMRegisterE::GUR3 => {
                return Some(Self{
                                    register: reg,
                                    value: Some(regs.GUR3), 
                                    typ: VMRegisterType::Normal, 
                                    sr0_val: None, 
                                    sr1_val: None
                                })
            }

            VMRegisterE::GUR4 => {
                return Some(Self{
                                    register: reg,
                                    value: Some(regs.GUR4), 
                                    typ: VMRegisterType::Normal, 
                                    sr0_val: None, 
                                    sr1_val: None
                                })
            }

            VMRegisterE::GUR5 => {
                return Some(Self{
                                    register: reg,
                                    value: Some(regs.GUR5), 
                                    typ: VMRegisterType::Normal, 
                                    sr0_val: None, 
                                    sr1_val: None
                                })
            }

            VMRegisterE::GUR6 => {
                return Some(Self{
                                    register: reg,
                                    value: Some(regs.GUR6), 
                                    typ: VMRegisterType::Normal, 
                                    sr0_val: None, 
                                    sr1_val: None
                                })
            }

            VMRegisterE::GUR7 => {
                return Some(Self{
                                    register: reg,
                                    value: Some(regs.GUR7), 
                                    typ: VMRegisterType::Normal, 
                                    sr0_val: None, 
                                    sr1_val: None
                                })
            }

            VMRegisterE::GUR8 => {
                return Some(Self{
                                    register: reg,
                                    value: Some(regs.GUR8), 
                                    typ: VMRegisterType::Normal, 
                                    sr0_val: None, 
                                    sr1_val: None
                                })
            }

            VMRegisterE::GUR9 => {
                return Some(Self{
                                    register: reg,
                                    value: Some(regs.GUR9), 
                                    typ: VMRegisterType::Normal, 
                                    sr0_val: None, 
                                    sr1_val: None
                                })
            }

            VMRegisterE::GURA => {
                return Some(Self{
                                    register: reg,
                                    value: Some(regs.GURA), 
                                    typ: VMRegisterType::Normal, 
                                    sr0_val: None, 
                                    sr1_val: None
                                })
            }

            VMRegisterE::GURB => {
                return Some(Self{
                                    register: reg,
                                    value: Some(regs.GURB), 
                                    typ: VMRegisterType::Normal, 
                                    sr0_val: None, 
                                    sr1_val: None
                                })
            }

            VMRegisterE::GURC => {
                return Some(Self{
                                    register: reg,
                                    value: Some(regs.GURC), 
                                    typ: VMRegisterType::Normal, 
                                    sr0_val: None, 
                                    sr1_val: None
                                })
            }

            VMRegisterE::GURD => {
                return Some(Self{
                                    register: reg,
                                    value: Some(regs.GURD), 
                                    typ: VMRegisterType::Normal, 
                                    sr0_val: None, 
                                    sr1_val: None
                                })
            }

            VMRegisterE::GURE => {
                return Some(Self{
                                    register: reg,
                                    value: Some(regs.GURE), 
                                    typ: VMRegisterType::Normal, 
                                    sr0_val: None, 
                                    sr1_val: None
                                })
            }

            VMRegisterE::GURF => {
                return Some(Self{
                                    register: reg,
                                    value: Some(regs.GURF), 
                                    typ: VMRegisterType::Normal, 
                                    sr0_val: None, 
                                    sr1_val: None
                                })
            }

            VMRegisterE::RR0 => {
                return Some(Self{
                                    register: reg,
                                    value: Some(regs.RR0), 
                                    typ: VMRegisterType::Normal, 
                                    sr0_val: None, 
                                    sr1_val: None
                                })
            }

            VMRegisterE::RR1 => {
                return Some(Self{
                                    register: reg,
                                    value: Some(regs.RR1), 
                                    typ: VMRegisterType::Normal, 
                                    sr0_val: None, 
                                    sr1_val: None
                                })
            }

            VMRegisterE::RR2 => {
                return Some(Self{
                                    register: reg,
                                    value: Some(regs.RR2), 
                                    typ: VMRegisterType::Normal, 
                                    sr0_val: None, 
                                    sr1_val: None
                                })
            }

            VMRegisterE::RR3 => {
                return Some(Self{
                                    register: reg,
                                    value: Some(regs.RR3), 
                                    typ: VMRegisterType::Normal, 
                                    sr0_val: None, 
                                    sr1_val: None
                                })
            }

            VMRegisterE::RR4 => {
                return Some(Self{
                                    register: reg,
                                    value: Some(regs.RR4), 
                                    typ: VMRegisterType::Normal, 
                                    sr0_val: None, 
                                    sr1_val: None
                                })
            }

            VMRegisterE::RR5 => {
                return Some(Self{
                                    register: reg,
                                    value: Some(regs.RR5), 
                                    typ: VMRegisterType::Normal, 
                                    sr0_val: None, 
                                    sr1_val: None
                                })
            }

            VMRegisterE::SR0 => {
                return Some(Self{
                                    register: reg,
                                    value: None, 
                                    typ: VMRegisterType::SR0, 
                                    sr0_val: Some(regs.SR0), 
                                    sr1_val: None
                                })
            }

            VMRegisterE::SR3 => {
                return Some(Self{
                                    register: reg,
                                    value: None, 
                                    typ: VMRegisterType::SR0, 
                                    sr0_val: Some(regs.SR3), 
                                    sr1_val: None
                                })
            }

            VMRegisterE::SR1 => {
                return Some(Self{
                                    register: reg,
                                    value: None, 
                                    typ: VMRegisterType::SR1, 
                                    sr0_val: None, 
                                    sr1_val: Some(regs.SR1)
                                })
            }

            VMRegisterE::SR4 => {
                return Some(Self{
                                    register: reg,
                                    value: None, 
                                    typ: VMRegisterType::SR1, 
                                    sr0_val: None, 
                                    sr1_val: Some(regs.SR4)
                                })
            }

            VMRegisterE::SR2 => {
                return Some(Self{
                                    register: reg,
                                    value: Some(regs.SR2), 
                                    typ: VMRegisterType::Normal, 
                                    sr0_val: None, 
                                    sr1_val: None
                                })
            }

            VMRegisterE::SR5 => {
                return Some(Self{
                                    register: reg,
                                    value: Some(regs.SR5), 
                                    typ: VMRegisterType::Normal, 
                                    sr0_val: None, 
                                    sr1_val: None
                                })
            }

            VMRegisterE::CP => {
                return Some(Self{
                                    register: reg,
                                    value: Some(regs.CP), 
                                    typ: VMRegisterType::Normal, 
                                    sr0_val: None, 
                                    sr1_val: None
                                })
            }

            VMRegisterE::CP => {
                return Some(Self{
                                    register: reg,
                                    value: Some(regs.CP), 
                                    typ: VMRegisterType::Normal, 
                                    sr0_val: None, 
                                    sr1_val: None
                                })
            }

            VMRegisterE::NUL => {
                return None
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum VMRegisterE{
    GUR0,
    GUR1,
    GUR2,
    GUR3,
    GUR4,
    GUR5,
    GUR6,
    GUR7,
    GUR8,
    GUR9,
    GURA,
    GURB,
    GURC,
    GURD,
    GURE,
    GURF,

    RR0,
    RR1,
    RR2,
    RR3,
    RR4,
    RR5,

    SR0,
    SR1,
    SR2,
    SR3,
    SR4,
    SR5,

    CP,
    NUL
}

impl From<i64> for VMRegisterE{
    fn from(n: i64) -> Self{
        return match n{
            0 => Self::GUR0,
            1 => Self::GUR1,
            2 => Self::GUR2,
            3 => Self::GUR3,
            4 => Self::GUR4,
            5 => Self::GUR5,
            6 => Self::GUR6,
            7 => Self::GUR7,
            8 => Self::GUR8,
            9 => Self::GUR9,
            10 => Self::GURA,
            11 => Self::GURB,
            12 => Self::GURC,
            13 => Self::GURD,
            14 => Self::GURE,
            15 => Self::GURF,

            16 => Self::RR0,
            17 => Self::RR1,
            18 => Self::RR2,
            19 => Self::RR3,
            20 => Self::RR4,
            21 => Self::RR5,

            22 => Self::SR0,
            23 => Self::SR1,
            24 => Self::SR2,
            25 => Self::SR3,
            26 => Self::SR4,
            27 => Self::SR5,

            28 => Self::CP,
            _ => Self::NUL
        }
    }
}

impl VMRegisterE{
    pub fn len() -> usize{
        28
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct VMRegisters{
    pub(super) GUR1: i64,
    pub(super) GUR0: i64,
    pub(super) GUR2: i64,
    pub(super) GUR3: i64,
    pub(super) GUR4: i64,
    pub(super) GUR5: i64,
    pub(super) GUR6: i64,
    pub(super) GUR7: i64,
    pub(super) GUR8: i64,
    pub(super) GUR9: i64,
    pub(super) GURA: i64,
    pub(super) GURB: i64,
    pub(super) GURC: i64,
    pub(super) GURD: i64,
    pub(super) GURE: i64,
    pub(super) GURF: i64,

    pub(super) RR0: i64,
    pub(super) RR1: i64,
    pub(super) RR2: i64,
    pub(super) RR3: i64,
    pub(super) RR4: i64,
    pub(super) RR5: i64,

    pub(super) SR0: u8,    //Flag register(FR)
    pub(super) SR1: Vec<i64>, //Return point register(RPR)
    pub(super) SR2: i64,
    pub(super) SR3: u8,    
    pub(super) SR4: Vec<i64>, 
    pub(super) SR5: i64,

    pub(super) CP: i64
}

impl VMRegisters{
    pub fn read_GUR(&mut self, n: i64) -> Option<i64>{
        match n{
            0 => Some(self.GUR0),
            1 => Some(self.GUR1),
            2 => Some(self.GUR2),
            3 => Some(self.GUR3),
            4 => Some(self.GUR4),
            5 => Some(self.GUR5),
            6 => Some(self.GUR6),
            7 => Some(self.GUR7),
            8 => Some(self.GUR8),
            9 => Some(self.GUR9),
            10 => Some(self.GURA),
            11 => Some(self.GURB),
            12 => Some(self.GURC),
            13 => Some(self.GURD),
            14 => Some(self.GURE),
            15 => Some(self.GURF),
            _ => None
        }
    }

    pub fn set_GUR(&mut self, n: i64, v: i64) -> Result<(), ()>{
        match n{
            0 => {
                self.GUR0 = v;
                Ok(())
            }
            1 => {
                self.GUR1 = v;
                Ok(())
            }
            2 => {
                self.GUR2 = v;
                Ok(())
            }
            3 => {
                self.GUR3 = v;
                Ok(())
            }
            4 => {
                self.GUR4 = v;
                Ok(())
            }
            5 => {
                self.GUR5 = v;
                Ok(())
            }
            6 => {
                self.GUR6 = v;
                Ok(())
            }
            7 => {
                self.GUR7 = v;
                Ok(())
            }
            8 => {
                self.GUR8 = v;
                Ok(())
            }
            9 => {
                self.GUR9 = v;
                Ok(())
            }
            10 => {
                self.GURA = v;
                Ok(())
            }
            11 => {
                self.GURB = v;
                Ok(())
            }
            12 => {
                self.GURC = v;
                Ok(())
            }
            13 => {
                self.GURD = v;
                Ok(())
            }
            14 => {
                self.GURE = v;
                Ok(())
            }
            15 => {
                self.GURF = v;
                Ok(())
            }
            _ => Err(())
        }
    }

    pub fn read_RR(&mut self, n: i64) -> Option<i64>{
        match n{
            16 => Some(self.RR0),
            17 => Some(self.RR1),
            18 => Some(self.RR2),
            19 => Some(self.RR3),
            20 => Some(self.RR4),
            21 => Some(self.RR5),
            _ => None
        }
    }

    pub fn set_sr0(&mut self, set: bool){
        if set == true{
            self.SR0 = 1;
        }else{
            self.SR0 = 0;
        }
    } 

    pub(super) fn set_RR(&mut self, n: i64, v: i64) -> Result<(), ()>{
        match n{
            16 => {
                self.RR0 = v;
                Ok(())
            }
            17 => {
                self.RR1 = v;
                Ok(())
            }
            18 => {
                self.RR2 = v;
                Ok(())
            }
            19 => {
                self.RR3 = v;
                Ok(())
            }
            20 => {
                self.RR4 = v;
                Ok(())
            }
            21 => {
                self.RR5 = v;
                Ok(())
            }
            _ => Err(())
        }
    }

    pub fn sr0_state(&mut self) -> bool{
        if self.SR0 != 0{true}else{false}
    }

    pub fn set_normal_register(&mut self, reg: i64, val: i64) -> Result<(), ()>{
        if reg <= 15{
            self.set_GUR(reg, val)?;
        }else if reg > 15 && reg <= 21{
            self.set_RR(reg, val)?;
        }else if reg == 27{
            self.SR5 = val;
        } else if reg == 24{
            self.SR2 = val;
        }
        Ok(())
    }
}
