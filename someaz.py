# -*- coding: utf-8 -*-
"""
Created on Sun Feb 13 23:17:24 2022

@author: clemd
"""

import random

def create_T(taille):
    T=[]
    for i in range(taille):
        row = []
        for j in range(taille):
            row.append('-')
        T.append(row)
    return T

def get_random_first_player():
    return random.randint(0, 1)

def affichage(T) :
    print("              1   2   3")
    print("            ┌───┬───┬───┐")
    print("          A │",T[0][0],"│",T[0][1],"│",T[0][2],"│")
    print("            ├───┼───┼───┤")
    print("          B │",T[1][0],"│",T[1][1],"│",T[1][2],"│")
    print("            ├───┼───┼───┤")
    print("          C │",T[2][0],"│",T[2][1],"│",T[2][2],"│")
    print("            └───┴───┴───┘")
    print("\n══════════════════════════════════════")
    return


def fix_spot(player,T):
    
    case = str(input("Dans quelle case voulez-vous jouer ?\n( ex : B1 ) \n"))
    if len(case) == 2:
        row = str(case[0])
        col = int(case[1])
    elif len(case) == 3:
        row = str(case[0])
        col = int(case[2])
    elif len(case) >3 or len(case) <2:
        print("\n    █ Valeur de la case erronée")
        print("\n══════════════════════════════════════")
        fix_spot(player)
        return
    
    if row == "a" or row == "A":
        row = 1
    if row == "b" or row == "B":
        row = 2
    if row == "c" or row == "C":
        row = 3
        
    if col > 3 or col == 0:
        print("\n    █ La case n'existe pas")
        print("\n══════════════════════════════════════")
        fix_spot(player)
        return
    
    if T[row-1][col-1] == "X" or T[row-1][col-1] == "O": 
        print("\n    █ La case est déjà prise")
        print("\n══════════════════════════════════════")
        fix_spot(player)
        return
    
    T[row-1][col-1] = player
    
    return T

def verif_plus(plateau) :
    
    n = len(plateau)
    
    # Vérification des lignes
    
    for i in range (n) :
        
        compteurX = 0
        compteurO = 0
        
        for j in range (n) :
            
            if plateau[i][j] == "X" :
                
                compteurX += 1
                
                if compteurX == 2 :
                    
                    for k in range (n) :
                        
                        if plateau[i][k] == "-" :
                            
                            plateau[i][k] = "A"
                
            elif plateau[i][j] == "O" :
                
                compteurO += 1
                
                if compteurO == 2 :
                    
                    for k in range (n) :
                        
                        if plateau[i][k] == "-" :
                            
                            plateau[i][k] = "A"
    
    # Vérification des colonnes
    
    for i in range (n) :
        
        compteurX = 0
        compteurO = 0
        
        for j in range (n) :
        
            if plateau[j][i] == "X" :
                
                compteurX += 1
                
                if compteurX == 2 :
                    
                    for k in range (n) :
                        
                        if plateau[k][i] == "-" :
                            
                            plateau[k][i] = "A"
                            
            elif plateau[j][i] == "O" :
                
                compteurO += 1
                
                if compteurO == 2 :
                    
                    for k in range (n) :
                        
                        if plateau[k][i] == "-" :
                            
                            plateau[k][i] = "A"
                            
    # Vérification des diagonales
    
    compteurX = 0
    compteurO = 0
    
    for i in range (n) :
        
        if plateau[i][i] == "X" :
            
            compteurX += 1
            
            if compteurX == 2 :
                
                for k in range (n) :
                    
                    if plateau[k][k] == "-" :
                        
                        plateau[k][k] = "A"
                        
                
        elif plateau[i][i] == "O" :
            
            compteurO += 1
            
            if compteurO == 2 :
                
                for k in range (n) :
                    
                    if plateau[k][k] == "-" :
                        
                        plateau[k][k] = "A"
                        
                        
    compteurX = 0
    compteurO = 0
                        
    for i in range (n) :
        
        if plateau[i][n - 1 - i] == "X" :
            
            compteurX += 1
            
            if compteurX == 2 :
                
                for k in range (n) :
                    
                    if plateau[k][n - 1 - k] == "-" :
                        
                        plateau[k][n - 1 - k] = "A"
                        
        elif plateau[i][n - 1 - i] == "O" :
            
            compteurO += 1
            
            if compteurO == 2 :
                
                for k in range (n) :
                    
                    if plateau[k][n - 1 - k] == "-" :
                        
                        plateau[k][n - 1 - k] = "A"
                
    return plateau

