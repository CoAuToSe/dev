taille = 8
grilla = []
null_state = ' '

for i in range(taille):
    temp = []
    for j in range(taille):
        # temp.append(str((i+j+1 -j*i + i%2 + j %2)%3-1))
        temp.append(null_state)
    grilla.append(temp)

grilla = [  ['0', ' ', ' ', '1', ' ', ' ', ' ', ' '],
            [' ', '1', ' ', ' ', '0', ' ', ' ', '1'],
            [' ', ' ', ' ', ' ', ' ', '1', ' ', '1'],
            ['1', ' ', ' ', '0', ' ', ' ', ' ', ' '],
            [' ', ' ', '0', ' ', ' ', '1', ' ', ' '],
            [' ', ' ', '0', '1', ' ', ' ', ' ', '0'],
            ['0', ' ', ' ', '1', ' ', '1', '1', ' '],
            ['1', '0', ' ', ' ', ' ', '0', ' ', ' ']]

# grilla = [  ['0', ' ', ' ', ' ', '0', ' ', '0', ' '],
#             ['0', ' ', ' ', ' ', ' ', '1', ' ', ' '],
#             [' ', ' ', ' ', '1', ' ', ' ', '0', ' '],
#             [' ', ' ', '0', ' ', '0', ' ', ' ', ' '],
#             [' ', ' ', ' ', ' ', ' ', ' ', ' ', '1'],
#             ['0', ' ', '0', ' ', '0', '0', ' ', ' '],
#             [' ', ' ', ' ', ' ', ' ', ' ', '1', ' '],
#             [' ', '1', ' ', ' ', ' ', ' ', ' ', ' ']]

# grilla = [  [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
#             [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
#             [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
#             [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
#             [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
#             [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
#             [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
#             [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ']]

# grilla = [  [' ', ' ', ' ', ' '],
#             ['1', '0', ' ', ' '],
#             ['0', ' ', '0', ' '],
#             [' ', '1', '0', ' ']]

# grilla = [  [' ', ' ', ' ', '0', ' ', '1', '0', ' ', '1', ' '],
#             ['0', ' ', ' ', '0', ' ', ' ', '1', ' ', '0', ' '],
#             [' ', '0', '0', ' ', '0', ' ', ' ', ' ', ' ', '0'],
#             ['0', ' ', '1', ' ', ' ', ' ', ' ', '0', ' ', ' '],
#             [' ', ' ', ' ', ' ', '1', ' ', '1', ' ', ' ', '1'],
#             [' ', ' ', '0', ' ', ' ', ' ', ' ', ' ', '1', ' '],
#             [' ', ' ', ' ', ' ', '0', ' ', ' ', ' ', '0', '0'],
#             [' ', ' ', ' ', '1', ' ', '1', ' ', '0', ' ', ' '],
#             [' ', '0', ' ', ' ', ' ', ' ', ' ', '1', ' ', '0'],
#             [' ', ' ', ' ', '1', ' ', ' ', ' ', ' ', '1', '1']]
# grilla[0][0]="0"
# grilla[0][2]="0"
# grilla[3][0]="1"
# grilla[2][1]="0"
# grilla[3][1]="0"
# grilla[3][2]="0"
# grilla[3][3]="1"
# grilla[0][0]="1"
# grilla[0][0]="1"

