#!/bin/bash

echo "Compilation du projet..."

# Aller dans le répertoire du projet
cd neural_network_gpu

# Compiler le projet en WebAssembly
wasm-pack build --target web

# Copier les fichiers générés dans le dossier www
cp -r pkg www/

echo "Compilation terminée."

echo "Lancement du serveur..."

# Aller dans le dossier www
cd www

# Lancer un serveur HTTP sur le port 8080
http-server -c-1 -p 8080

# L'option -c-1 désactive la mise en cache
