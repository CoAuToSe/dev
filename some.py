global PROFONDEUR, JOUEUR, IA, VIDE
PROFONDEUR = 3
VIDE = "-"
JOUEUR = "X"
IA = "O"
        
def affichage(T) :

    print("    1   2   3")
    print("  ┌───┬───┬───┐")
    print("A │",T[0][0],"│",T[0][1],"│",T[0][2],"│")
    print("  ├───┼───┼───┤")
    print("B │",T[1][0],"│",T[1][1],"│",T[1][2],"│")
    print("  ├───┼───┼───┤")
    print("C │",T[2][0],"│",T[2][1],"│",T[2][2],"│")
    print("  └───┴───┴───┘")
    
    return
    
def possibilites(plateau, joueur):
    liste_possibilites = []

    len_plateau = len(plateau)

    x = joueur
    if "-" not in plateau[0] and "-" not in plateau[1] and "-" not in plateau[
            2]:
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


def cree_plateau(taille):
    liste = []
    for i in range(taille):
        row = []
        for j in range(taille):
            row.append('-')
        liste.append(row)
    return liste


def affiche_plateau_avec_score(score, plateau):
    print("score:")
    print(score)
    print("plateau:")
    for ligne in plateau:
        print(ligne)



def is_there_any_1_in_2(carac, board):
    there_is = False
    for row in board:
        for case in row:
            if case == carac:
                there_is = True
    return there_is


def score_coup(utilisateur, adversaire, coup, profondeur):
    if profondeur <= 0 or is_player_win(coup, utilisateur) != 0 or not(is_there_any_1_in_2("-", coup)):
        return is_player_win(coup, utilisateur)
    else:
        # on regarde comment l'adversaire va pouvoir répondre au coup joué
        possibilite_de_jouer = possibilites(coup, adversaire)
        score_du_coup = 0.0
        for coup_possible in possibilite_de_jouer:
            # on additionne l'opposé des scores de ce que pourra répondre l'adversaire à ce coup 
            temp = score_coup(adversaire, utilisateur, coup_possible, profondeur - 1)
            # score_du_coup -= temp * ((abs(score_du_coup * temp / len(possibilite_de_jouer)))**(2/3)+1)
            score_du_coup -= temp 
        score_du_coup /= len(possibilite_de_jouer)
        # score_du_coup = score_du_coup / len(possibilite_de_jouer)
        return score_du_coup

def score_coup_into_1(ref_container, utilisateur, adversaire, coup, profondeur):
    if profondeur <= 0 or is_player_win(coup, utilisateur) != 0 or not(is_there_any_1_in_2("-", coup)):
        ref_container[0] = is_player_win(coup, utilisateur)
        ref_container[1] = coup
        return is_player_win(coup, utilisateur)
    else:
        # on regarde comment l'adversaire va pouvoir répondre au coup joué
        possibilite_de_jouer = possibilites(coup, adversaire)
        score_du_coup = 0.0
        for coup_possible in possibilite_de_jouer:
            # on additionne l'opposé des scores de ce que pourra répondre l'adversaire à ce coup (on veut pas qu'il gagne même en jouant son meilleur coup à chaque fois)
            temp = score_coup(adversaire, utilisateur, coup_possible, profondeur - 1)
            # score_du_coup -= temp * ((abs(score_du_coup * temp / len(possibilite_de_jouer)))**(2/3)+1)
            score_du_coup -= temp
        # score_du_coup /= len(possibilite_de_jouer)
        score_du_coup = score_du_coup / len(possibilite_de_jouer)
        ref_container[0] = score_du_coup
        ref_container[1] = coup
        return score_du_coup


# plateau_de_depart = cree_plateau(3)
#affiche_plateau_avec_score(0, plateau_de_depart)
#(score, nouveau_plateau) = fais_pousser_la_branche(IA, JOUEUR, plateau_de_depart, 10)
#affiche_plateau_avec_score(score, nouveau_plateau)
# plateau_de_depart[1][1] = JOUEUR
# plateau_de_depart[2][0] = JOUEUR
# plateau_de_depart[1][2] = JOUEUR