def verify_grille(griller):

    # check instant lignes
    for I,i in enumerate(griller):
        for J,j in enumerate(griller[I:]):
            if I != J and i == j and not(null_state in i):
                return (False,"grille incorrecte : la ligne %d : %s est la même que le ligne %d : %s" % (I, str(i), J, str(j)))

    # check instant colones            
    trans = transpose(griller)
    for I,i in enumerate(trans):
        for J,j in enumerate(trans[I:]):
            if I != J and i == j and not(null_state in i):
                return (False,"grille incorrecte : la colone %d : %s est la même que le colone %d : %s" % (I, str(i), J, str(j)))

    # check instant nombre de 0 et 1 en ligne
    for I,i in enumerate(griller):
        # if not(null_state in i):
        num_0 = 0
        num_1 = 0
        for J,j in enumerate(i):
            if j == "0":
                num_0 += 1
            if j == "1":
                num_1 += 1
        if num_0 != num_1 and not(null_state in i):
            return (False,"grille incorrecte : la ligne %d : %s est composé d'un nombre différent de 0:%d et de 1:%d" % (I, str(i), num_0, num_1))
        if num_0 > len(griller)//2:
            return (False,"grille incorrecte : la ligne %d : %s est composé d'un trop grand nombre de 0:%d " % (I, str(i), num_0))
        if num_1 > len(griller)//2:
            return (False,"grille incorrecte : la ligne %d : %s est composé d'un trop grand nombre de 1:%d " % (I, str(i), num_1))

    # check instant nombre de 0 et 1 en colone    
    # trans = transpose(griller)
    for I,i in enumerate(trans):
        # if not(null_state in i):
        num_0 = 0
        num_1 = 0
        for J,j in enumerate(i):
            if j == "0":
                num_0 += 1
            if j == "1":
                num_1 += 1
        if num_0 != num_1 and not(null_state in i):
            return (False,"grille incorrecte : la colone %d : %s est composé d'un nombre différent de 0:%d et de 1:%d" % (I, str(i), num_0, num_1))
        if num_0 > len(griller)//2:
            return (False,"grille incorrecte : la colone %d : %s est composé d'un trop grand nombre de 0:%d " % (I, str(i), num_0))
        if num_1 > len(griller)//2:
            return (False,"grille incorrecte : la colone %d : %s est composé d'un trop grand nombre de 1:%d " % (I, str(i), num_1))

    # check instant de tout les coups joués
    for I,i in enumerate(griller):
        for J,j in enumerate(i):
            results = verif_coup(griller, I, J)
            if not results[0]:
                return results

    return (True,"grille correcte %s" % str(griller))

def transpose(tableau):
    returned = []
    for i in range(len(tableau)):
        temp = []
        for j in range(len(tableau[0])):
            temp.append(tableau[j][i])
        returned.append(temp)
    return returned


def verif_coup(grillin, posx, posy):
    def campl(val):
        # print(val)
        if val >= len(grillin):
            return len(grillin)-1
        if val < 0:
            return 0
        return val

    def temp(dx,dy):
        if (campl(posy+dy) != posy+dy and dy !=0) or (campl(posx+dx) != posx+dx and dx !=0):
            # print("passed because clamped : %d|%d and %d|%d." % (posy, campl(posy+dy) , posx, campl(posx+dx)))
            return False
        if grillin[posx][posy] == null_state or grillin[campl(posx+dx)][campl(posy+dy)] == null_state:
            # print("passed because : %s and %s." % (grillin[posx][posy], grillin[campl(posx+dx)][campl(posy+dy)]))
            return False
        if dx ==0 and dy == 0:
            raise "u dumb"
        if dx==0:
            # print("comparing dy=%d:" % dy, grillin[posx][posy],"and", grillin[posx][campl(posy+dy)])
            # if campl(posy+dy) == posy
            return grillin[posx][posy] == grillin[posx][campl(posy+dy)]
        if dy==0:
            # print("comparing dx=%d:" % dx, grillin[posx][posy], "and", grillin[campl(posx+dx)][posy])
            return grillin[posx][posy] == grillin[campl(posx+dx)][posy]

    if temp(0,1):
        if temp(0,2):
            return (False, "coup incorrecte: la case en %s est la même qu'en %s" % (str(posx)+str(posy),str(posx)+str(posy+2) ))
        if temp(0,-1):
            return (False, "coup incorrecte: la case en %s est la même qu'en %s" % (str(posx)+str(posy),str(posx)+str(posy-1) ))

    if temp(0,-1):
        if temp(0,-2):
            return (False, "coup incorrecte: la case en %s est la même qu'en %s" % (str(posx)+str(posy),str(posx)+str(posy-2) ))
        if temp(0,1):
            return (False, "coup incorrecte: la case en %s est la même qu'en %s" % (str(posx)+str(posy),str(posx)+str(posy+1) ))
    
    if temp(1,0):
        if temp(2,0):
            return (False, "coup incorrecte: la case en %s est la même qu'en %s" % (str(posx)+str(posy),str(posx+2)+str(posy) ))
        if temp(-1,0):
            return (False, "coup incorrecte: la case en %s est la même qu'en %s" % (str(posx)+str(posy),str(posx-1)+str(posy) ))

    if temp(-1,0):
        if temp(-2,0):
            return (False, "coup incorrecte: la case en %s est la même qu'en %s" % (str(posx)+str(posy),str(posx-2)+str(posy) ))
        if temp(1,0):
            return (False, "coup incorrecte: la case en %s est la même qu'en %s" % (str(posx)+str(posy),str(posx+1)+str(posy) ))
    

    return (True, "coup correcte")

