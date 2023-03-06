import streamlit as st
from web3 import Web3
from dotenv import load_dotenv
import os

# Load environment variables
load_dotenv()
RPC_ENDPOINT = os.getenv('RPC_ENDPOINT')
CONTRACT_ADDRESS = os.getenv('CONTRACT_ADDRESS')
PRIVATE_KEY = os.getenv('PRIVATE_KEY')

# Connect to Ethereum network
web3 = Web3(Web3.HTTPProvider(RPC_ENDPOINT))
account = web3.eth.account.from_key(PRIVATE_KEY)

# Load the contract ABI
with open('ArtRegistry.json') as f:
    contract_abi = json.load(f)['abi']

# Load the contract instance
contract = web3.eth.contract(address=CONTRACT_ADDRESS, abi=contract_abi)

# Streamlit app
st.title('Art Registry')

# Register new artwork
st.header('Register new artwork')
name = st.text_input('Name')
artist = st.text_input('Artist')
appraisal_value = st.number_input('Appraisal value', step=1)
token_uri = st.text_input('Token URI')
if st.button('Register artwork'):
    tx_hash = contract.functions.registerArtwork(account.address, name, artist, appraisal_value, token_uri).transact({'from': account.address})
    st.success(f'Transaction sent: {tx_hash.hex()}')

# Update artwork appraisal value
st.header('Update artwork appraisal value')
token_id = st.number_input('Token ID', step=1)
new_appraisal_value = st.number_input('New appraisal value', step=1)
report_uri = st.text_input('Report URI')
if st.button('Update appraisal value'):
    tx_hash = contract.functions.updateAppraisal(token_id, new_appraisal_value, report_uri).transact({'from': account.address})
    st.success(f'Transaction sent: {tx_hash.hex()}')
