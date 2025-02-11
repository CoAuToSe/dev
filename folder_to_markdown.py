#!/usr/bin/env python3
import os

def extraire_dossiers_vers_markdown():
    # Nom du fichier de sortie (sera exclu du scan)
    nom_fichier_sortie = "extracted_files.md"
    
    # Ouvrir le fichier de sortie en écriture
    with open(nom_fichier_sortie, "w", encoding="utf-8") as f_sortie:
        # Parcours récursif du répertoire courant
        for racine, dossiers, fichiers in os.walk("."):
            for nom_fichier in fichiers:
                # Ignorer le fichier de sortie
                if nom_fichier == nom_fichier_sortie:
                    continue

                # Construction du chemin complet et du chemin relatif
                chemin_complet = os.path.join(racine, nom_fichier)
                chemin_relatif = os.path.relpath(chemin_complet, ".")

                # Écrire le début du bloc de code avec le nom du fichier
                f_sortie.write(f"```{chemin_relatif}\n")
                
                try:
                    # Lire le contenu du fichier
                    with open(chemin_complet, "r", encoding="utf-8") as f_entree:
                        contenu = f_entree.read()
                    f_sortie.write(contenu)
                except Exception as e:
                    # En cas d'erreur (par exemple, fichier binaire ou problème d'encodage), noter l'erreur
                    f_sortie.write(f"Erreur lors de la lecture du fichier : {e}")
                
                # Fin du bloc de code Markdown
                f_sortie.write("\n```\n\n")

if __name__ == "__main__":
    extraire_dossiers_vers_markdown()
