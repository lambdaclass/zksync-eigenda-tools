// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

contract Counter {
    uint private number;

    // Function to set the number
    function setNumber(uint _number) public {
        number = _number;
    }

    // Function to get the number
    function getNumber() public view returns (uint) {
        return number;
    }

    function increment() public {
        number++;
    }
}
