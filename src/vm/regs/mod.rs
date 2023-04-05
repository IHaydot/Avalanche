

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
    pub(super) SR2: i32,
    pub(super) SR3: u8,    
    pub(super) SR4: Vec<u64>, 
    pub(super) SR5: i32,

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
}
