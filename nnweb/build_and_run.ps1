# Build and Run the Project
Write-Host "Compilation du projet..."

# Aller dans le répertoire du projet
Set-Location neural_network_gpu

# Compiler le projet en WebAssembly
wasm-pack build --target web

# Copier les fichiers générés dans le dossier www
Copy-Item -Path pkg -Destination www\pkg -Recurse -Force

Write-Host "Compilation terminée."

Write-Host "Lancement du serveur..."

# Aller dans le dossier www
Set-Location www

# Lancer le serveur HTTP sur le port 8080
http-server -c-1 -p 8080