def pretty_print(grille):
    for e in grille:
        print(e)


                # les_coup_forcees_implique =resolve(grillin)[1]
                # for e in les_coup_forcees_implique:
                #     if not verify_grille(e)[0]:
                #         return (False, "coup incorrecte: ce coup implique une grille impossible")

def resolve(grille):
    coups_possible = []
    coups_forces = []
    for E,e in enumerate(grille):
        for F,f in enumerate(e):
            if f == null_state:
                # print("checking %d %d" % (E,F))
                # pretty_print(grille)
                # print()
                grille_temp_0 = clone(grille)
                grille_temp_0[E][F] = "0"

                grille_temp_1 = clone(grille)
                grille_temp_1[E][F] = "1"
                po_0 = verify_grille(grille_temp_0)
                po_1 = verify_grille(grille_temp_1)


                # print("grille 0:", po_0)
                # pretty_print(grille_temp_0)
                # print("grille 1:", po_1)
                # pretty_print(grille_temp_1)

                # if po_0[0]:
                #     coups_possible.append(grille_temp_0)
                # if po_1[0]:
                #     coups_possible.append(grille_temp_1)
                if not( po_0[0] and po_1[0]):
                    # print("some: 0:%d | 1:%d " %(po_0[0],po_1[0]))
                    if po_0[0]:
                        # print("returned 0")
                        coups_forces.append(grille_temp_0)
                    if po_1[0]:
                        # print("returned 1")
                        coups_forces.append(grille_temp_1)
                    # if po_0[0] == po_1[0]:
                    #     print()
                    #     print(E,F)
                    #     pretty_print(grille)
                    #     raise "on ne peut rien placer ici"
                else:
                    if po_0[0]:
                        coups_possible.append(grille_temp_0)
                    if po_1[0]:
                        coups_possible.append(grille_temp_1)

    if len(coups_possible)==0 and len(coups_forces)==0:
        raise "impossible de jouer"
    # print()
    # for e in coups_possible:
    #     print("possible")
    #     pretty_print(e)
    # for e in coups_forces:
    #     print("forced")
    #     pretty_print(e)
    # print()
    return (coups_possible,coups_forces)
    
def clone(tableu):
    lo = []
    for i in tableu:
        temp = []
        for j in i:
            temp.append(j)
        lo.append(temp)
    return lo

def inside(val, tableu):
    for i in tableu:
        for j in i:
            if j == val:
                return True
    return False

global global_index, global_index_done
global_index=0
global_index_done=0
import time
def resolve_complete(grilling, index = 0):
    global global_index, global_index_done
    print(index, "|", global_index, "|", global_index_done, end='\r')
    while inside(null_state, grilling):
        # time.sleep(1)
        results_resolve = resolve(grilling)
        if results_resolve[1] != []:
            grilling = results_resolve[1][0]
        else:
            # pretty_print(grilling)
            # for e in results_resolve[0]:
            #     time.sleep(1)
            #     print(len(results_resolve[0]))
            #     pretty_print(e)
            #     print()
            # for e in results_resolve[1]:
            #     time.sleep(1)
            #     print()
            #     pretty_print(e)
            #     print()
            # raise "fuck you"
            list_possible = []
            # if len(results_resolve[0])>1:
            #     # print("cette grille possède %d coup possible" % len(list_possible))
            #     global_index+=len(results_resolve[0])
            for e in results_resolve[0]:
                try:
                    # print(len(results_resolve[0]))
                    # time.sleep(0.5)
                    # pretty_print(e)
                    # print()
                    co_possible = resolve_complete(e, index+1)
                    list_possible.append(co_possible)
                    return co_possible
                except:
                    pass
            if len(results_resolve[0])>1:
                # print("cette grille possède %d coup possible" % len(list_possible))
                global_index_done+=len(results_resolve[0])
            # grilling=list_possible[len(list_possible)-1]
            grilling=list_possible[0]
            # for e in list_possible:
            #     time.sleep(1)
            #     print()
            #     pretty_print(e)
            #     print()
        # print()
        # pretty_print(grilling)
        # print()
    return grilling

