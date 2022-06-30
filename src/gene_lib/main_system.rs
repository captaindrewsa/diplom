use std::{rc::Rc, cell::RefCell, borrow::Borrow};
use super::compartment::Compartment;

#[derive(Debug)]
pub struct GeneSys{
    storage: Vec<Rc<RefCell<Compartment>>>,

}
impl Default for GeneSys {
    fn default() -> Self {
        Self { storage: Default::default() }
    }
}

impl GeneSys {
    pub fn create_compartments(&mut self, names: Vec<&str>){
        for name in names{
            self.storage.push(Rc::new(RefCell::new(Compartment::new(name))));
        }
    }
    pub fn create_tree(&mut self, tree: Vec<(&str, Vec<&str>)>){
        for (par_comp, subs_comps) in tree{
            for sub_comp in subs_comps{
                self.merge_par_and_sub(par_comp, sub_comp);
            }
        }
    }

    pub fn add_substance_to_compartment(&mut self, comp_name: &str, substance: &str){
        for comp in self.storage.iter(){
            if comp.borrow_mut().get_name() == comp_name{
                if substance.starts_with("EC"){
                    comp.borrow_mut().enzymes.push(substance.to_string());
                } else if substance.starts_with("C") {
                    comp.borrow_mut().compounds.push(substance.to_string());
                }
            }
        }
    }

    pub fn compile(&mut self){
        for comp in self.storage.iter(){
            comp.borrow_mut().compile();
        }
    }

    pub fn print_result(&mut self){
        let mut out = String::new();
        let raz = "======================";
        for comp in self.storage.iter(){
            let mut tmp = String::new();
            let name = comp.borrow_mut().get_name();
            let enzymes = comp.borrow_mut().enzymes.clone();
            let compounds = comp.borrow_mut().compounds.clone();
            tmp.push_str(format!("Имя:{}\nСостав:\n\tЭнзимы:{:?}\n\tКомпаунды:{:?}\n{}\n",name,enzymes,compounds, raz).as_str());
            out.push_str(&tmp);
        }
        println!("{}",out);
    }

    fn merge_par_and_sub(&mut self, par: &str, sub: &str){
        for par_comp in self.storage.iter(){
            if par_comp.borrow_mut().get_name().as_str() == par{
                for sub_comp in self.storage.iter(){
                    if sub_comp.borrow_mut().get_name().as_str() == sub{
                        sub_comp.borrow_mut().par_comp = Some(Rc::downgrade(par_comp));
                        par_comp.borrow_mut().sub_comp.push(Rc::downgrade(sub_comp));
                        println!("Компартменты '{}' и '{}' найдены и связаны", &par, &sub);
                    } else {
                        continue;
                    }
                }
            } else {
                continue;
            }
        }
    }
}
