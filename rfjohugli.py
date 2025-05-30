taille = 6
grilla = []
null_state = ' '

for i in range(taille):
    temp = []
    for j in range(taille):
        # temp.append(str((i+j+1 -j*i + i%2 + j %2)%3-1))
        temp.append(null_state)
    grilla.append(temp)

# grilla = [  ['0', ' ', ' ', '1', ' ', ' ', ' ', ' '],
#             [' ', '1', ' ', ' ', '0', ' ', ' ', '1'],
#             [' ', ' ', ' ', ' ', ' ', '1', ' ', '1'],
#             ['1', ' ', ' ', '0', ' ', ' ', ' ', ' '],
#             [' ', ' ', '0', ' ', ' ', '1', ' ', ' '],
#             [' ', ' ', '0', '1', ' ', ' ', ' ', '0'],
#             ['0', ' ', ' ', '1', ' ', '1', '1', ' '],
#             ['1', '0', ' ', ' ', ' ', '0', ' ', ' ']]

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

# grilla = [  [' ', ' ', ' ', ' ', ' ', ' '],
#             [' ', '0', ' ', '1', ' ', ' '],
#             [' ', ' ', ' ', ' ', '0', ' '],
#             [' ', '1', ' ', '1', ' ', ' '],
#             [' ', ' ', ' ', ' ', ' ', ' '],
#             [' ', ' ', ' ', ' ', ' ', ' ']]

# grilla = [  [' ', ' ', ' ', ' ', ' ', ' '],
#             [' ', ' ', ' ', ' ', ' ', ' '],
#             [' ', ' ', ' ', ' ', ' ', ' '],
#             [' ', ' ', ' ', ' ', ' ', ' '],
#             [' ', ' ', ' ', ' ', ' ', ' '],
#             [' ', ' ', ' ', ' ', ' ', ' ']]

# grilla = [  [' ', ' ', ' ', ' '],
#             ['1', '0', ' ', ' '],
#             ['0', ' ', '0', ' '],
#             [' ', '1', '0', ' ']]

# grilla = [  [' ', ' ', ' ', ' '],
#             [' ', '1', ' ', ' '],
#             [' ', ' ', ' ', ' '],
#             [' ', ' ', ' ', ' ']]

# grilla = [  [' ', ' ', ' ', ' '],
#             [' ', ' ', ' ', ' '],
#             [' ', ' ', ' ', ' '],
#             [' ', ' ', ' ', ' ']]

grilla = [  [' ', ' ', ' ', '0', ' ', '1', '0', ' ', '1', ' '],
            ['0', ' ', ' ', '0', ' ', ' ', '1', ' ', '0', ' '],
            [' ', '0', '0', ' ', '0', ' ', ' ', ' ', ' ', '0'],
            ['0', ' ', '1', ' ', ' ', ' ', ' ', '0', ' ', ' '],
            [' ', ' ', ' ', ' ', '1', ' ', '1', ' ', ' ', '1'],
            [' ', ' ', '0', ' ', ' ', ' ', ' ', ' ', '1', ' '],
            [' ', ' ', ' ', ' ', '0', ' ', ' ', ' ', '0', '0'],
            [' ', ' ', ' ', '1', ' ', '1', ' ', '0', ' ', ' '],
            [' ', '0', ' ', ' ', ' ', ' ', ' ', '1', ' ', '0'],
            [' ', ' ', ' ', '1', ' ', ' ', ' ', ' ', '1', '1']]
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
        for J,j in enumerate(griller):
            if I != J and i == j and not(null_state in i):
                return (False,"grille incorrecte : la ligne %d : %s est la même que le ligne %d : %s" % (I, str(i), J, str(j)))

    # check instant colones            
    trans = transpose(griller)
    for I,i in enumerate(trans):
        for J,j in enumerate(trans):
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
    trans = transpose(griller)
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

