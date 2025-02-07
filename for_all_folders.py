import os
import subprocess
import argparse

already_treated = []

def executer_commande_dans_dossiers(commande, dossier_courant, dossiers_exclus, error_code_stop = 0):
    # Récupère le chemin du répertoire courant
    # Parcourt chaque élément dans le répertoire courant
    print(os.listdir(dossier_courant))
    succes = False
    for element in os.listdir(dossier_courant):
        # Si le dossier fait partie de la liste des dossiers à exclure, on le saute.
        # print(dossier_courant, element,dossiers_exclus, element in dossiers_exclus)
        if element in dossiers_exclus:
            print(f"Le dossier '{element}' est exclu.")
            # continue
        else:
            chemin_element = os.path.join(dossier_courant, element)
            # Vérifie si l'élément est un dossier
            if os.path.isdir(chemin_element):
                # pretty print
                pretty_command = ' '.join(commande)
                print(f"\nExécution de la commande `{pretty_command}` dans : {chemin_element}")
                
                # check if déjà traité
                if chemin_element in already_treated:
                    raise BaseException(f"sus {dossier_courant} : {already_treated}")
                # exécution commande
                try:
                    # Exécute la commande dans le dossier spécifié
                    resultat = subprocess.run(
                        commande,
                        cwd=chemin_element,
                        # check=True,
                        # capture_output=True,
                        # text=True
                        shell=True
                    )
                    # print(resultat)
                    already_treated.append(chemin_element)
                    if resultat.stdout != None:
                        print("Sortie :")
                        print(resultat.stdout)
                    if resultat.returncode != error_code_stop:
                        # print("lol")
                        succes = False
                except BaseException as e:
                    raise BaseException(e)
                except subprocess.CalledProcessError as e:
                    print(f"Une erreur est survenue dans {chemin_element} : {e}")
                    print("Message d'erreur :", e.stderr)
                    return False
                except FileNotFoundError as e:
                    print(f"Commande non trouvée dans {chemin_element} : {e}")
                    return False
                except Exception as e:
                    print(f"Erreur inattendue dans {chemin_element} : {e}")
                    return False
                # try:
                # si les sous éléments de dossiers sont des dossier : récursive
                print(os.listdir(chemin_element))
                for sub_element in os.listdir(chemin_element):
                    print(f"going for sub elements {sub_element}")
                    chemin_sub_element = os.path.join(chemin_element, sub_element)
                    print(chemin_element, sub_element, dossiers_exclus, sub_element in dossiers_exclus)
                    if sub_element in dossiers_exclus:
                        print(f"Le dossier '{sub_element}' est exclu.")
                        succes = True
                    else:
                        if os.path.isdir(chemin_sub_element):
                            if chemin_sub_element in already_treated:
                                raise BaseException(f"sus {dossier_courant} : {already_treated}")
                            # Pour chaque sous-dossier, on essaie d'exécuter la commande.
                            if executer_commande_dans_dossiers(commande, chemin_sub_element, dossiers_exclus):
                                succes = True
                        else:
                            print(f"L'élément {chemin_sub_element} est un fichier.")
                            succes = True
                if not succes:
                    print(f"La commande n'a pas pu être exécutée correctement dans aucun sous-dossier de {dossier_courant}.")
                # except BaseException as e:
                #     raise BaseException(e)
                # except Exception as sous_erreur:
                #     print(f"Impossible de parcourir les sous-dossiers de {dossier_courant} : {sous_erreur}")
    return succes

if __name__ == "__main__":
    # Configuration de l'analyseur d'arguments
    parser = argparse.ArgumentParser(
        description="Exécute une commande (avec ses arguments) dans tous les dossiers du répertoire courant, en excluant certains dossiers."
    )
    parser.add_argument(
        "commande",
        nargs="+",
        help="La commande à exécuter, avec ses arguments (exemple : ls -la)"
    )
    parser.add_argument(
        "--exclure", "-ex",
        nargs="+",
        default=[],
        help="Liste des noms de dossiers à exclure (exemple : --exclure .vscode node_modules)"
    )
    parser.add_argument(
        "--error_code", "-e",
        nargs="+",
        default=[],
        help="Liste des noms de dossiers à exclure (exemple : --error_code 101)"
    )
    
    args = parser.parse_args()
    
    # La commande est récupérée sous forme de liste, et on récupère la liste des dossiers à exclure
    print(executer_commande_dans_dossiers(args.commande, os.getcwd(), args.exclure))



# import os
# import subprocess
# import argparse

# def executer_commande_recursif(commande, dossier):
#     """
#     Essaie d'exécuter la commande dans le dossier indiqué.
#     - Si l'exécution aboutit (aucune CalledProcessError), affiche la sortie et retourne True.
#     - Si l'exécution échoue par CalledProcessError, parcourt les sous-dossiers récursivement.
#     - Pour toute autre exception, affiche l'erreur et retourne False.
#     """
#     # Reconstituer la commande en chaîne pour shell=True (utile pour les commandes internes comme "echo")
#     commande_str = ' '.join(commande)
#     print(f"\nEssai de la commande `{commande_str}` dans : {dossier}")
#     print(dossier.replace("\\","\\"))
#     print("C:\\Users\\Aurélien\\OneDrive\\Bureau\\work\\ownwork\\app_proto")