def possibilites(plateau, joueur):
    
    liste_possibilites = []

    len_plateau = len(plateau)

    x = joueur
    if "-" not in plateau[0] and "-" not in plateau[1] and "-" not in plateau[2]:
        print("Pas de possibilités.")
        return liste_possibilites

    else:
        nombre_de_vide = 0
        for ligne in plateau:
            for case in ligne:
                if case == "-":
                    nombre_de_vide += 1

        for i in range(nombre_de_vide):
            liste_possibilites.append(plateau)

        chaine = ""

        #transforme les tableaux de plateau en une chaine de caractère
        for plateau_possible in liste_possibilites:
            for ligne in plateau_possible:
                for case in ligne:
                    chaine += str(case)
            chaine += ":"

        # sépare les différents plateau pour créer une liste de plateau
        chaine = chaine.split(":")
        chaine.pop()  # enlève l'élement vide à la fin

        position = chaine[0].find("-")
        chainebis = chaine[0][:position] + x + chaine[0][position + 1:]
        chaine[0] = chainebis

        for i in range(1, nombre_de_vide):
            position = chainebis.find("-")
            chainebis = chainebis[:position] + x + chainebis[position + 1:]
            chaine[i] = chaine[i][:position] + x + chaine[i][position + 1:]

        tableau_plateau = []
        ligne = []
        index_ligne = 0

        for str_plateau in chaine:
            for char_case in str_plateau:
                ligne += [char_case]
                index_ligne += 1

                if index_ligne == len_plateau:
                    tableau_plateau += [ligne]
                    ligne = []
                    index_ligne = 0

        nombre_ligne = 0
        un_plateau = []
        liste_plateau = []
        for une_ligne in tableau_plateau:
            un_plateau += [une_ligne]
            nombre_ligne += 1
            if nombre_ligne == len_plateau:
                liste_plateau += [un_plateau]
                un_plateau = []
                nombre_ligne = 0

    return liste_plateau

def is_there_any__in_(carac, board):
    there_is = False
    for row in board:
        for case in row:
            if case == carac:
                there_is = True
    return there_is

def score_coup(utilisateur, adversaire, coup, profondeur):
    if profondeur <= 0 or is_player_win(
            coup, utilisateur) != 0 or not(is_there_any__in_("-", coup)):
        return is_player_win(coup, utilisateur)
    else:
        # on regarde comment l'adversaire va pouvoir répondre au coup joué
        possibilite_de_jouer = possibilites(coup, adversaire)
        score_du_coup = 0.0
        for coup_possible in possibilite_de_jouer:
            #on additionne l'opposé des scores de ce que pourra répondre l'adversaire à ce coup 
            score_du_coup -= score_coup(adversaire, utilisateur, coup_possible,
                                        profondeur - 1)
        #score_du_coup /= len(possibilite_de_jouer)
        score_du_coup = score_du_coup / len(possibilite_de_jouer)
        return score_du_coup

def fix_spot_J_VS_IA(player, palteau, profondeur):
    
    if player == "X" :
        suceed_choice = False
        while not(suceed_choice):
            case = str(input("Dans quelle case voulez-vous jouer ?\n( ex : B1 ) \n"))
            if len(case) == 2:
                row = str(case[0])
                col = int(case[1])
            elif len(case) == 3:
                row = str(case[0])
                col = int(case[2])
            elif len(case) >3 or len(case) <2:
                print("\n    █ Valeur de la case erronée")
                print("\n══════════════════════════════════════")
                continue
                # fix_spot_J_VS_IA(player, palteau = palteau, profondeur = profondeur)
                # return T
            
            if row == "a" or row == "A":
                row = 1
            if row == "b" or row == "B":
                row = 2
            if row == "c" or row == "C":
                row = 3
                
            if col > 3 or col == 0:
                print("\n    █ La case n'existe pas")
                print("\n══════════════════════════════════════")
                continue
                # fix_spot_J_VS_IA(player, palteau = palteau, profondeur = profondeur)
                # return T
            
            if palteau[row-1][col-1] == "X" or palteau[row-1][col-1] == "O": 
                print("\n    █ La case est déjà prise")
                print("\n══════════════════════════════════════")
                continue
                # fix_spot_J_VS_IA(player, palteau = palteau, profondeur = profondeur)
                # return T
            suceed_choice = True
        palteau[row-1][col-1] = player
        return palteau
        
    elif player == "O" :
        
        T = verif_plus(palteau)
        
        for i in range (len(T)) :
            for j in range (len (T)) :
                if T[i][j] == "A" :
                    
                    T[i][j] = "O"
                    
                    for a in range (len(T)) :
                        for b in range (len(T)) :
                            if T[a][b] == "A" :
                                T[a][b] = "-"
                    return T
                
        scores = []
        for i in possibilites(T, player) :
            
            scores += [(score_coup("O", "X", i, profondeur), i)]
            # print(score, i)
        
        score_max = scores[0][0]
        meilleur_coup = scores[0][1]
        for e in scores:
            if e[0] >= score_max :
                
                score_max = e[0]
                meilleur_coup = e[1]
                
        T = meilleur_coup
        return T

def is_player_win(T, player):
    #print(T,player)
    win = 0.0

    
    #print(1000.0*coeff_utilisateur)

    n = len(T)

    # checking rows
    for i in range(n):
        win = 1000.0
        for j in range(n):
            if T[i][j] != player:
                win = 0.0
                break
        if win == 1000.0:
            return 1000.0

    # checking columns
    for i in range(n):
        win = 1000.0
        for j in range(n):
            if T[j][i] != player:
                win = 0.0
                break
        if win == 1000.0:
            return 1000.0

    # checking diagonals
    win = 1000.0
    for i in range(n):
        if T[i][i] != player:
            win = 0.0
            break
    if win == 1000.0:
        return 1000.0

    win = 1000.0
    for i in range(n):
        if T[i][n - 1 - i] != player:
            win = 0.0
            break
    if win == 1000.0:
        return 1000.0
    return 0.0