# list_thread = []
# for e in possibilites(plateau_de_depart, JOUEUR):
#     list_thread.append(Thread(target= print, args=(score_coup(JOUEUR, IA, e, 100), e)))

# for e in list_thread:
#     e.start()
# for e in list_thread:
#     e.join()
# plateau_de_depart[1][1] = JOUEUR
# plateau_de_depart[0][1] = IA
# plateau_de_depart[2][2] = JOUEUR
# print(is_player_win(plateau_de_depart, JOUEUR))

# FONCTION A NE PAS UTILISER
# retourne le coup à jouer peu être remplacé par "show_me_how_to_win"
def joue(utilisateur, advedsaire, plateau_de_depart, profondeur):
    from threading import Thread
    list_thread = []
    something = [[], []]
    for e in possibilites(plateau_de_depart, utilisateur):
        something[0] += [[0.0, []]]
        list_thread.append(Thread(target= score_coup_into_1, args=(something[0][-1], utilisateur, advedsaire, e, profondeur)))

    for e in possibilites(plateau_de_depart, advedsaire):
        something[1] += [[0.0, []]]
        list_thread.append(Thread(target= score_coup_into_1, args=(something[1][-1], advedsaire, utilisateur, e, profondeur)))

    for e in list_thread:
        e.start()
    for e in list_thread:
        e.join()
    print(something)

    list_score = [[],[]]
    list_board = [[],[]]
    for index, player in enumerate(something):
        for e in player:
            list_score[index] += [e[0]]
            list_board[index] += [e[1]]
    print(list_score, list_board)


    best_score = (list_score[0][0], 0)
    for E, e in enumerate(list_score[0]):
        if best_score[0] <= e:
            best_score = (e,E)

    best_enemie_score = (list_score[1][0], 0)
    for E, e in enumerate(list_score[1]):
        if best_enemie_score[0] <= e:
            best_enemie_score = (e,E)

    print(best_score, best_enemie_score)


    if best_enemie_score[0] == 1000.0:
        return (-best_enemie_score[0], list_board[0][best_enemie_score[1]])
    else:
        return (best_score[0], list_board[0][best_score[1]])

plateau_de_depart = cree_plateau(3)
# plateau_de_depart[1][1] = IA

# plateau_de_depart[1][1] = JOUEUR
# plateau_de_depart[0][1] = IA
# plateau_de_depart[2][2] = JOUEUR
# plateau_de_depart[0][0] = IA
jouee = plateau_de_depart
affichage(jouee)

# représente une partie IA vs IA
index = 0
profondeur_actuel = 9
while False and not(is_player_win(jouee, JOUEUR) != 0 or is_player_win(jouee, IA) != 0):
    if index%2 == 0:
        score, jouee = joue(JOUEUR, IA, jouee, profondeur_actuel)
        print(score)
        affichage(jouee)
    else:
        score, jouee = joue(IA, JOUEUR, jouee, profondeur_actuel)
        print(score)
        affichage(jouee)
    index += 1
# print(joue(IA, JOUEUR, plateau_de_depart, 3))

# retourne le nombre de "carac" dans la liste "lista"
def count(carac, lista):
    index = 0
    for e in lista:
        if e == carac:
            index += 1
    return index
          
# retourne l'adversaire de "joueur"
def adversaire(joueur):
    if joueur == IA:
        return JOUEUR
    elif joueur == JOUEUR:
        return IA
    raise "joueur inconnu"

# retourne si le plateau est remplie ou non
def plateau_est_remplie(my_plateau):
    list_agree = []
    for e in my_plateau:
        if count(VIDE, e) != 0:
            list_agree.append(False)
    return len(list_agree) == 0

