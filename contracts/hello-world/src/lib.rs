#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Symbol, String, Address, symbol_short};

// Structure to track charity information
#[contracttype]
#[derive(Clone)]
pub struct Charity {
    pub charity_id: u64,
    pub name: String,
    pub wallet: Address,
    pub total_received: i128,
    pub donation_count: u64,
    pub is_active: bool,
}

// Structure to track individual donations
#[contracttype]
#[derive(Clone)]
pub struct Donation {
    pub donation_id: u64,
    pub charity_id: u64,
    pub donor: Address,
    pub amount: i128,
    pub timestamp: u64,
}

// Symbol for tracking total charity count
const CHARITY_COUNT: Symbol = symbol_short!("CH_COUNT");

// Symbol for tracking total donation count
const DONATION_COUNT: Symbol = symbol_short!("DN_COUNT");

// Mapping for storing charities
#[contracttype]
pub enum CharityBook {
    Charity(u64)
}

// Mapping for storing donations
#[contracttype]
pub enum DonationBook {
    Donation(u64)
}

#[contract]
pub struct DonationPlatform;

#[contractimpl]
impl DonationPlatform {
    
    // Function 1: Register a new charity on the platform
    pub fn register_charity(env: Env, name: String, wallet: Address) -> u64 {
        let mut charity_count: u64 = env.storage().instance().get(&CHARITY_COUNT).unwrap_or(0);
        charity_count += 1;
        
        // Create new charity record
        let charity = Charity {
            charity_id: charity_count,
            name: name.clone(),
            wallet: wallet,
            total_received: 0,
            donation_count: 0,
            is_active: true,
        };
        
        // Store the charity
        env.storage().instance().set(&CharityBook::Charity(charity_count), &charity);
        env.storage().instance().set(&CHARITY_COUNT, &charity_count);
        
        env.storage().instance().extend_ttl(5000, 5000);
        
        log!(&env, "Charity Registered with ID: {}", charity_count);
        charity_count
    }
    
    // Function 2: Make a donation to a specific charity
    pub fn donate(env: Env, charity_id: u64, donor: Address, amount: i128) -> u64 {
        // Validate amount
        if amount <= 0 {
            log!(&env, "Donation amount must be positive");
            panic!("Invalid donation amount");
        }
        
        // Get charity details
        let mut charity = Self::view_charity(env.clone(), charity_id);
        
        if !charity.is_active {
            log!(&env, "Charity is not active");
            panic!("Charity not active");
        }
        
        if charity.charity_id == 0 {
            log!(&env, "Charity not found");
            panic!("Charity does not exist");
        }
        
        // Get donation count
        let mut donation_count: u64 = env.storage().instance().get(&DONATION_COUNT).unwrap_or(0);
        donation_count += 1;
        
        // Record the donation
        let donation = Donation {
            donation_id: donation_count,
            charity_id: charity_id,
            donor: donor,
            amount: amount,
            timestamp: env.ledger().timestamp(),
        };
        
        // Update charity stats
        charity.total_received += amount;
        charity.donation_count += 1;
        
        // Store updated data
        env.storage().instance().set(&DonationBook::Donation(donation_count), &donation);
        env.storage().instance().set(&CharityBook::Charity(charity_id), &charity);
        env.storage().instance().set(&DONATION_COUNT, &donation_count);
        
        env.storage().instance().extend_ttl(5000, 5000);
        
        log!(&env, "Donation of {} XLM recorded for Charity ID: {}", amount, charity_id);
        donation_count
    }
    
    // Function 3: View charity details by ID
    pub fn view_charity(env: Env, charity_id: u64) -> Charity {
        let key = CharityBook::Charity(charity_id);
        
        env.storage().instance().get(&key).unwrap_or(Charity {
            charity_id: 0,
            name: String::from_str(&env, "Not_Found"),
            wallet: Address::from_string(&String::from_str(&env, "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF")),
            total_received: 0,
            donation_count: 0,
            is_active: false,
        })
    }
    
    // Function 4: View donation details by ID
    pub fn view_donation(env: Env, donation_id: u64) -> Donation {
        let key = DonationBook::Donation(donation_id);
        
        env.storage().instance().get(&key).unwrap_or(Donation {
            donation_id: 0,
            charity_id: 0,
            donor: Address::from_string(&String::from_str(&env, "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF")),
            amount: 0,
            timestamp: 0,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_register_and_donate() {
        let env = Env::default();
        let contract_id = env.register_contract(None, DonationPlatform);
        let client = DonationPlatformClient::new(&env, &contract_id);
        
        // Test charity registration
        let charity_name = String::from_str(&env, "Test Charity");
        let wallet = Address::from_string(&String::from_str(&env, "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF"));
        
        let charity_id = client.register_charity(&charity_name, &wallet);
        assert_eq!(charity_id, 1);
    }
}