def is_T_filled(T):
    is_filled = True
    for row in T :
        for item in row:
            if item == "-":
                is_filled = False
    return is_filled

def swap_player_turn(player):
    return 'X' if player == 'O' else 'O'


# retourne l'adversaire de "joueur"
def adversaire(joueur):
    if joueur == "O":
        return "X"
    elif joueur == "X":
        return "O"
    raise "joueur inconnu"

def humanvhuman():
    
    T = create_T(3)

    player = 'X' if get_random_first_player() == 1 else 'O'
    
    while True:
        
        print("\n══════════════════════════════════════")
        print("    Joueur de symbole ",player," de jouer")
        print("══════════════════════════════════════\n")
        affichage(T)

        fix_spot(player,T)

        if is_player_win(T, player) == 1000.0:
            print("\n══════════════════════════════════════")
            print("         Joueur ",player," a gagné !")
            break

        if is_player_win(T, adversaire(player)) == 1000.0:
            print("\n══════════════════════════════════════")
            print("         Joueur ",adversaire(player)," a gagné !")
            break

        print(is_T_filled(T))
        if is_T_filled(T):
            print("\n══════════════════════════════════════")
            print("              Égalité!")
            break

        player = swap_player_turn(player)

    print("══════════════════════════════════════\n")
    
    affichage(T)

def J_VS_IA (profondeur,taille) :
    
    T = create_T(taille)
    
    player = 'X' if get_random_first_player() == 1 else 'O'
    
    while True:
        
        print("\n══════════════════════════════════════")
        print("    Joueur de symbole ",player," de jouer")
        print("══════════════════════════════════════\n")
        affichage(T)
        # print(fix_spot_J_VS_IA(player,T,profondeur))
        T = fix_spot_J_VS_IA(player,T,profondeur)
        print(is_player_win(T, player))
        print(is_player_win(T, adversaire(player)))

        if is_player_win(T, player) == 1000.0:
            print("\n══════════════════════════════════════")
            print("         Joueur ",player," a gagné !")
            break

        if is_player_win(T, adversaire(player)) == 1000.0:
            print("\n══════════════════════════════════════")
            print("         Joueur ",adversaire(player)," a gagné !")
            break


        if is_T_filled(T):
            print("\n══════════════════════════════════════")
            print("              Égalité!")
            break


        player = swap_player_turn(player)


    print("══════════════════════════════════════\n")
    print(T)
    affichage(T)
    
    
    

def menu():

    print("""
══════════════════════════════════════


 __  __                  _
|  \/  | ___  _ __ _ __ (_) ___  _ __   
| |\/| |/ _ \| '__| '_ \| |/ _ \| '_ \  
| |  | | (_) | |  | |_) | | (_) | | | | 
|_|  |_|\___/|_|  | .__/|_|\___/|_| |_| 
                   |_|
""")
    choix = str(input("""
══════════════════════════════════════

    A: Joueur VS IA
    B: Joueur1 VS Joueur2
    C: IA VS IA

    Veuillez choisir le mode de jeu: """))

    if choix == "A" or choix == "a":
        choixdepth = str(input("""
══════════════════════════════════════ 
                           
    Difficulté de l'IA entre 1 et 10: """))
        if int(choixdepth) >= 1 and int(choixdepth) <= 10 :
            profondeur = int(choixdepth)
            choixtaille = str(input("""
══════════════════════════════════════

    1: 3 x 3
    2: 4 x 4
                           
    Taille du plateau: """))
            if int(choixtaille) == 1 or int(choixtaille) == 2:
                taille = int(choixtaille)+2
                J_VS_IA (profondeur,taille)
                return
            
            else:
                print("\n    █ Choix invalide, retour au menu principal")
                menu()
                return
            
        else:
            print("\n    █ Choix invalide, retour au menu principal")
            menu()
            return
        
    if choix == "B" or choix == "b":
        humanvhuman()
        return
				
    if choix == "C" or choix == "c":
        choixdepth = str(input("""
══════════════════════════════════════ 

    Difficulté de l'IA entre 1 et 10: """))
        if choixdepth == "1" or "2" or "3":
            depth = int(choixdepth)
            choixtaille = str(input("""
══════════════════════════════════════ 

   1: 3 x 3
   2: 4 x 4
                           
   Taille du plateau: """))
            if int(choixtaille) == 1 or int(choixtaille) == 2:
                taille = int(choixtaille)
                return morpionclassique(depth,taille)
            else:
                print("\n    █ Choix invalide, retour au menu principal")
                menu()
                return
    else: 
        print("\n    █ Choix hors menu, veuillez réessayer...")

        menu()
            
menu()