# réecriture de la fonction présente dans la fiche de l'APP (fonction du prof)
def eval_configuration(plateau, joueur):

    plateau_remplie = plateau_est_remplie(plateau) 
    un_gagnant = is_player_win(plateau, joueur) == 1000.0 or is_player_win(plateau, adversaire(joueur)) == 1000.0


    if plateau_remplie or un_gagnant:
        if plateau_remplie and not(un_gagnant):
            return 0
        else:
            if is_player_win(plateau, joueur) == 1000.0:
                return 1000
            elif is_player_win(plateau, adversaire(joueur)) == 1000.0:
                return -1000
        return 0
    else:

        def return_lignes_colones_diagonales(plateau_mine):
            num_element = len(plateau_mine)
            lignes = []
            colones = []
            diagonales = [[], []]
            for i in range(num_element):
                lignes += [[]]
                colones += [[]]
                for j in range(num_element):
                    lignes[-1] += [plateau_mine[i][j]]
                for j in range(num_element):
                    colones[-1] += [plateau_mine[j][i]]
                diagonales[0] += [plateau_mine[i][i]]
                diagonales[1] += [plateau_mine[i][num_element - 1 - i]]
            return lignes, colones, diagonales

        lignes, colones, diagonales = return_lignes_colones_diagonales(plateau)
        # print(lignes, colones, diagonales)

        somme = 0
        component_plateau = [lignes, colones, diagonales]
        for component in component_plateau:
            for element in component:
                if adversaire(joueur) in element and joueur in element:
                    pass
                else:
                    if count(adversaire(joueur), element) == 2:
                        somme -= 30
                    elif count(joueur, element) == 2:
                        somme += 30
                    elif count(adversaire(joueur), element) == 1:
                        somme -= 10
                    elif count(joueur, element) == 1:
                        somme += 10
        return somme

def return_tige(plateau_branche, score_branche, bourgeons_branche):
    return [(plateau_branche, score_branche), bourgeons_branche]

# retourne l'abre représantant le jeu dans son intégralité avec le score de chaque coup(en utilisant la fontion proposé par le prof(eval_configuration))
def tree_maker_prof(le_premier_qui_joue, taille = 3, l_autre = "", plateau = ["@"]):
    if l_autre == "":
        l_autre = adversaire(le_premier_qui_joue)

    if plateau ==  ["@"]:
        plateau = cree_plateau(taille)

    score = eval_configuration(plateau, le_premier_qui_joue)
    if abs(score) != 1000 and not(plateau_est_remplie(plateau)):
        bourgeons = possibilites(plateau, l_autre)
    else:
        bourgeons = []

    branche = return_tige(plateau, score, bourgeons)
    for index, tige in enumerate(branche[1]):
        branche[1][index] = tree_maker_prof(l_autre, plateau=tige)
    return [branche]

# retourne l'abre représantant le jeu dans son intégralité avec le score de chaque coup(en utilisant la fontion score_coup)
def tree_maker_score(le_premier_qui_joue, taille = 3, l_autre = "", plateau = ["@"]):
    if l_autre == "":
        l_autre = adversaire(le_premier_qui_joue)
    if plateau ==  ["@"]:
        plateau = cree_plateau(taille)

    score = score_coup(le_premier_qui_joue, l_autre,plateau, profondeur = 10)
    if abs(score) != 1000.0 and not(plateau_est_remplie(plateau)):
        bourgeons = possibilites(plateau, l_autre)
    else:
        bourgeons = []

    branche = return_tige(plateau, score, bourgeons)
    for index, tige in enumerate(branche[1]):
        branche[1][index] = tree_maker_score(l_autre, plateau=tige)
    return [branche]

plateau_de_depart = cree_plateau(3)
plateau_de_depart[1][1] = "O"
plateau_de_depart[0][0] = "X"
plateau_de_depart[0][2] = "O"
# print(eval_configuration(plateau_de_depart))
# for e in possibilites(plateau_de_depart, IA):
#     print(eval_configuration(e, IA))
for e in possibilites(plateau_de_depart, JOUEUR):
    print(eval_configuration(e, JOUEUR))


# plateau_de_depart = cree_plateau(3)
# print(tree_maker(IA))

