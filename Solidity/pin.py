import requests
import json

# Your Pinata API keys
PINATA_API_KEY = 'YOUR_API_KEY'
PINATA_SECRET_API_KEY = 'YOUR_SECRET_API_KEY'

# IPFS pinning endpoint for Pinata
ENDPOINT = 'https://api.pinata.cloud/pinning/pinFileToIPFS'

# Your Solidity contract file path
CONTRACT_FILE = 'path/to/ArtRegistry.sol'

# Your sample image file path
IMAGE_FILE = 'path/to/sample_image.jpg'

# Prepare the request headers
headers = {
    'pinata_api_key': PINATA_API_KEY,
    'pinata_secret_api_key': PINATA_SECRET_API_KEY,
}

# Upload the Solidity contract file
with open(CONTRACT_FILE, 'rb') as f:
    response = requests.post(ENDPOINT, files={'file': f}, headers=headers)
    contract_hash = json.loads(response.text)['IpfsHash']

# Upload the sample image file
with open(IMAGE_FILE, 'rb') as f:
    response = requests.post(ENDPOINT, files={'file': f}, headers=headers)
    image_hash = json.loads(response.text)['IpfsHash']

# Print the IPFS hashes for the files
print(f'Solidity contract IPFS hash: {contract_hash}')
print(f'Sample image IPFS hash: {image_hash}')
