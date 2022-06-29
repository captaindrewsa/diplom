// use mysql::*;
// use mysql::prelude::*;
// use mysql::Opts;
// use mysql::serde::de::Error;
use std::{fs::{self, File}, io::{Read, Result}, collections::HashMap};

pub fn get_reactions_db()-> Result<HashMap<String, Vec<Vec<String>>>>{
    let mut content = String::new();
    File::open("/home/captaindrewsa/Programming/Rust/diplom2/src/gene_lib/db/reactions.csv")?.read_to_string(&mut content)?;
    let content : Vec<&str> = content.split("\n").collect();    // ["EnzymeCode, Name, Substrat, Product"]
    let mut otp: HashMap<String, Vec<Vec<String>>> = HashMap::new(); // todo! "EnzymeCode" : ["Name", "Substrat", "Product"]
    for stroka in content{
        let temporary: Vec<&str> = stroka.split(",").collect(); // ["EnzymeCode", "Name", "Substrat", "Product"]
        let mut otp_hm: Vec<Vec<String>> = Vec::new();
        for (idx, elem) in temporary.iter().enumerate(){
            if idx ==1 {
                otp_hm.push(vec![elem.to_string()]);
            } else if idx == 2 || idx == 3 {
                let tmp_elem :Vec<String> = elem.split(";")
                    .into_iter()
                    .map(|rstr| rstr.to_string())
                    .collect();
                otp_hm.push(tmp_elem);
            } else {
                continue;
            }
        }
        otp.insert(temporary[0].to_string(), otp_hm);
    }
    Ok(otp)
}





// #[derive(Debug)]
// struct Enzyme{
//     enzyme_code: String,
//     Name: String,
//     Subsrat: String,
//     Product: String
// }


// pub fn test_fn() -> Result<mysql::PooledConn>{
//     let url = "mysql://root:Asavoz76Vlas5712Drewsa_Bad@localhost:3306/enzyme";
//     // let mut builder = OptsBuilder::new();
//     // builder.ip_or_hostname(Some("root@localhost"))
//     //     .db_name(Some("enzyme"))
//     //     .ssl_opts(Some(SslOpts::default()));

//     let pool = Pool::new(Opts::from_url(url)?).expect("Не приконектилось");
//     let mut conn = pool.get_conn()?;
    
//     conn.query_drop(
//         "USE enzyme;"
//     )?;

//     conn.query_drop(
//         r"CREATE TABLE IF NOT EXIST enzymes (
//             EnzymeCode CHAR(50) not null,
//             Name CHAR(255),
//             Substrate CHAR(500),
//             Product CHAR(500)
//         )"
//     )?;

//     let enzyme_list = vec![
//         Enzyme{ enzyme_code: "EC 3.1.3.48".to_string(), Name: "PTPase".to_string(), Subsrat: "C01167_IN;C00001_IN".to_string(), Product: "C00585_IN;C00009_IN".to_string()}
//     ];

//     conn.exec_batch(
//         r"INSERT INTO enzymes (Enzyme_code, Name, Substrate, Product) VALUES(:enzyme_code, :name, :sub, :prod)"
//         , enzyme_list
//             .iter()
//             .map(|e| params! {
//                 "enzyme_code"=> e.enzyme_code.clone(),
//                 "name" => e.Name.clone(),
//                 "sub"=> e.Subsrat.clone(),
//                 "prod" => e.Product.clone()
//             })
//         )?;
//     let selected_enzyme = conn
//         .query_map(
//             "SELECT * FROM enzymes",
//             |(code, name, sub, prod)|{
//                 Enzyme{ enzyme_code: code, Name: name, Subsrat: sub, Product: prod}
//             },
//             )?;
//     println!("{:?}", selected_enzyme[0]);
//     Ok(conn)
// }