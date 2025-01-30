pub mod products;
pub const MANAGER: &str = "John Doe";


#[derive(Clone)]
pub enum InventoryType {
    FloorSpace,
    ShelfSpace,
}

pub struct Inventory {
    pub floor_space: u32,
    pub shelf_space: u32,
    pub of_type: InventoryType,
}

impl Inventory {
    pub fn new(floor_space: u32, shelf_space: u32, of_type: InventoryType) -> Inventory {
        Inventory {
            floor_space,
            shelf_space,
            of_type,
        }
    }

    pub fn add_shelf_space(&mut self, shelf_space: u32) {
        self.shelf_space += shelf_space;
    }

    pub fn add_floor_space(&mut self, floor_space: u32) {
        self.floor_space += floor_space;
    }

    pub fn remove_shelf_space(&mut self, shelf_space: u32) {
        self.shelf_space -= shelf_space;
    }

    pub fn remove_floor_space(&mut self, floor_space: u32) {
        self.floor_space -= floor_space;
    }

    pub fn get_type(&self) -> InventoryType {
        self.of_type.clone()
    }
}