struct DropTableEntry {
    item_id: u16,
    min_amount: u8,
    max_amount: u8,
    chance: f32,
}

struct DropTable {
    entries: Vec<DropTableEntry>,
}

struct DropTableManager {
    tables: Vec<DropTable>,
}