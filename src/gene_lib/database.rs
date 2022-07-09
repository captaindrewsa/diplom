// use mysql::*;
// use mysql::prelude::*;
// use mysql::Opts;
// use mysql::serde::de::Error;
use std::{
    collections::{HashMap, HashSet, BTreeMap},
    fs::{self, File},
    io::{Read, Result}, vec, process::Output,
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

pub fn create_reactions_tree_light(compounds: Vec<&str>) -> Option<Vec<String>> {
    let mut output_reactions: Vec<String> = Vec::new();
    let mut reaction_buff: String = String::new();
    let mut comp_buff: Vec<String> = add_three_symbols(compounds);
    
    loop {
        if let Some(reactions) = find_reactions_for_buffer(comp_buff.clone()){
            output_reactions.push(reactions.clone());
            reaction_buff = reactions.clone();
            // println!("{}",reactions.clone());
        } else {
            break;
        }
        if let Some(substrats) = get_substrat_from_reactions(reaction_buff.clone()){
            comp_buff = substrats.clone().iter().map(|s| s.to_string()).collect();
        } else { 
            break;
        }

    }
    if !output_reactions.is_empty(){
        Some(vec![output_reactions.join("<-"), comp_buff.join(";")])
    } else {
        None
    }
    
}


fn find_reactions_for_buffer(substrat: Vec<String>) -> Option<String>{    
    let mut output_modules: Vec<String> = Vec::new();
    loop{
        
        let mut tmp_reaction_buffer: Vec<String> = Vec::new(); 
        let mut sub_buff = add_vec_to_set(substrat.clone().iter().map(|elem| {elem[0..elem.len()-3].to_string()}).collect());
        let mut black_reaction_list: HashSet<String> = HashSet::new();

        for (reaction_name, substrat_product) in get_reactions_db().unwrap().iter(){
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
    
    if !output_modules.is_empty(){
        let output = output_modules.join(";");
        Some(output)
    } else {
        None
    }
}

fn get_substrat_from_reactions(module: String)-> Option<Vec<String>>{
    let reacions: Vec<String> = module.split(";").map(|elem| {elem.to_string()}).collect();
    let mut out: Vec<String> = Vec::new();
    for reaction in reacions.iter(){
        if let Some(sub_prod) = get_reactions_db().unwrap().get(reaction){
            let mut tmp = sub_prod.clone();
            out.append(&mut tmp[0]);
        } else {
            continue;
        }
    }
    if !out.is_empty(){
        Some(out)
    } else {
        None
    }

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

fn add_three_symbols(stroka: Vec<&str>)-> Vec<String>{
    let tmp: Vec<String> = stroka.iter().map(|s| {s.to_string()}).collect();
    let mut output_stroka: Vec<String>= Vec::new();
    for elem in tmp{
        let mut elem_tmp = elem.clone();
        elem_tmp.push_str("000");
        output_stroka.push(elem_tmp);
    }
    return output_stroka;
}


