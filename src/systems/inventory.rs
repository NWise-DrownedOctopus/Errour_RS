use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Currency {
    Gold,
    Gems, 
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ItemId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub currencies: HashMap<Currency, u32>, // Currency Type -> count
    pub items: HashMap<ItemId, u32>, // ItemId -> count
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            currencies: HashMap::new(),
            items: HashMap::new(),
        }
    }

    pub fn get_currency(&self, c: Currency) -> u32 {
        // We grab a reference (then dereference) to value at c, but then use unwrap_or to make sure we get 0 if the entry isn't there
        *self.currencies.get(&c).unwrap_or(&0)
    }

    pub fn add_currency(&mut self, c: Currency, amount: u32) {
        *self.currencies.entry(c).or_insert(0) += amount;
    }

    pub fn spend_currency(&mut self, c: Currency, amount: u32) -> Result<(), String> {
        let balance = self.get_currency(c);
        if balance < amount {
            return Err(format!("Not enough {:?}", c));
        }
        self.currencies.insert(c, balance - amount);
        Ok(())
    }

    pub fn add_item(&mut self, id: ItemId, amount: u32) {
        *self.items.entry(id).or_insert(0) += amount;
    }

    pub fn remove_item(&mut self, id: &ItemId, amount:u32) -> Result<(), String> {
        let count = self.items.get(id).cloned().unwrap_or(0);
        if count < amount {
            return Err("Not Enough items".into());
        }
        self.items.insert(id.clone(), count - amount);
        Ok(())
    }

    // For server use later 
    /* 
    pub fn apply_action(&mut self, action: InventoryAction) -> Result<(), String> {
        match action {
            InventoryAction::AddCurrency { currency, amount } => {
                self.add_currency(currency, amount);
                Ok(())
            }
            InventoryAction::SpendCurrency { currency, amount } => {
                self.spend_currency(currency, amount)
            }
            InventoryAction::AddItem { id, amount } => {
                self.add_item(id, amount);
                Ok(())
            }
            InventoryAction::RemoveItem { id, amount } => {
                self.remove_item(&id, amount)
            }
        }
    }
    */
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InventoryAction {
    AddCurrency { currency: Currency, amount: u32 },
    SpendCurrency { currency: Currency, amount: u32 },
    AddItem { id: ItemId, amount: u32 },
    RemoveItem { id: ItemId, amount: u32 },
}