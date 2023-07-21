#!/bin/bash

usage() {
    echo "Usage: $0 -s <source> -n <network>"
    echo "Available networks: mainnet, testnet, futurenet, localnet"
    exit 1
}

while getopts ":n:s:" opt; do
    case $opt in
        n)
            network=$OPTARG
            ;;
        s)
            source=$OPTARG
            ;;
        \?)
            echo "Invalid option: -$OPTARG"
            usage
            ;;
    esac
done

shift $((OPTIND - 1))

if [[ -z $network ]]; then
    usage
fi

case $network in
    mainnet)
        echo "Mainnet Not Supported Yet"
        exit 1
        ;;
    testnet)
        echo "Testnet Not Supported Yet"
        exit 1
        ;;
    futurenet)
        rpc_url="https://rpc-futurenet.stellar.org:443"
        network_passphrase="Test SDF Future Network ; October 2022"
        ;;
    localnet)
        rpc_url="http://localhost:8000/soroban/rpc"
        network_passphrase="Standalone Network ; February 2017"
        ;;
    *)
        echo "Invalid network: $network"
        usage
        ;;
esac

soroban contract deploy --wasm dist/std_reference.optimized.wasm --source "$source" --rpc-url "$rpc_url" --network-passphrase "$network_passphrase"
