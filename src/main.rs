mod gene_lib;

use std::borrow::BorrowMut;

use diplom::gene_lib::{compartment, main_system::GeneSys, database};

fn main(){
    let mut sys = GeneSys::default();
    sys.create_compartments(vec!["Среда","Клетка","Ядро","Митохондрия","Вакуоль"]);
    sys.create_tree(vec![("Среда", vec!["Клетка"]),("Клетка", vec!["Ядро","Митохондрия"]),("Митохондрия", vec!["Вакуоль"])]);
    sys.add_substance_to_compartment("Клетка", "EC 3.1.3.48");
    sys.add_substance_to_compartment("Клетка", "C01167");
    sys.add_substance_to_compartment("Клетка", "C00001");
    sys.compile();
    sys.print_result();
    
}



