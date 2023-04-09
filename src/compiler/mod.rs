
use core::num::ParseIntError;

pub mod keywords;
use keywords::CompilerKeywords;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
enum CompilerStates{
    Command,
    Reg, 
    Skip,
    DSkip,
    Num,
    PosLabel,
    ContainerLabel,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompileErrors{
    PARSE_FAILED,
    UNKNOWN_LABEL,
    WRITE_TO_OUTPUT_FILE_FAILED,
    BAD_SYNTAX,
}

#[derive(Debug, Default)]
pub struct CompileErrorsList{
    pub parse_int_error: Option<ParseIntError>,
    pub unknown_label_name: Option<String>,
    pub output_file_failure: Option<std::io::Error>,
    pub bad_syntax_opcode: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
enum LabelType{
    Positional,
    Storage,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Label{
    name: String,
    pos: u64,
    ltype: LabelType,
}

pub fn compile(code: String, name: String) -> Result<Vec<i64>, (CompileErrors, CompileErrorsList)>{
    let raw = code.to_lowercase()
                    .chars() //hello fds cds fgd!
                    .collect::<String>()
                    .replace('\n', " ")
                    .split(' ')
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|c| {
                        c.to_string();
                        let c = c.trim();
                        c.to_string()
                    })
                    .filter(|s| !s.is_empty())
                    .map(|s| {
                        s.chars().filter(|&c| {
                            c != ' ' 
                        })
                        .collect::<String>()
                        .trim()
                        .to_string()
                    })
                    .collect::<Vec<String>>();

    let mut state = CompilerStates::Command;
    let mut stateQ: Vec<CompilerStates> = vec![];
    let mut ret: Vec<i64> = vec!{};
    let mut ret_len = 0;
    let mut removables: Vec<usize> = vec![]; 

    let mut labels: Vec<Label> = vec![]; 

    for s in raw.clone().iter(){
        match &s[..]{
            "nop" | "ret" | "add" | "set" | "cpy" | "eq" | "jmp" | "jcs" | "jcns" | "call" | "stop" 
            | "sub" | "zero" | "rcs" | "rcns" | "push" | "pull" | "pushal" | "pullal" => {ret_len += 4;},

            _ => ()
        }
        if s.chars().last().unwrap() == ':'{
            let mut s = s.clone();
            s.pop();
            println!("Found positional label {s}");
            labels.push(Label { name: s, pos: ret_len as u64 - 1, ltype: LabelType::Positional});
        }else if s.chars().last().unwrap() == ';'{
            let mut s = s.clone();
            s.pop();
            println!("Found storage label {s}");
            labels.push(Label { name: s, pos: ret_len as u64, ltype: LabelType::Storage});
        }
    }

