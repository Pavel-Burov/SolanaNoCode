from solana.rpc.api import Client
from solana.rpc.types import TxOpts
import json

def interact_with_contract(program_id, user_keypair, initial_value):
    client = Client("https://api.devnet.solana.com")


    transaction = {
        "keys": [
            {"pubkey": user_keypair.public_key, "is_signer": True, "is_writable": True}
        ],
        "programId": program_id,
        "data": b''
    }

    response = client.send_transaction(transaction, user_keypair, opts=TxOpts(skip_preflight=True))
    print("Transaction response:", response)

if __name__ == "__main__":
    program_id = "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS"
    user_keypair = ""
    initial_value = 100

    interact_with_contract(program_id, user_keypair, initial_value)
