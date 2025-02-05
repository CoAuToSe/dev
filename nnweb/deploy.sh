#!/bin/bash

echo "Déploiement du projet..."

# Rendre les scripts exécutables
chmod +x install_prerequisites.sh
chmod +x create_project_files.sh
chmod +x build_and_run.sh

# Exécuter les scripts
./install_prerequisites.sh
./create_project_files.sh
./build_and_run.sh
