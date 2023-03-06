// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";
import "@openzeppelin/contracts/utils/math/SafeMath.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";

contract ArtRegistry is ERC721URIStorage, ReentrancyGuard {
    using SafeMath for uint256;
    
    struct Artwork {
        string name;
        string artist;
        uint256 appraisalValue;
    }
    
    Artwork[] private artCollection;
    
    event Appraisal(uint256 indexed tokenId, uint256 appraisalValue, string reportURI);
    
    constructor() ERC721("ArtRegistryToken", "ART") {}
    
    function registerArtwork(
        address owner,
        string memory name,
        string memory artist,
        uint256 initialAppraisalValue,
        string memory tokenURI
    ) public nonReentrant returns (uint256) {
        require(bytes(name).length > 0, "Name should not be empty");
        require(bytes(artist).length > 0, "Artist should not be empty");
        
        uint256 tokenId = artCollection.length;
        artCollection.push(Artwork(name, artist, initialAppraisalValue));
        
        _safeMint(owner, tokenId);
        _setTokenURI(tokenId, tokenURI);
        
        return tokenId;
    }
    
    function updateAppraisal(
        uint256 tokenId,
        uint256 newAppraisalValue,
        string memory reportURI
    ) public nonReentrant {
        require(_exists(tokenId), "Token does not exist");
        require(ownerOf(tokenId) == msg.sender, "Only the owner can update the appraisal value");
        
        artCollection[tokenId].appraisalValue = newAppraisalValue;
        
        emit Appraisal(tokenId, newAppraisalValue, reportURI);
    }
    
    function getArtwork(uint256 tokenId) public view returns (
        string memory name,
        string memory artist,
        uint256 appraisalValue
    ) {
        require(_exists(tokenId), "Token does not exist");
        
        Artwork storage artwork = artCollection[tokenId];
        name = artwork.name;
        artist = artwork.artist;
        appraisalValue = artwork.appraisalValue;
    }
    
    function totalArtwork() public view returns (uint256) {
        return artCollection.length;
    }
    
    //function withdraw() public nonReentrant {
    //    require(msg.sender == ownerOf(), "Only the contract owner can withdraw funds");
    //    payable(ownerOf()).transfer(address(this).balance);
    //}
}
