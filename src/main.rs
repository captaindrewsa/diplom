mod gene_lib;
use diplom::gene_lib::{main_system::GeneSys};

fn main() {
    let mut sys = GeneSys::default();
    sys.create_compartments(vec!["Клетка", "Среда"]);
    sys.create_tree(vec![("Среда", vec!["Клетка"])]);
    sys.add_substance_to_compartment("Клетка", "EC 3.1.3.48");
    sys.add_substance_to_compartment("Клетка", "C01167");
    sys.add_substance_to_compartment("Клетка", "C00001");
    sys.compile();
    sys.print_result();
}