global index_lo,last_solve
index_lo = 0
last_solve= []

def resolve_complete2(grilling, index = 0):
    global index_lo, last_solve
    while inside(null_state, grilling):
        index_lo +=1
        print(index_lo, index, end='\r')
        results_resolve = resolve(grilling)
        if results_resolve[1] != []:
            grilling = results_resolve[1][0]
        else:
            list_possible = []
            for e in results_resolve[0]:
                try:
                        
                    temp_grille = e
                    results_resolvingt = resolve(temp_grille)
                    while results_resolvingt[1] != []:
                        results_resolvingt = resolve(temp_grille)
                        if results_resolvingt[1] != []:
                            temp_grille = results_resolvingt[1][0]
                    list_possible.append(temp_grille)
                except:
                    pass
            if list_possible == []:
                raise "grille pas possible"
            for e in list_possible:
                try:
                    grilling=resolve_complete2(e,index +1)
                except:
                    pass
    return grilling

def resolveur(grille, index = 0):
    list_coups_possible_general = [(grille,counte(null_state, grille), 0)]
    dont_end = True
    while dont_end:
        index +=1
        print(index, end='\r')
        len_coup_possible_gene = len(list_coups_possible_general)
        if len_coup_possible_gene == 0:
            raise "impossible de continuer"

            
        supposition_min = list_coups_possible_general[0][2]
        for i in range(len(list_coups_possible_general)):
            if supposition_min > list_coups_possible_general[i][2]:
                supposition_min = list_coups_possible_general[i][2]
        
        coup_inte = []
        for e in list_coups_possible_general:
            if supposition_min == e[2]:
                coup_inte.append(e)

        list_coups_possible = []
        if coup_inte != []:
            len_coup_possible = len(coup_inte)
            list_coups_possible = coup_inte
        else:
            len_coup_possible = len_coup_possible_gene
            list_coups_possible = list_coups_possible_general
        
        for i in range(len_coup_possible):
            coup_actuelle = list_coups_possible[i]
            if completed(coup_actuelle[0]):
                return coup_actuelle[0]

            results_resolve = resolve(coup_actuelle[0])

            if results_resolve[1] != []:
                temp_grille = coup_actuelle[0]
                while results_resolve[1] != []:
                    results_resolve = resolve(temp_grille)
                    if results_resolve[1] != []:
                        temp_grille = results_resolve[1][0]
                list_coups_possible_general.append((temp_grille, counte(null_state, temp_grille), coup_actuelle[2]))
            else:
                for e in results_resolve[0]:
                    list_coups_possible_general.append((temp_grille, counte(null_state, temp_grille), coup_actuelle[2]+1))

        for E,e in enumerate(list_coups_possible_general):
            if e in list_coups_possible:
                list_coups_possible_general.pop(E)

        # list_coups_possible = trie_liste(list_coups_possible)
        for e in list_coups_possible:
            print(e[1], e[2])
            pretty_print(e[0])
            print()
        # dont_end = False
    return list_coups_possible[0][0]
        
def trie_liste(grillon):
    list_returned = []
    while grillon != []:
        if len(grillon) == 1:
            list_returned.append(grillon.pop())
        else:
            min_indice = 0
            for i in range(len(grillon)):
                if grillon[min_indice][1] > grillon[i][1]:
                    min_indice = i
            
            list_returned.append(grillon.pop(min_indice))
    return list_returned



def completed(grilleer):
    return not inside(null_state, grilleer)

def counte(fal, fol):
    num = 0
    for i in fol:
        for j in i:
            if j == fal:
                num +=1
    return num

# raise "fuck you"
print(verify_grille(grilla))
pretty_print(grilla)
print()
grilla = resolve_complete(grilla)
# grilla = resolve_complete2(grilla)
# grilla = resolveur(grilla)
print("possibilitées :", max(global_index,1))
pretty_print(grilla)