def score_case(griz, x,y):
    sc = 0
    if x+1<len(griz) and griz[x+1][y] != ' ':
        sc += 10
    if x-1>=0 and griz[x-1][y] != ' ':
        sc += 10
    if y+1<len(griz[0]) and griz[x][y+1] != ' ':
        sc += 10
    if y-1>=0 and griz[x][y-1] != ' ':
        sc += 10

    if x+1<len(griz) and y+1<len(griz[0]) and griz[x+1][y+1] != ' ':
        sc += 1
    if x+1<len(griz) and y-1>=0 and griz[x+1][y-1] != ' ':
        sc += 1
    if x-1>=0 and y+1<len(griz[0]) and griz[x-1][y+1] != ' ':
        sc += 1
    if x-1>=0 and y-1>=0 and griz[x-1][y-1] != ' ':
        sc += 1

    for i in range(len(griz)):
        if  griz[x][y-i] != ' ':
            sc *= 2
        if  griz[x-i][y] != ' ':
            sc *= 2
    return sc

def resolve(grille):
    coups_possible = []
    coups_possible_temp = []
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
                        # coups_possible_temp.append(grille_temp_0)
                        coups_possible_temp.append((grille_temp_0, score_case(grille,E,F)))
                    if po_1[0]:
                        # coups_possible_temp.append(grille_temp_1)
                        coups_possible_temp.append((grille_temp_1, score_case(grille,E,F)))
    # coups_possible = coups_possible_temp
    while coups_possible_temp != []:
        max_score = coups_possible_temp[0][1]
        for e in coups_possible_temp:
            if e[1] > max_score:
                max_score = e[1]
        decalage=0
        for i in range(len(coups_possible_temp)):
            if coups_possible_temp[i-decalage][1] == max_score:
                coups_possible.append(coups_possible_temp.pop(i-decalage)[0])
                decalage+=1

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


global Hached
# Hached=[[[[[]* (3**taille)]* (3**taille)]* (3**taille)]* (3**taille) ]

def recusrif_crete(legntha, init_value = 0,tailla=0):
    if tailla ==0:
        return init_value
    else:
        list_ro_return = []
        for e in range(legntha):
            list_ro_return.append(recusrif_crete(legntha, init_value,tailla-1))
        return list_ro_return

# Hached = []
# for i0 in range(3**taille):
#     temp0=[]
#     for i1 in range(3**taille):
#         temp1=[]
#         for i2 in range(3**taille):
#             temp2=[]
#             for i3 in range(3**taille):
#                 temp3=0
#                 temp2.append(temp3)
#             temp1.append(temp2)
#         temp0.append(temp1)
#     Hached.append(temp0)
Hached = [[]]*(3**len(grilla))
# Hached = recusrif_crete(3**len(grilla), 0, len(grilla))



def hache(grillage):
    global Hached
    num=[0]*len(grillage)
    for E,e in enumerate(grillage):
        for F,f in enumerate(e):
            if f == "0":
                num[E] += 1*(3**F)
            elif f == "1":
                num[E] += 2*(3**F)
            # elif f == " ":
            #     num[E] += 0*(3**F)
    print(num, grillage, val_at_indice(Hached, num))
    # if Hached[num[0]][num[1]][num[2]][num[3]] == 0:
    #     Hached[num[0]][num[1]][num[2]][num[3]] = 1
    #     return False
    
    if val_at_indice(Hached, num) == 0:
        change(Hached, num, 1)
        return False
    else:
        return True

def recusrif_crete_condi(gri,legntha, init_value = 0,tailla=0):
    if tailla ==0:
        return init_value
    else:
        list_ro_return = []
        for e in range(legntha):
            list_ro_return.append(recusrif_crete(legntha, init_value,tailla-1))
        return list_ro_return


def hache_condi(grillage):
    global Hached
    num = hac_val(grillage)
    val_at_indice_condi(Hached, num, grillage)
    # change(Hached, num, 1)
    # print(num, grillage, val_at_indice(Hached, num))
    if val_at_indice(Hached, num) == 0:
        change(Hached, num, 1)
        return False
    else:
        return True

def hac_val(grillage):
    num=[0]*len(grillage)
    for E,e in enumerate(grillage):
        for F,f in enumerate(e):
            if f == "0":
                num[E] += 1*(3**F)
            elif f == "1":
                num[E] += 2*(3**F)
    return num


