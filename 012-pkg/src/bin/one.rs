use pkg::inventory;
use pkg::orders;


fn main() {
    println!("inventory manager: {}", inventory::MANAGER);
    println!("orders manager: {}", orders::MANAGER);

    let mut i1 = inventory::Inventory::new(0, 0, inventory::InventoryType::FloorSpace);
    let mut i2: inventory::Inventory = inventory::Inventory::new(0, 0, inventory::InventoryType::ShelfSpace);


    i1.add_floor_space(100);
    i1.add_shelf_space(10);

    i2.add_floor_space(100);
    i2.add_shelf_space(10);


    let p1 = inventory::products::Products::new(
        String::from("Product 1"),
        10.0,
        100,
    );

    println!("p1: {:?}", p1);
    
}
