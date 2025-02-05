#!/bin/bash

echo "Installation des prérequis..."

# Installer la cible WebAssembly pour Rust
rustup target add wasm32-unknown-unknown

# Installer wasm-bindgen-cli et wasm-pack
cargo install wasm-bindgen-cli
cargo install wasm-pack

# Vérifier que npm est installé pour installer un serveur HTTP simple
if ! command -v npm &> /dev/null
then
    echo "npm n'est pas installé. Veuillez installer Node.js et npm."
    exit
fi

# Installer http-server pour servir les fichiers statiques
npm install -g http-server

echo "Installation terminée."
