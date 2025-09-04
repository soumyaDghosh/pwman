use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub(crate) service: String,
    pub(crate) password: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Vault {
    accounts: Vec<Account>,
}


impl Vault {
    pub fn load(path: &str) -> Self {
        if let Ok(mut file) = File::open(path) {
            let mut data = String::new();
            file.read_to_string(&mut data).unwrap();
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            Vault::default()
        }
    }

    pub fn save(&self, path: &str) {
        let data = serde_json::to_string_pretty(self).unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(data.as_bytes()).unwrap();
    }

    pub fn add_account(&mut self, service: String, password: String) {
        self.accounts.push(Account { service, password });
    }

    pub fn get_account(&self, service: &str) -> Option<&Account> {
        for acc in &self.accounts {
            if acc.service == service {
                return Some(acc);
            }
        }
        None
    }

    pub fn list_accounts(&self) -> &Vec<Account> {
        &self.accounts
    }
}