#     # resultat = subprocess.run(
#     #     "cargo clean",
#     #     cwd="C:\\Users\\Aurélien\\OneDrive\\Bureau\\work\\ownwork\\app_proto",
#     #     # check=True,
#     #     # capture_output=True,
#     #     # text=True,
#     #     shell=True
#     # )
    
#     # commande_liste = ["cargo", "clean"] =
#     # commande_str = " ".join(commande_liste)

#     # Deuxième commande
#     # dossier = "C:\\Users\\Aurélien\\OneDrive\\Bureau\\work\\ownwork\\app_proto"  # assurez-vous que ce chemin est correct
#     resultat = subprocess.run(
#         commande_str,
#         cwd=str(dossier),
#         shell=True
#     )
#     # print(f"\n{resultat2}")
    
#     # resultat = subprocess.run(
#     #     commande_str,
#     #     cwd=str(dossier),
#     #     # check=True,
#     #     # capture_output=True,
#     #     # text=True,
#     #     shell=True
#     # )
#     try:
#         print(f"Commande exécutée avec succès dans {dossier}")
#         print("Sortie :")
#         print(resultat.stdout)
#         # On a réussi dans ce dossier : on arrête d'explorer plus bas pour cette branche.
#         return True
#     except subprocess.CalledProcessError as e:
#         print(f"Erreur lors de l'exécution dans {dossier} : {e}")
#         print("Recherche dans les sous-dossiers...")
#         # La commande a échoué : on parcourt récursivement tous les sous-dossiers.
#         succes = False
#         try:
#             for element in os.listdir(dossier):
#                 chemin_element = os.path.join(dossier, element)
#                 if os.path.isdir(chemin_element):
#                     # Pour chaque sous-dossier, on essaie d'exécuter la commande.
#                     if executer_commande_recursif(commande, chemin_element):
#                         succes = True
#             if not succes:
#                 print(f"La commande n'a pas pu être exécutée correctement dans aucun sous-dossier de {dossier}.")
#         except Exception as sous_erreur:
#             print(f"Impossible de parcourir les sous-dossiers de {dossier} : {sous_erreur}")
#         return succes
#     except Exception as e:
#         print(f"Erreur inattendue dans {dossier} : {e}")
#         return False

# if __name__ == "__main__":
#     parser = argparse.ArgumentParser(
#         description="Exécute une commande (avec arguments) dans le répertoire courant et ses sous-dossiers récursivement, "
#                     "jusqu'à obtenir une exécution réussie (sans CalledProcessError)."
#     )
#     parser.add_argument(
#         "commande",
#         nargs="+",
#         help="La commande à exécuter, avec ses arguments (exemple : echo hello)"
#     )
    
#     args = parser.parse_args()
    
#     # On démarre la recherche à partir du répertoire courant.
#     executer_commande_recursif(args.commande, os.getcwd())




# # # import os
# # # import subprocess
# # # import argparse

# # # def executer_commande_dans_dossiers(commande):
# # #     # Récupère le chemin du répertoire courant
# # #     dossier_courant = os.getcwd()
    
# # #     # Parcourt chaque élément dans le répertoire courant
# # #     for element in os.listdir(dossier_courant):
# # #         chemin_element = os.path.join(dossier_courant, element)
# # #         # Vérifie si l'élément est un dossier
# # #         if os.path.isdir(chemin_element):
# # #             print(f"\nExécution de la commande dans : {chemin_element}")
# # #             try:
# # #                 # On reconstruit la commande sous forme de chaîne
# # #                 commande_str = ' '.join(commande)
# # #                 # Exécute la commande dans le dossier spécifié avec shell=True
# # #                 resultat = subprocess.run(
# # #                     commande_str,
# # #                     cwd=chemin_element,
# # #                     check=True,
# # #                     capture_output=True,
# # #                     text=True,
# # #                     shell=True
# # #                 )
# # #                 print("Sortie :")
# # #                 print(resultat.stdout)
# # #             except subprocess.CalledProcessError as e:
# # #                 print(f"Une erreur est survenue dans {chemin_element} : {e}")
# # #                 print("Message d'erreur :", e.stderr)
# # #             except FileNotFoundError as e:
# # #                 print(f"Commande non trouvée dans {chemin_element} : {e}")
# # #             except Exception as e:
# # #                 print(f"Erreur inattendue dans {chemin_element} : {e}")

# # # if __name__ == "__main__":
# # #     # Configuration de l'analyseur d'arguments
# # #     parser = argparse.ArgumentParser(
# # #         description="Exécute une commande (avec ses arguments) dans tous les dossiers du répertoire courant."
# # #     )
# # #     parser.add_argument(
# # #         "commande",
# # #         nargs="+",
# # #         help="La commande à exécuter, avec ses arguments (exemple : echo hello)"
# # #     )
    
# # #     args = parser.parse_args()
    
# # #     # La commande est récupérée sous forme de liste
# # #     executer_commande_dans_dossiers(args.commande)