    for (i, s) in raw.clone().iter().enumerate(){
         //For closure
        let stateQi = stateQ.clone();
        ret_len = ret.iter().len();
        
        let mut command_handler = |s: String| -> Result<(Vec<i64>, Vec<CompilerStates>), (CompileErrors, CompileErrorsList)>{
            let mut inner_ret = vec![];
            let mut inner_stateQ = stateQi.clone();

            match &s[..]{
                "nop" => {
                    for _ in 0..=3 {inner_ret.push(00)};
                }

                "add" => {
                    if raw.clone()[i + 1].chars().next().unwrap() == '#'{
                        inner_stateQ.push(CompilerStates::Reg);
                    }else{
                        return Err((CompileErrors::BAD_SYNTAX, CompileErrorsList{bad_syntax_opcode: Some(raw.clone().index(i + 1).to_string()), ..Default::default()}))
                    }
                    if raw.clone()[i + 2].chars().next().unwrap() == '#'{
                        inner_ret.push(01);
                        inner_stateQ.push(CompilerStates::Reg);
                    }
                    else if raw.clone()[i + 2].chars().next().unwrap() == '$'{
                        inner_ret.push(15);
                        inner_stateQ.push(CompilerStates::Num);
                    }
                    else{
                        inner_ret.push(15);
                        //return Err((CompileErrors::BAD_SYNTAX, CompileErrorsList{bad_syntax_opcode: Some(raw.clone().index(i + 2).to_string()), ..Default::default()}))
                        inner_stateQ.push(CompilerStates::ContainerLabel);
                    }
                    inner_stateQ.push(CompilerStates::Skip);
                }

                "set" => {
                    inner_ret.push(02);
                    if raw.clone()[i + 1].chars().next().unwrap() == '#'{
                        inner_stateQ.push(CompilerStates::Reg);
                    }else{
                        return Err((CompileErrors::BAD_SYNTAX, CompileErrorsList{bad_syntax_opcode: Some(raw.clone().index(i + 1).to_string()), ..Default::default()}))
                    }
                    if raw.clone()[i + 2].chars().next().unwrap() == '$'{
                        inner_stateQ.push(CompilerStates::Num);
                    }else{
                        //return Err((CompileErrors::BAD_SYNTAX, CompileErrorsList{bad_syntax_opcode: Some(raw.clone().index(i + 2).to_string()), ..Default::default()}))
                        inner_stateQ.push(CompilerStates::ContainerLabel);
                    }
                    inner_stateQ.push(CompilerStates::Skip);
                }

                "cpy" => {
                    inner_ret.push(03);
                    if raw.clone()[i + 1].chars().next().unwrap() == '#'{
                        inner_stateQ.push(CompilerStates::Reg);
                    }else{
                        return Err((CompileErrors::BAD_SYNTAX, CompileErrorsList{bad_syntax_opcode: Some(raw.clone().index(i + 1).to_string()), ..Default::default()}))
                    }
                    if raw.clone()[i + 2].chars().next().unwrap() == '#'{
                        inner_stateQ.push(CompilerStates::Reg);
                    }else{
                        return Err((CompileErrors::BAD_SYNTAX, CompileErrorsList{bad_syntax_opcode: Some(raw.clone().index(i + 2).to_string()), ..Default::default()}))
                    }
                    inner_stateQ.push(CompilerStates::Skip);
                }

                "jmp" => {
                    inner_ret.push(04);
                    if raw.clone()[i + 1].chars().next().unwrap() == '$'{
                        inner_stateQ.push(CompilerStates::Num);
                    }else{
                        inner_stateQ.push(CompilerStates::PosLabel);
                    }
                    inner_stateQ.push(CompilerStates::DSkip);
                }

                "eq" => {
                    inner_ret.push(06);
                    if raw.clone()[i + 1].chars().next().unwrap() == '#'{
                        inner_stateQ.push(CompilerStates::Reg);
                    }else{
                        return Err((CompileErrors::BAD_SYNTAX, CompileErrorsList{bad_syntax_opcode: Some(raw.clone().index(i + 1).to_string()), ..Default::default()}))
                    }
                    if raw.clone()[i + 2].chars().next().unwrap() == '#'{
                        inner_stateQ.push(CompilerStates::Reg);
                    }else{
                        //return Err((CompileErrors::BAD_SYNTAX, CompileErrorsList{bad_syntax_opcode: Some(raw.clone().index(i + 2).to_string()), ..Default::default()}))
                        inner_stateQ.push(CompilerStates::ContainerLabel);
                    }
                    inner_stateQ.push(CompilerStates::Skip);
                }

                "jcs" => {
                    inner_ret.push(05);
                    if raw.clone()[i + 1].chars().next().unwrap() == '$'{
                        inner_stateQ.push(CompilerStates::Num);
                    }else{
                        inner_stateQ.push(CompilerStates::PosLabel);
                    }
                    inner_stateQ.push(CompilerStates::DSkip);
                }

                "jcns" => {
                    inner_ret.push(07);
                    if raw.clone()[i + 1].chars().next().unwrap() == '$'{
                        inner_stateQ.push(CompilerStates::Num);
                    }else{
                        inner_stateQ.push(CompilerStates::PosLabel);
                    }
                    inner_stateQ.push(CompilerStates::DSkip);
                }

                "call" => {
                    inner_ret.push(08);
                    if raw.clone()[i + 1].chars().next().unwrap() == '$'{
                        inner_stateQ.push(CompilerStates::Num);
                    }else{
                        inner_stateQ.push(CompilerStates::PosLabel);
                    }
                    inner_stateQ.push(CompilerStates::DSkip);
                }

                "ret" => {
                    inner_ret.push(09);
                    for _ in 0..=2{inner_ret.push(0);}
                }

                "stop" => {
                    inner_ret.push(10);
                    for _ in 0..=2{inner_ret.push(0);}
                }

                "sub" => {
                    if raw.clone()[i + 1].chars().next().unwrap() == '#'{
                        inner_stateQ.push(CompilerStates::Reg);
                    }else{
                        return Err((CompileErrors::BAD_SYNTAX, CompileErrorsList{bad_syntax_opcode: Some(raw.clone().index(i + 1).to_string()), ..Default::default()}))
                    }
                    if raw.clone()[i + 2].chars().next().unwrap() == '#'{
                        inner_ret.push(11);
                        inner_stateQ.push(CompilerStates::Reg);
                    }else if raw.clone()[i + 2].chars().next().unwrap() == '$'{
                        inner_ret.push(16);
                        inner_stateQ.push(CompilerStates::Num);
                    }else{
                        inner_ret.push(16);
                        //return Err((CompileErrors::BAD_SYNTAX, CompileErrorsList{bad_syntax_opcode: Some(raw.clone().index(i + 2).to_string()), ..Default::default()}))
                        inner_stateQ.push(CompilerStates::ContainerLabel);
                    }
                    inner_stateQ.push(CompilerStates::Skip);
                }

                "zero" => {
                    inner_ret.push(12);
                    if raw.clone()[i + 1].chars().next().unwrap() == '#'{
                        inner_stateQ.push(CompilerStates::Reg);
                    }else{
                        return Err((CompileErrors::BAD_SYNTAX, CompileErrorsList{bad_syntax_opcode: Some(raw.clone().index(i + 1).to_string()), ..Default::default()}))
                    }

                    inner_stateQ.push(CompilerStates::DSkip);
                }

                "rcs" => {
                    inner_ret.push(13);
                    for _ in 0..=2{inner_ret.push(0);}
                }

                "rcns" => {
                    inner_ret.push(14);
                    for _ in 0..=2{inner_ret.push(0);}
                }

                "push" => {
                    inner_ret.push(17);
                    inner_stateQ.push(CompilerStates::Reg);
                    inner_stateQ.push(CompilerStates::DSkip);
                }

                "pull" => {
                    inner_ret.push(18);
                    inner_stateQ.push(CompilerStates::Reg);
                    inner_stateQ.push(CompilerStates::DSkip);
                }

                "pushal" => {
                    inner_ret.push(19);
                    for _ in 0..=2{inner_ret.push(0);}
                }

                "pullal" => {
                    inner_ret.push(20);
                    for _ in 0..=2{inner_ret.push(0);}
                }

                //Keywords
                "byte" => {
                    inner_stateQ.push(CompilerStates::Num);
                    removables.push(ret_len);
                }

                _ => {
                    if s.chars().last().unwrap() != ':' &&
                       s.chars().last().unwrap() != ';' 
                    {
                        inner_stateQ.push(CompilerStates::Num);
                    }
                }
            }
            Ok((inner_ret, inner_stateQ))
        };
        println!("Looking at {} with state {:?}", &s[..], state);
        match state{
            CompilerStates::Command => {
                let mut result = command_handler(s.clone())?;
                ret.append(&mut result.0);
                stateQ = result.1;
            }

            CompilerStates::Reg => {
                let s = s.chars()
                        .filter(|&c| !c.is_ascii_punctuation())
                        .collect::<String>();
                
                /*let reg = match s.parse::<u8>(){
                    Ok(r) => r, 
                    Err(e) => return Err((CompileErrors::PARSE_FAILED, CompileErrorsList{
                        parse_int_error: Some(e),
                        ..Default::default()
                    }))
                }; */

                let reg = match s.as_str(){
                    "gur0" => 0,
                    "gur1" => 1,
                    "gur2" => 2,
                    "gur3" => 3,
                    "gur4" => 4,
                    "gur5" => 5,
                    "gur6" => 6,
                    "gur7" => 7,
                    "gur8" => 8,
                    "gur9" => 9,
                    "gura" => 10,
                    "gurb" => 11,
                    "gurc" => 12,
                    "gurd" => 13,
                    "gure" => 14,
                    "gurf" => 15,
                    "rr0" => 16,
                    "rr1" => 17,
                    "rr2" => 18,
                    "rr3" => 19,
                    "rr4" => 20,
                    "rr5" => 21,
                    "cp" => 22,

                    _ => return Err((CompileErrors::BAD_SYNTAX, CompileErrorsList{bad_syntax_opcode: Some(s.to_string()), ..Default::default()}))
                };

                ret.push(reg);
                if i + 1 == raw.clone().len() && stateQ[0] == CompilerStates::Skip{
                    ret.push(0);
                }else if i + 1 == raw.clone().len() && stateQ[0] == CompilerStates::DSkip{
                    ret.push(0);
                    ret.push(0);
                }
            }

            CompilerStates::Num => {
                let s = s.chars()
                        .filter(|&c| c != '$')
                        .collect::<String>();
                let num = match s.parse::<i32>(){
                    Ok(r) => r, 
                    Err(e) => return Err((CompileErrors::PARSE_FAILED, CompileErrorsList{
                        parse_int_error: Some(e),
                        ..Default::default()
                    }))
                };  
                ret.push(num.into());
                if i + 1 == raw.clone().len() && stateQ[0] == CompilerStates::Skip{
                    ret.push(0);
                }else if i + 1 == raw.clone().len() && stateQ[0] == CompilerStates::DSkip{
                    ret.push(0);
                    ret.push(0);
                }
            }

            CompilerStates::Skip => {
                ret.push(0);
                let mut result = command_handler(s.clone())?;
                ret.append(&mut result.0);
                stateQ = result.1;
            }

            CompilerStates::DSkip =>{
                ret.push(0);
                ret.push(0);
                let mut result = command_handler(s.clone())?;
                ret.append(&mut result.0);
                stateQ = result.1;
            }

            CompilerStates::PosLabel =>{
                let label = match labels
                            .iter()
                            .find(|&l| &l.clone().name == s && &l.ltype == &LabelType::Positional){
                                Some(l) => l,
                                None => return Err((CompileErrors::UNKNOWN_LABEL, CompileErrorsList{unknown_label_name: Some(s.to_string()), ..Default::default()}))
                            };
                let &pos = &label.pos;
                ret.push((pos + 1).try_into().unwrap());   
                if i + 1 == raw.clone().len() && stateQ[0] == CompilerStates::Skip{
                    ret.push(0);
                }else if i + 1 == raw.clone().len() && stateQ[0] == CompilerStates::DSkip{
                    ret.push(0);
                    ret.push(0);
                }                
            }

            CompilerStates::ContainerLabel =>{
                let label = match labels
                        .iter()
                        .find(|&l| (&l.clone().name == s) && (&l.clone().ltype == &LabelType::Storage)){
                                Some(l) => l,
                                None => return Err((CompileErrors::UNKNOWN_LABEL, CompileErrorsList{unknown_label_name: Some(s.to_string()), ..Default::default()}))
                        };
                let &pos = &label.pos;
                match label.ltype{
                    LabelType::Storage => {
                        ret.push(match raw.clone()[(pos + 2) as usize]
                            .chars()
                            .filter(|&c| !c.is_ascii_punctuation() || c != '!')
                            .collect::<String>()
                            .parse::<i64>(){
                            Ok(i) => i,
                            Err(e) => {
                                println!("Failed to parse num {}", raw.clone()[(pos + 2) as usize]);
                                return Err((CompileErrors::PARSE_FAILED, CompileErrorsList{
                                parse_int_error: Some(e),
                                ..Default::default()
                                }))
                            }
                        });
                    }

                    _ => panic!("Impossible panic hit!") 
                }

                if i + 1 == raw.clone().len() && stateQ[0] == CompilerStates::Skip{
                    ret.push(0);
                }else if i + 1 == raw.clone().len() && stateQ[0] == CompilerStates::DSkip{
                    ret.push(0);
                    ret.push(0);
                }
            }

            _ => ()
        }

        if !stateQ.is_empty(){
            state = stateQ[0];
            stateQ.remove(0);
        }else{
            state = CompilerStates::Command;
        }
    }

