use super::compartment::Compartment;
use std::{borrow::Borrow, cell::RefCell, ops::Deref, rc::Rc, rc::Weak};

#[derive(Debug)]
pub struct GeneSys {
    storage: Vec<Weak<RefCell<Compartment>>>,
    obj_storage: Vec<Rc<RefCell<Compartment>>>,
}
impl Default for GeneSys {
    fn default() -> Self {
        Self {
            storage: Vec::new(),
            obj_storage: Vec::new(),
        }
    }
}

impl GeneSys {
    pub fn create_compartments(&mut self, names: Vec<&str>) {
        for name in names {
            let tmp = Rc::new(RefCell::new(Compartment::new(name)));
            self.storage.push(Rc::downgrade(&tmp));
            self.obj_storage.push(tmp);
            /*Нужно хранить сторадж самих объектов и отдельно сторадж ссылок*/
        }
    }
    pub fn create_tree(&mut self, tree: Vec<(&str, Vec<&str>)>) {
        for (par_comp, subs_comps) in tree {
            for sub_comp in subs_comps {
                self.merge_par_and_sub(par_comp, sub_comp);
            }
        }
    }

    pub fn add_substance_to_compartment(&mut self, comp_name: &str, substance: &str) {
        for comp in self.storage.iter() {
            let tmp = comp.clone().upgrade();
            if let Some(value) = tmp {
                if value.borrow_mut().name.as_str() == comp_name {
                    if substance.starts_with("EC") {
                        value.borrow_mut().enzymes.push(substance.to_string());
                    } else if substance.starts_with("C") {
                        value.borrow_mut().compounds.push(substance.to_string());
                    } else {
                        println!("Неизвестный тип вещества{substance}");
                    }
                } else {
                    continue;
                }
            } else {
                println!("Что-то не так с компартментом");
                continue;
            }
        }
    }

    pub fn compile(&mut self) {
        for comp in self.storage.iter() {
            let tmp = comp.clone().upgrade();
            if let Some(value) = tmp {
                value.borrow_mut().compile();
            } else {
                println!("Невозможно вызвать метод Compile");
            }
        }
    }

    pub fn print_result(&mut self) {
        let mut out = String::new();
        let raz = "======================";
        for comp in self.storage.iter() {
            let mut tmp_out = String::new();
            let tmp_obj = comp.clone().upgrade();
            if let Some(value) = tmp_obj {
                let name = value.borrow_mut().name.to_string();
                let enzymes = value.borrow_mut().enzymes.clone();
                let compounds = value.borrow_mut().compounds.clone();
                tmp_out.push_str(
                    format!(
                        "Имя:{}\nСостав:\n\tЭнзимы:{:?}\n\tКомпаунды:{:?}\n{}\n",
                        name, enzymes, compounds, raz
                    )
                    .as_str(),
                );
                out.push_str(&tmp_out);
            } else {
                println!("Не удалось вывести состояние компартмента");
            }
        }
        println!("{}", out);
    }

    fn merge_par_and_sub(&mut self, par: &str, sub: &str) {
        for par_compart in self.storage.iter() {
            if par_compart
                .clone()
                .upgrade()
                .unwrap()
                .borrow_mut()
                .name
                .as_str()
                == par
            {
                for sub_comp in self.storage.iter() {
                    if sub_comp
                        .clone()
                        .upgrade()
                        .unwrap()
                        .borrow_mut()
                        .name
                        .as_str()
                        == sub
                    {
                        sub_comp.clone().upgrade().unwrap().borrow_mut().par_comp =
                            Some(par_compart.clone());

                        par_compart
                            .clone()
                            .upgrade()
                            .unwrap()
                            .borrow_mut()
                            .sub_comp
                            .push(Some(sub_comp.clone()));
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
