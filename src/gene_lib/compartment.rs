use std::{
    cell::RefCell,
    path::Component,
    rc::{Rc, Weak}, ops::Deref,
};

use super::database;

#[derive(Debug)]
pub struct Compartment {
    pub name: String,
    pub compounds: Vec<String>,
    pub enzymes: Vec<String>,
    pub par_comp: Option<Weak<RefCell<Compartment>>>,
    pub sub_comp: Vec<Option<Weak<RefCell<Compartment>>>>,
}


impl Compartment {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            compounds: Vec::new(),
            enzymes: Vec::new(),
            par_comp: None,
            sub_comp: Vec::new(),
        }
    }
    pub fn get_name(&mut self) -> String {
        self.name.clone()
    }

    fn get_sub(&mut self) -> Vec<Option<std::rc::Weak<RefCell<Compartment>>>> {
        self.sub_comp.clone()
    }

    fn get_par(&self) -> Option<Weak<RefCell<Compartment>>> {
        self.par_comp.clone()
    }
    fn check_substrate(&self, sub: &String) -> bool {
        if sub.ends_with("IN") {
            self.compounds.contains(&sub[..sub.len() - 3].to_string())
        } else if sub.ends_with("UT") {
            self.par_comp
            .clone()
            .unwrap()
            .upgrade()
            .unwrap()
            .try_borrow_mut().ok()
            .unwrap()
            .compounds.contains(&sub[..sub.len() - 3].to_string())
        } else {
            false
        }
    }

    pub fn compile(&mut self) {
        for substance in self.enzymes.iter() {
            //Перебираем все энзимы в компартменте
            let substrat_product = database::get_reactions_db().unwrap()[substance].clone(); //Запрашиваем реакцию для энзима
            if substrat_product[0]
                .iter()
                .all(|sub| self.check_substrate(sub))
            {
                for product in substrat_product[1].iter() {
                    if product.ends_with("IN") {
                        self.compounds
                            .push(product[..product.len() - 3].to_string());
                    } else if product.ends_with("UT") {
                        self.par_comp
                            .clone()
                            .unwrap()
                            .upgrade()
                            .unwrap()
                            .try_borrow_mut().ok()
                            .unwrap()
                            .compounds.push(product[..product.len()-3].to_string());
                    } else {
                        println!("Окончания в бд неверные");
                    }
                }
            } else {
                println!(
                    "Для '{}' не хватает входных параметров:\n{:?}",
                    substance, substrat_product[0]
                );
            }
        }
    }
}

impl Default for Compartment {
    fn default() -> Self {
        Self {
            name: Default::default(),
            compounds: Default::default(),
            enzymes: Default::default(),
            par_comp: Default::default(),
            sub_comp: Default::default(),
        }
    }
}

impl Clone for Compartment {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            compounds: self.compounds.clone(),
            enzymes: self.enzymes.clone(),
            par_comp: self.par_comp.clone(),
            sub_comp: self.sub_comp.clone(),
        }
    }
}