    'finishing: for i in removables{
        ret.remove(i);
    }

    println!("{:?}", raw);
    println!("{:?}", ret);

    match write_into_file(ret.clone(), name){
        Ok(()) => (),
        Err(e) => return Err((CompileErrors::WRITE_TO_OUTPUT_FILE_FAILED, CompileErrorsList{output_file_failure: Some(e), ..Default::default()}))
    }

    Ok(ret)
}

use std::{fs::OpenOptions, io::{Write, Read}, path::Path, ops::Index};

fn write_into_file(data: Vec<i64>, name: String) -> Result<(), std::io::Error>{
    let file = match OpenOptions::new().truncate(true).write(true).create(true).open(name.clone()){
        Ok(f) => f,
        Err(e) => return Err(e)
    };
    file.set_len(0)?;
    let mut file = match OpenOptions::new().write(true).create(true).open(name.clone()){
        Ok(f) => f,
        Err(e) => return Err(e)
    };
    for num in data{
        file.write_all(format!("{} ", num).as_bytes())?;
    }
    Ok(())
}

pub fn read_file<P>(path: P) -> Result<Vec<i64>, std::io::Error> 
where P: AsRef<Path> 
{
    let mut file = match OpenOptions::new().read(true).open(path)
    {
        Ok(f) => f,
        Err(e) => return Err(e)
    };
    
    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents){
        return Err(e)
    }
 
    let ret = contents
                    .split(" ")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
                    .iter()
                    .filter(|&s| !&s.is_empty())
                    .map(|s| {
                        match s.parse::<i64>(){
                            Ok(r) => r,
                            Err(e) => panic!("Failed to parse a number, {:?}", e)
                        }
                    })
                    .collect::<Vec<i64>>();
    Ok(ret)
}