// use mysql::*;
// use mysql::prelude::*;
// use mysql::Opts;
// use mysql::serde::de::Error;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{Read, Result},
};

pub fn get_reactions_db() -> Result<HashMap<String, Vec<Vec<String>>>> {
    let mut content = String::new(); //Создаем Стринг для данных из файла
    File::open("/home/captaindrewsa/Programming/Rust/diplom/src/gene_lib/db/reactions.csv")?
        .read_to_string(&mut content)?; //Переносим данные в строку
    let content: Vec<&str> = content.split("\n").collect(); // -> ["EnzymeCode, Substrat, Product"]
    let mut otp: HashMap<String, Vec<Vec<String>>> = HashMap::new(); // Хотим так -> "EnzymeCode" : ["Substrat", "Product"]
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
    let path_to_tree = "db/reactions_tree.csv";
    let mut file_tree = File::create(path_to_tree)?;
    let mut file_reactions = get_reactions_db().unwrap();
    /*
    */

    Ok(())
}

/*
fn find_reactions_for_buffer(substrat: Vec<String>) -> Vec<String>{
    
    let mut output_modules: Vec<Vec<String>> = Vec::new();

    loop{
        let mut buffer = HashSet::from(substrat);
        let mut tmp_reactions_module: Vec<String> = Vec::new();

        for reaction in reactions.db{
            let reactoion_buffer = HashSet::from(reaction.product);
            if buffer.difference(&reaction_buffer) != buffer{
                buffer = buffer.difference(&reaction_buffer);
                tmp_reactions_module.push(reaction.name);
                continue;
            } else if buffer.is_empty() && !tmp_reaction_module.is_empty(){
                output_modules.push(tmp_reaction_module);
                /*КАК_ТО СДЕЛАТЬ ЧЕРНЫЙ ЛИСТ РЕАКЦИЙ. ПРОВЕРКА НА ИЗМЕНЕНИЯ С ПРЕДЫДУЩИМ ЦИКЛОМ */
                break;
            }
        }

    }
}
*/

