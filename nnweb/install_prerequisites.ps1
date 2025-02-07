# Install Prerequisites
Write-Host "Installation des prérequis..."

# Installer la cible WebAssembly pour Rust
rustup target add wasm32-unknown-unknown

# Installer wasm-pack
cargo install wasm-pack

# Vérifier si npm est installé
if (-Not (Get-Command npm -ErrorAction SilentlyContinue)) {
    Write-Host "npm n'est pas installé. Veuillez installer Node.js et npm depuis https://nodejs.org/."
    exit
}

# Installer http-server
npm install -g http-server

Write-Host "Installation terminée."