def val_at_indice(liste_hache, num):
    if len(num)>1:
        return val_at_indice(liste_hache[num[0]], num[1:])
    else:
        return liste_hache[num[0]]

def val_at_indice_condi(liste_hache, num, grillon):
    # print(liste_hache, num, grillon)
    
    if liste_hache[num[0]] == [] and len(num)>2:
        liste_hache[num[0]] = [[]]*(3**len(grillon))
        # print()
        # print(liste_hache)
        # print()
    if liste_hache[num[0]] == [] and len(num)==2:
        liste_hache[num[0]] = [0]*(3**len(grillon))
        # print()
        # print(liste_hache)
        # print()
    if len(num)>2:
        return val_at_indice_condi(liste_hache[num[0]], num[1:], grillon)
    else:
        # if liste_hache[num[0]] == []:
        #     liste_hache[num[0]] = [0]*(3**len(grillon))
        #     print()
        #     print(liste_hache)
        #     print()
        return liste_hache[num[0]]

def change(liste_hache, num, value):
    if len(num)>1:
        change(liste_hache[num[0]], num[1:], value)
    else:
        liste_hache[num[0]] = value


global global_index, global_index_done, num_possi, final_listing_re, listing_re
final_listing_re= []
listing_re= []
num_possi = 0
global_index=0
global_index_done=0
import time
def resolve_complete(grilling, index = 0):
    global global_index, global_index_done, num_possi, final_listing_re, listing_re
    if hache_condi(grilling):
        raise "already calculated"
    print(index, "|", global_index, "|", global_index_done,"       ", end='\r')
    while inside(null_state, grilling):
        # time.sleep(1)
        results_resolve = resolve(grilling)
        if results_resolve[1] != []:
            grilling = results_resolve[1][0]
        else:
            # print()
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
            if len(results_resolve[0])>1:
                # print("cette grille possède %d coup possible" % len(list_possible))
                global_index+=len(results_resolve[0])
            for e in results_resolve[0]:
                try:
                    # print(len(results_resolve[0]))
                    # time.sleep(0.5)
                    # pretty_print(e)
                    # print()
                    co_possible = resolve_complete(e, index+1)
                    list_possible.append(co_possible)
                    # listing_re.append(co_possible)
                    if co_possible not in final_listing_re:
                        print("solution trouvé %d       " % (len(final_listing_re)+1))
                        pretty_print(co_possible)
                        final_listing_re.append(co_possible)
                    # print(list_possible)
                    # return co_possible
                except:
                    pass
            if len(results_resolve[0])>1:
                # print("cette grille possède %d coup possible" % len(list_possible))
                global_index_done+=len(results_resolve[0])
            # grilling=list_possible[len(list_possible)-1]
            # print(list_possible)
            grilling=list_possible[0]
            
            if index == 0:
                # final_list=[]
                # for e in listing_re:
                #     if not(e in final_listing_re):
                #         final_listing_re.append(e)
                num_possi = len(final_listing_re)
                # for e in final_list:
                #     # time.sleep(1)
                #     print(len(final_list))
                #     pretty_print(e)
                #     # print()
                # final_list=[]
                # for e in list_possible:
                #     if not(e in final_list):
                #         final_list.append(e)
                # for e in final_list:
                # for e in list_possible:
                #     # time.sleep(1)
                #     # print(len(final_list))
                #     print(len(list_possible))
                #     pretty_print(e)
                #     # print()
        # print()
        # pretty_print(grilling)
        # print()
    return grilling


start_time = time.time()
# raise "fuck you"
print(verify_grille(grilla))
pretty_print(grilla)
print()
grilla = resolve_complete(grilla)


# for e in final_listing_re:
#     # time.sleep(1)
#     # print(len(final_list))
#     print(len(final_listing_re))
#     pretty_print(e)

print("possibilitées :", max(num_possi,1), "        ")
pretty_print(grilla)
some =open("tempa.txt", 'w')
some.write(str(Hached))
some.close()
# print(Hached)

print("--- %s seconds ---" % (time.time() - start_time))