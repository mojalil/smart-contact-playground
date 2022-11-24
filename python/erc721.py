# Import required libraries
from typing import Dict, Any, Tuple
from collections import OrderedDict
from hashlib import sha256

# Define a class for the NFT
class NFT:
    def __init__(self, owner: str, properties: Dict[str, Any]):
        self.owner = owner
        self.properties = properties

# Define a class for the ERC721A contract
class ERC721A:
    def __init__(self, name: str, symbol: str):
        self.name = name
        self.symbol = symbol
        self.total_supply = 0
        self.nfts = OrderedDict()
        self.balances = {}

    # Define a function to create a new NFT
    def create_nft(self, owner: str, properties: Dict[str, Any]) -> str:
        # Generate a unique ID for the NFT
        id = sha256(b"nft").hexdigest()

        # Add the NFT to the contract
        self.nfts[id] = NFT(owner, properties)

        # Update the owner's balance
        if owner in self.balances:
            self.balances[owner] += 1
        else:
            self.balances[owner] = 1

        # Update the total supply
        self.total_supply += 1

        # Return the ID of the new NFT
        return id

    # Define a function to get the owner of an NFT
    def get_owner(self, id: str) -> str:
        try:
            return self.nfts[id].owner
        except KeyError:
            return ""

    # Define a function to transfer an NFT from one owner to another
    def transfer(self, id: str, to: str) -> Tuple[bool, str]:
        # Check if the NFT exists
        if id not in self.nfts:
            return False, "The NFT doesn't exist"

        # Check if the sender is the owner
        if self.nfts[id].owner == to:
            return False, "The sender is already the owner of the NFT"

        # Update the owner of the NFT
        self.nfts[id].owner = to

        # Update the balances
        if self.nfts[id].owner in self.balances:
            self.balances[self.nfts[id].owner] += 1
        else:
            self.balances[self.nfts[id].owner] = 1

        if self.nfts[id].owner in self.balances:
            self.balances[self.nfts[id].owner] -= 1
        else:
            self.balances[self.nfts[id].owner] = -1

        # Return success
        return True, ""

# Define a main function to test the contract
def main():
    # Create a new ERC721A contract
    erc721a = ERC721A("MyNFT", "MNFT")

    # Create a new NFT
    properties = {"color": "red", "size": "large"}

    id = erc721a.create_nft("Alice", properties)
    print("Created NFT with ID", id)

    # Transfer the NFT to Bob
    success, error = erc721a.transfer(id, "Bob")
    if success:
        print("Transferred NFT to Bob")
    else:
        print("Error:", error)

    # Get the owner of the NFT
    owner = erc721a.get_owner(id)
    if owner:
        print("The owner of the NFT is", owner)
    else:
        print("The NFT doesn't exist")

if __name__ == "__main__":
    main()
