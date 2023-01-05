# Kujira Denom Wrapper
Proxy Contract that creates a Denom on instantiation and sends messages related to this Denom to the Kujira Denom module.

This contract aims to facilitate the interaction of CW3 Multisigs and other contracts with the Kujira Denom module.

It also contains an option to remove the admin from the contract, removing the ability to mint more tokens.


## To compile
`sh optimize.sh`

## To run integration "tests"
`sh integration.sh`