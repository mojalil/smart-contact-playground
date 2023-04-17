// Import required libraries
use std::collections::BTreeMap;
use sha3::{Digest, Keccak256};

// Define a struct for the NFT
#[derive(Debug)]
struct NFT {
    owner: String,
    properties: BTreeMap<String, String>,
}

// Define a struct for the ERC721A contract
#[derive(Debug)]
struct ERC721A {
    name: String,
    symbol: String,
    total_supply: u32,
    nfts: BTreeMap<String, NFT>,
    balances: BTreeMap<String, u32>,
}

// Implement the ERC721A contract
impl ERC721A {
    // Define a function to create a new NFT
    fn create_nft(&mut self, owner: String, properties: BTreeMap<String, String>) -> String {
        // Generate a unique ID for the NFT
        let id = format!("{:x}", Keccak256::digest(b"nft").as_slice());

        // Add the NFT to the contract
        self.nfts.insert(id.clone(), NFT { owner, properties });

        // Update the owner's balance
        let balance = self.balances.entry(owner.clone()).or_insert(0);
        *balance += 1;

        // Update the total supply
        self.total_supply += 1;

        // Return the ID of the new NFT
        id.clone()
    }

    // Define a function to get the owner of an NFT
    fn get_owner(&self, id: &str) -> Option<&str> {
        match self.nfts.get(id) {
            Some(nft) => Some(nft.owner.as_str()),
            None => None,
        }
    }

    // Define a function to transfer an NFT from one owner to another
    fn transfer(&mut self, id: &str, to: String) -> Result<(), String> {
        // Check if the NFT exists
        match self.nfts.get(id) {
            Some(nft) => {
                // Check if the sender is the owner
                if nft.owner != to {
                    // Update the owner of the NFT
                    let nft = self.nfts.get_mut(id).unwrap();
                    nft.owner = to.clone();

                    // Update the balances
                    let from_balance = self.balances.get_mut(&nft.owner).unwrap();
                    *from_balance -= 1;
                    let to_balance = self.balances.entry(to.clone()).or_insert(0);
                    *to_balance += 1;

                    // Return success
                    Ok(())
                } else {
                    // If the sender is the owner, return an error
                    Err("The sender is already the owner of the NFT".to_string())
                }
            }
            None => {
                // If the NFT doesn't exist, return an error
                Err("The NFT doesn't exist".to_string())
            }
        }
    }
}

// Define a main function to test the contract
fn main() {
    // Create a new ERC721A contract
    let mut erc721a = ERC721A {
        name: "MyNFT".to_string(),
        symbol: "MNFT".to_string(),
        total_supply: 0,
        nfts: BTreeMap::new(),
        balances: BTreeMap::new(),
    };

    // Create a new NFT
    let properties = [("color".to_string(), "red".to_string()), ("size".to_string(), "large".to_string())]
        .iter().cloned().collect();

    let id = erc721a.create_nft("Alice".to_string(), properties);
    println!("Created NFT with ID {}", id);

    // Transfer the NFT to Bob
    match erc721a.transfer(&id, "Bob".to_string()) {
        Ok(_) => println!("Transferred NFT to Bob"),
        Err(e) => println!("Error: {}", e),
    }

    // Get the owner of the NFT
    match erc721a.get_owner(&id) {
        Some(owner) => println!("The owner of the NFT is {}", owner),
        None => println!("The NFT doesn't exist"),
    }
}