#retour le meilleur coup que le joueur(player) pourrais faire
def show_me_how_to_win(current_state, player):
    if not(is_player_win(current_state, player=player) == 1000.0 or is_player_win(current_state, player=adversaire(player)) == 1000.0):
        scores = []
        for coup_possible in possibilites(current_state, player):
            scores += [(score_coup(player, adversaire(player), coup_possible, 10), coup_possible)]
        best_move = scores[0]
        for e in scores:
            if best_move[0] <= e[0]:
                best_move = e
        return best_move
    else:
        if is_player_win(current_state, player=player) == 1000.0:
            print("You already won", player)
            return (1000.0, current_state)
        if is_player_win(current_state, player=adversaire(player)) == 1000.0:
            print("You already lost", player)
            return (1000.0, current_state)

# retourne la liste des coup à faire pour un situation initiale(current_state) en fonction de celui qui doit jouer(player)
def show_me_how_to_really_win(current_state, player, original_player ="¤"):
    if original_player == "¤":
        original_player = player
    the_moves = []
    while not(is_player_win(current_state, player=player) == 1000.0 or is_player_win(current_state, player=adversaire(player)) == 1000.0 ) and is_there_any_1_in_2("-", current_state):
        scores = []
        for coup_possible in possibilites(current_state, player):
            scores += [(score_coup(player, adversaire(player), coup_possible, 10), coup_possible)]
        if player == original_player:
            best_move = scores[0]
            for e in scores:
                if best_move[0] <= e[0]:
                    best_move = e
            current_state = best_move[1]
            the_moves += [best_move]
        else:
            worst_move = scores[0]
            for e in scores:
                if worst_move[0] >= e[0]:
                    worst_move = e
            current_state = worst_move[1]
            the_moves += [worst_move]
        player = adversaire(player)
        # print(the_moves)
    return the_moves

#retour le pire coup que le joueur(player) pourrais faire
def show_me_how_to_loose(current_state, player):
    if not(is_player_win(current_state, player=player) == 1000.0 or is_player_win(current_state, player=adversaire(player)) == 1000.0):
        scores = []
        for coup_possible in possibilites(current_state, player):
            scores += [(score_coup(player, adversaire(player), coup_possible, 10), coup_possible)]
        worst_move = scores[0]
        for e in scores:
            if worst_move[0] >= e[0]:
                worst_move = e
        return worst_move
    else:
        if is_player_win(current_state, player=player) == 1000.0:
            print("You already won", player)
            return (1000.0, current_state)
        if is_player_win(current_state, player=adversaire(player)) == 1000.0:
            print("You already lost", player)
            return (1000.0, current_state)


plateau_de_depart = cree_plateau(3)
plateau_de_depart = show_me_how_to_win(plateau_de_depart, IA)[1]
print(plateau_de_depart)
plateau_de_depart = show_me_how_to_loose(plateau_de_depart, JOUEUR)[1]
print(plateau_de_depart)
plateau_de_depart = show_me_how_to_win(plateau_de_depart, IA)[1]
print(plateau_de_depart)
plateau_de_depart = show_me_how_to_loose(plateau_de_depart, JOUEUR)[1]
print(plateau_de_depart)
plateau_de_depart = show_me_how_to_win(plateau_de_depart, IA)[1]
print(plateau_de_depart)
plateau_de_depart = show_me_how_to_loose(plateau_de_depart, JOUEUR)[1]
print(plateau_de_depart)
plateau_de_depart = show_me_how_to_win(plateau_de_depart, IA)[1]
print(plateau_de_depart)
plateau_de_depart = show_me_how_to_loose(plateau_de_depart, JOUEUR)[1]
print(plateau_de_depart)
plateau_de_depart = show_me_how_to_win(plateau_de_depart, IA)[1]
print(plateau_de_depart)
plateau_de_depart = show_me_how_to_loose(plateau_de_depart, JOUEUR)[1]
print(plateau_de_depart)
plateau_de_depart = show_me_how_to_win(plateau_de_depart, IA)[1]
print(plateau_de_depart)
plateau_de_depart = show_me_how_to_loose(plateau_de_depart, JOUEUR)[1]
print(plateau_de_depart)
print()
plateau_de_depart = cree_plateau(3)
print(show_me_how_to_really_win(plateau_de_depart, IA))