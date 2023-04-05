
pub mod regs;
pub mod states;
pub mod inst;
use std::{io::Write, borrow::BorrowMut};

use inst::VMInstructions;
use regs::VMRegisters;
use states::VMStates;


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VMInstance{
    pub(crate) regs: VMRegisters,
    CP: i64,
    program: Option<Vec<i64>>,
    state: VMStates,
    debug: bool
}

impl Default for VMInstance{
    fn default() -> Self{
        VMInstance{
            regs: VMRegisters::default(),
            CP: 0,
            program: None,
            state: VMStates::NO_PROGRAM,
            debug: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum VMRunErrors{
    UNITILIZED_INSTANCE,
    BAD_PROGRAM,
}

impl VMInstance{
    pub fn set_program(&mut self, program: Vec<i64>){
        if program.is_empty(){ self.state = VMStates::NO_PROGRAM; }

        self.program = Some(program);
        self.regs = VMRegisters::default();
        self.CP = 0;
        self.state = VMStates::IDLE;
    }

    fn next_8_bits(&mut self) -> i64{
        self.CP += 1;
        self.program.clone().unwrap()[self.CP as usize]
    }

    fn next_16_bits(&mut self) -> u16{
        self.CP += 1;
        let ret = ((self.program.clone().unwrap()[self.CP as usize] as u16) << 8) | self.program.clone().unwrap()[self.CP as usize + 1] as u16;
        self.CP += 1;
        ret
    } //OLD

    pub fn state(self) -> VMStates{
        self.state
    }

    pub fn regs(self) -> VMRegisters{
        self.regs
    }

    fn inner_run(&mut self, run_once: bool) -> Result<(), VMRunErrors>{
        if self.state != VMStates::IDLE ||
           self.program.is_none() || 
           self.program.clone().unwrap().is_empty()
        {
            self.state = VMStates::UNITILIZED;
            return Err(VMRunErrors::UNITILIZED_INSTANCE)
        }

        self.regs.CP = self.CP;
        self.state = VMStates::ACTIVE;

        let mut running = true;

        'main: while running{
            if self.CP >= self.program.clone().unwrap().len() as i64{
                break 'main;
            }

            let instr = VMInstructions::from(self.program.clone().unwrap()[self.CP as usize]);

            match instr{
                VMInstructions::NOP => {
                    if self.debug == true{
                        println!("Looking at NOP at position {} with reg states:\n{:?}",
                            	    self.CP.clone(),
                                    self.clone().regs()
                                );
                    }
                    self.next_16_bits();
                    self.next_8_bits();
                    self.CP += 1;
                },

                VMInstructions::ADD => {
                    
                    let reg1 = self.next_8_bits();
                    let reg2 = self.next_8_bits();
                    self.next_8_bits();
                    self.regs.RR0 = self.regs.read_GUR(reg1).unwrap() + self.regs.read_GUR(reg2).unwrap(); 
                    if self.debug == true{
                        println!("Looking at ADD at position {} with reg states:\n{:?}\nReg1:{},Reg2:{},Result:{}",
                            	    self.CP.clone(),
                                    self.clone().regs(),
                                    reg1,
                                    reg2,
                                    self.regs.clone().read_RR(6).unwrap()
                                );
                    }
                    self.CP += 1;
                }

                VMInstructions::ADD1 => {
                    let reg = self.next_8_bits();
                    let num = self.next_8_bits();
                    self.next_8_bits();
                    self.regs.RR0 = self.regs.read_GUR(reg).unwrap() + num;
                    if self.debug == true{
                        println!("Looking at ADD at position {} with reg states:\n{:?}\nReg:{},Num:{},Result:{}",
                            	    self.CP.clone(),
                                    self.clone().regs(),
                                    reg,
                                    num,
                                    self.regs.clone().read_RR(6).unwrap()
                                );
                    }
                    self.CP += 1;
                }

                VMInstructions::SET => {
                    let reg = self.next_8_bits();
                    let val = self.next_8_bits(); 
                    self.next_8_bits();

                    self.regs.set_GUR(reg, val).unwrap();

                    if self.debug == true{
                        println!("Looking at SET at position {} with reg states:\n{:?}\nReg:{},Val:{}",
                            	    self.CP.clone(),
                                    self.clone().regs(),
                                    reg,
                                    val,
                                )
                    }

                    self.CP += 1;
                }
            
                VMInstructions::JMP => {
                    let pos = self.next_8_bits();
                    self.CP = pos;
                    if self.debug == true{
                        println!("Looking at JMP at position {} with reg states:\n{:?}\nPos:{}",
                            	    self.CP.clone(),
                                    self.clone().regs(),
                                    pos
                                )
                    }
                }

                VMInstructions::CPY => {
                    let cpy = self.next_8_bits();
                    let pst = self.next_8_bits();
                    self.next_8_bits();
                    
                    if cpy <= 15 && pst <= 15{
                        self.regs.set_GUR(pst, self.regs.clone().read_GUR(cpy).unwrap()).unwrap();
                    }else if cpy > 15 && pst <= 15{
                        self.regs.set_GUR(pst, self.regs.clone().read_RR(cpy).unwrap()).unwrap();
                    }

                    if self.debug == true{
                        println!("Looking at CPY at position {} with reg states:\n{:?}\nCPY:{},PST:{}",
                            	    self.CP.clone(),
                                    self.clone().regs(),
                                    cpy,
                                    pst
                                );
                    }
                    self.CP += 1;
                }

                VMInstructions::EQ => {
                    let reg1 = self.next_8_bits();
                    let reg2 = self.next_8_bits();
                    self.next_8_bits();
                    
                    let num1 = if reg1 <= 5{
                        self.regs.clone().read_GUR(reg1).unwrap()
                    }else{
                        self.regs.clone().read_RR(reg1).unwrap()
                    };

                    let num2 = if reg2 <= 5{
                        self.regs.clone().read_GUR(reg2).unwrap()
                    }else{
                        self.regs.clone().read_RR(reg2).unwrap()
                    };

                    self.regs.set_sr0(if num1 == num2{true}else{false});
                    
                    if self.debug == true{
                        println!("Looking at EQ at position {} with reg states:\n{:?}\nReg1:{},Reg2:{},Result:{}",
                            self.CP.clone(),
                            self.clone().regs(),
                            reg1,
                            reg2,
                            self.regs.clone().sr0_state()
                        );
                    }
                    self.CP += 1;
                }

                VMInstructions::JCS => {
                    let pos = self.next_8_bits();
                    self.next_16_bits();
                    if self.regs.sr0_state() == true{
                        self.CP = pos;
                    }else{
                        self.CP += 1;
                    }
                    if self.debug == true{
                        println!("Looking at JCS at position {} with reg states:\n{:?}\nPos:{}",
                            	    self.CP.clone(),
                                    self.clone().regs(),
                                    pos
                                );
                    }
                }

                VMInstructions::JCNS => {
                    let pos = self.next_8_bits();
                    self.next_16_bits();
                    if self.regs.sr0_state() == false{
                        self.CP = pos;
                    }else{
                        self.CP += 1;
                    }
                    if self.debug == true{
                        println!("Looking at JCNS at position {} with reg states:\n{:?}\nPos:{}",
                            	    self.CP.clone(),
                                    self.clone().regs(),
                                    pos
                                );
                    }
                }

                VMInstructions::CALL => {
                    self.regs.SR1.push(self.CP + 4); 
                    self.CP = self.next_8_bits();
                    if self.debug == true{
                        println!("Looking at CALL at position {} with reg states:\n{:?}\nPos:{};RetPoint:{}",
                            	    self.CP.clone(),
                                    self.clone().regs(),
                                    self.CP,
                                    self.regs.SR1[0]
                                );
                    }
                }

                VMInstructions::RET => {
                    if !self.regs.SR1.is_empty(){
                        self.CP = self.regs.SR1[0];
                        self.regs.SR1.remove(0);
                    }else{
                        self.state = VMStates::BAD_PROGRAM;
                        println!("Found RET without a return point!\nCP:{}",
                            self.CP    
                        );
                        return Err(VMRunErrors::BAD_PROGRAM)
                    }
                    if self.debug == true{
                        println!("Looking at RET at position {} with reg states:\n{:?}\nPos:{}",
                            	    self.CP.clone(),
                                    self.clone().regs(),
                                    self.CP,
                                );
                    }
                }

                VMInstructions::STOP => {
                    running = false;

                    println!("Found STOP! Stopping the virtual machine...");

                    if self.debug == true{
                        println!("Looking at STOP at position {} with reg states:\n{:?}\n",
                            	    self.CP.clone(),
                                    self.clone().regs()
                                );
                    }
                }

                VMInstructions::SUB => {
                    let reg1 = self.next_8_bits();
                    let reg2 = self.next_8_bits();
                    self.next_8_bits();
                    self.regs.RR0 = self.regs.read_GUR(reg1).unwrap() - self.regs.read_GUR(reg2).unwrap();
                    if self.debug == true{
                        println!("Looking at SUB at position {} with reg states:\n{:?}\nReg1:{},Reg2:{}",
                            	    self.CP.clone(),
                                    self.clone().regs(),
                                    reg1,
                                    reg2
                                );
                    }
                    self.CP += 1;
                }

                VMInstructions::SUB1 => {
                    let reg = self.next_8_bits();
                    let num = self.next_8_bits();
                    self.next_8_bits();
                    self.regs.RR0 = self.regs.read_GUR(reg).unwrap() - num;
                    if self.debug == true{
                        println!("Looking at SUB at position {} with reg states:\n{:?}\nReg:{},Num:{}",
                            	    self.CP.clone(),
                                    self.clone().regs(),
                                    reg,
                                    num,
                                );
                    }
                    self.CP += 1;
                }

                VMInstructions::ZERO => {
                    let reg1 = self.next_8_bits();
                    self.next_16_bits();
                    if reg1 <= 5{
                        self.regs.clone().set_GUR(reg1, 0).unwrap();
                    }else{
                        self.regs.clone().set_RR(reg1, 0).unwrap()
                    };
                    if self.debug == true{
                        println!("Looking at ZERO at position {} with reg states:\n{:?}\nReg:{}",
                        self.CP.clone(),
                        self.clone().regs(),
                        reg1,
                        );
                    }
                    self.CP += 1;
                }

                VMInstructions::RCS => {
                    if !self.regs.SR1.is_empty() && self.regs.sr0_state() == true{
                        self.CP = self.regs.SR1[0];
                        self.regs.SR1.remove(0);
                    }else{
                        self.next_16_bits();
                        self.next_8_bits();
                        self.CP += 1;
                    }

                    if self.debug == true{
                        println!("Looking at RCS at position {} with reg states:\n{:?}\nSR0:{}",
                            	    self.CP.clone(),
                                    self.clone().regs(),
                                    self.regs.sr0_state()
                                );
                    }
                }

                VMInstructions::RCNS => {
                    if !self.regs.SR1.is_empty() && self.regs.sr0_state() != true{
                        self.CP = self.regs.SR1[0];
                        self.regs.SR1.remove(0);
                    }else{
                        self.next_16_bits();
                        self.next_8_bits();
                        self.CP += 1;
                    }

                    if self.debug == true{
                        println!("Looking at RCNS at position {} with reg states:\n{:?}\nSR0:{}",
                            	    self.CP.clone(),
                                    self.clone().regs(),
                                    self.regs.sr0_state()
                                );
                    }
                }

                _ => {
                    self.state = VMStates::BAD_PROGRAM;
                    println!("Unexpected failure while executing program\nopcode:{}, CP:{}",
                            self.program.clone().unwrap()[self.CP as usize],
                            self.CP    
                    );
                    return Err(VMRunErrors::BAD_PROGRAM)
                }
            }

            self.regs.CP = self.CP;
            if run_once == true {running = false};
        }
        
        self.state = VMStates::IDLE;

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), VMRunErrors>{
        self.inner_run(false)
    }

    pub fn debug(&mut self) -> Result<(), VMRunErrors>{
        self.debug = true;
        let mut ret: Option<Result<(), VMRunErrors>> = None;
        while self.CP != self.program.iter().len() as i64{
            ret = Some(self.inner_run(true));
            let mut _s = String::new();
            let _ = std::io::stdin().read_line(&mut _s);
            std::io::stdout().flush().unwrap();
        }
        self.debug = false;
        ret.unwrap()
    }
}