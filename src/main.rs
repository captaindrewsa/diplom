mod gene_lib;
use diplom::gene_lib::{compartment, database::{self, create_reactions_tree}, main_system::GeneSys};

fn main() {
    // let mut sys = GeneSys::default();
    // sys.create_compartments(vec!["Клетка","Среда"]);
    // sys.create_tree(vec![
    //     ("Среда", vec!["Клетка"])
    // ]);
    // sys.add_substance_to_compartment("Клетка", "EC 3.1.3.48");
    // sys.add_substance_to_compartment("Клетка", "C01167");
    // sys.add_substance_to_compartment("Клетка", "C00001");
    // sys.compile();
    // sys.print_result();
    // println!("{:?}",get_substrat_from_reactions("EC 3.1.3.48".to_string()));
    println!("{:?}", create_reactions_tree(vec!["C1"]).unwrap());
}