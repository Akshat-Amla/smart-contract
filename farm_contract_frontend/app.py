from flask import Flask, render_template, request, redirect
from solathon import Client, Transaction, PublicKey
from solathon.utils import to_lamports
import json
import threading
import time

SOLANA_URL = "https://api.devnet.solana.com"
client = Client(SOLANA_URL)

# Replace with your actual deployed program ID
PROGRAM_ID = PublicKey("74XkHFVkWcqUA8nVXA25ioqSNR2yJFLwGnEppKvLJ4Ny")

app = Flask(__name__)

@app.route('/')
def home():
    return render_template('form.html')

def monitor_contract(duration, farmer_pubkey, buyer_pubkey):
    """Function to monitor contract duration and notify when completed."""
    time.sleep(duration * 60)  # Convert minutes to seconds
    # Notify that the contract is completed
    print(f"Contract between {farmer_pubkey} and {buyer_pubkey} has been successfully completed!")

@app.route('/submit', methods=['POST'])
def submit_contract():
    """Handles contract submission and starts monitoring."""
    farmer_pubkey = request.form['farmer']
    buyer_pubkey = request.form['buyer']
    crop_type = request.form['cropType']
    agreed_price = int(request.form['agreedPrice'])
    quality_score = int(request.form['qualityScore'])
    duration = int(request.form['duration'])
    contract_type = request.form['contractType']

    # Create a Solana transaction here
    transaction = Transaction()

    # Create an instruction to transfer funds as part of the contract (if applicable)
    try:
        # Convert agreed_price to lamports
        amount_lamports = to_lamports(agreed_price)
        
        # Create transfer instruction
        transfer_instruction = Transaction.transfer(
            from_public_key=PublicKey(farmer_pubkey),
            to_public_key=PublicKey(buyer_pubkey),
            amount_of_sol=amount_lamports
        )
        transaction.add(transfer_instruction)

        # Send the transaction
        response = client.send_transaction(transaction, PublicKey(farmer_pubkey))
        
        # Check response for success
        if not response.success:
            print(f"Transaction failed: {response.error}")
            return redirect('/')
        
        print(f"Transaction successful: {response.transaction_hash}")
        
    except Exception as e:
        print(f"Transaction creation or sending failed: {str(e)}")
        return redirect('/')

    # Start a thread to monitor the contract duration
    threading.Thread(target=monitor_contract, args=(duration, farmer_pubkey, buyer_pubkey)).start()

    print(f"Contract created successfully between {farmer_pubkey} and {buyer_pubkey} for {duration} minutes!")
    return redirect('/')  # Redirect to home

if __name__ == '__main__':
    app.run(debug=True)