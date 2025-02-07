# Deploy the Project
Write-Host "Déploiement du projet..."

# Exécuter les scripts dans l'ordre
.\install_prerequisites.ps1
.\create_project_files.ps1
.\..\build_and_run.ps1
