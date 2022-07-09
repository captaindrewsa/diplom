// use mysql::*;
// use mysql::prelude::*;
// use mysql::Opts;
// use mysql::serde::de::Error;
use std::{
    collections::{HashMap, HashSet, BTreeMap},
    fs::{self, File},
    io::{Read, Result}, vec,
};

pub fn get_reactions_db() -> Result<BTreeMap<String, Vec<Vec<String>>>> {
    let mut content = String::new(); //Создаем Стринг для данных из файла
    File::open("/home/captaindrewsa/Programming/Rust/diplom/src/gene_lib/db/reactions.csv")?
        .read_to_string(&mut content)?; //Переносим данные в строку
    let content: Vec<&str> = content.split("\n").collect(); // -> ["EnzymeCode, Substrat, Product"]
    let mut otp: BTreeMap<String, Vec<Vec<String>>> = BTreeMap::new(); // Хотим так -> "EnzymeCode" : ["Substrat", "Product"]
    for stroka in content {
        //Бегаем построчно...
        let temporary: Vec<&str> = stroka.split(",").collect(); // -> ["EnzymeCode", "Substrat", "Product"]
        let mut otp_hm: Vec<Vec<String>> = Vec::new(); //Хранилище для Substrate и Product
        for (idx, elem) in temporary.iter().enumerate() {
            if idx == 1 || idx == 2 {
                //Если субстрат или продукт:
                let tmp_elem: Vec<String> = elem
                    .split(";")
                    .into_iter()
                    .map(|rstr| rstr.to_string())
                    .collect(); // Переводим "C00001;C00002" в ["C00001", "C00002"]
                otp_hm.push(tmp_elem); //Добавялем в списочек
            } else {
                continue;
            }
        }
        otp.insert(temporary[0].to_string(), otp_hm); //Доавляем в словарь key = "EC..." и value = ["Substrate","Product"]
    }
    Ok(otp) //Возвращаем словарь с Энзим - Субстрт/Продукт
}

pub fn create_reactions_tree(compound: &str) -> Result<()> {
    // let path_to_tree = "db/reactions_tree.csv";
    // let mut file_tree = File::create(path_to_tree)?;
    // let mut file_reactions = get_reactions_db().unwrap();



    Ok(())
}


pub fn find_reactions_for_buffer(substrat: Vec<&str>) -> String{    
    let mut output_modules: Vec<String> = Vec::new();
    // let mut output_check = &output_modules.clone();
    
    loop{
        
        let mut tmp_reaction_buffer: Vec<String> = Vec::new(); 
        let mut sub_buff = add_vec_to_set(substrat.clone().iter().map(|elem| {elem.to_string()}).collect());
        let mut black_reaction_list: HashSet<String> = HashSet::new();

        for (reaction_name, substrat_product) in get_reactions_db().unwrap().iter(){
            // println!("ОТЛАДОЧНАЯ ИНФОРМАЦИЯ: {}|{:?}", &reaction_name, &substrat_product);
            let products: HashSet<String>= add_vec_to_set(substrat_product[1].clone().iter().map(|elem| {elem.to_owned()[..elem.len()-3].to_string()}).collect());
            let diff = HashSet::from_iter(sub_buff.difference(&products).map(|elem| {elem.clone()}));
            if diff != sub_buff && !black_reaction_list.contains(reaction_name){
                tmp_reaction_buffer.push(reaction_name.clone());
                black_reaction_list.insert(reaction_name.clone());
                sub_buff = diff;
                continue;
            } else if sub_buff.is_empty() && !tmp_reaction_buffer.is_empty(){
                tmp_reaction_buffer.sort();
                let tmp_reaction_buffer = tmp_reaction_buffer.join(";");
                output_modules.push(tmp_reaction_buffer);
                break;
            } else {
                continue;
            }
        }
        break;
       
    }
    let output = output_modules.join("*");
    return output;
}

fn add_vec_to_set(sub: Vec<String>)-> HashSet<String>{
    let mut vector = sub.clone();
    let mut set: HashSet<String> = HashSet::new();
    vector.sort();
    for elem in vector{
        set.insert(elem);
    }
    return set;
}


