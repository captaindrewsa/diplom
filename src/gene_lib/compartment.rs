use std::{path::Component, rc::{Rc, Weak}, cell::RefCell, borrow::{BorrowMut, Borrow}};

use super::database;


#[derive(Debug)]
pub struct Compartment{
    name: String,
    pub compounds: Vec<String>,
    pub enzymes: Vec<String>,
    pub par_comp: Option<Weak<RefCell<Compartment>>>,
    pub sub_comp: Vec<Weak<RefCell<Compartment>>>,
}
impl Compartment{
    pub fn new(name: &str)-> Self{
        Self { 
            name: name.to_string(), 
            compounds: Default::default(),
            enzymes: Default::default(), 
            par_comp: Default::default(), 
            sub_comp: Default::default() }
    }
    pub fn get_name(&mut self) -> String{
        self.name.clone()
    }

    pub fn get_sub(&mut self)->&Vec<std::rc::Weak<RefCell<Compartment>>> {
        &self.sub_comp
    }

    pub fn get_par(&mut self)-> &Option<std::rc::Weak<RefCell<Compartment>>>{
        &self.par_comp
    }

    pub fn compile(&mut self){
        for substance in self.enzymes.iter(){
            let name_substrat_product = database::get_reactions_db().unwrap()[substance].clone();
            if name_substrat_product[1].iter().all(|sub| {
                if sub.ends_with("IN"){
                    self.compounds.contains(&sub[..sub.len()-3].to_string())
                } else if sub.ends_with("UT"){
                    self.par_comp
                    .clone()
                    .unwrap()
                    .upgrade()
                    .unwrap()
                    .take()
                    .compounds
                    .contains(&sub[..sub.len()-3].to_string())
                } else {
                    false
                }
            }){
                for product in name_substrat_product[2].iter(){
                    if product.ends_with("IN"){
                        self.compounds.push(product[..product.len()-3].to_string());
                    } else {
                        self.par_comp
                            .clone()
                            .unwrap()
                            .upgrade()
                            .unwrap()
                            .take()
                            .compounds
                            .push(product[..product.len()-3].to_string());
                    }
                }
            } else {
                println!("Для '{}' не хватает входных параметров:\n{:?}", substance, name_substrat_product[1]);
            }
        }
    }
}
    


impl Default for Compartment {
    fn default() -> Self {
        Self { name: Default::default(), compounds: Default::default(), enzymes: Default::default(), par_comp: Default::default(), sub_comp: Default::default() }
    }
}

impl Clone for Compartment{
    fn clone(&self) -> Self {
        Self { name: self.name.clone(), compounds: self.compounds.clone(), enzymes: self.enzymes.clone(), par_comp: self.par_comp.clone(), sub_comp: self.sub_comp.clone() }
    }
}

