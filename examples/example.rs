use map_macros::{map, btree_map, set, btree_set};

fn main() {
    let mut mm = map!{};
    println!("{:?}", mm);
    mm.insert("hello", "world");
    
    let mm = btree_map!{"hello"=>"world", "yes"=>"ok"};
    println!("{:?}", mm);
    
    let mm = set!{"hello", "yes"};
    println!("{:?}", mm);
    
    let mm = btree_set!{"hello", "yes"};
    println!("{:?}", mm);
}
