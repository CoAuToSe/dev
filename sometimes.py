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


# grille super dure
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
        if val >= len(grillin):
            return len(grillin)-1
        if val < 0:
            return 0
        return val

    def temp(dx,dy):
        if (campl(posy+dy) != posy+dy and dy !=0) or (campl(posx+dx) != posx+dx and dx !=0):
            return False
        if grillin[posx][posy] == null_state or grillin[campl(posx+dx)][campl(posy+dy)] == null_state:
            return False
        if dx ==0 and dy == 0:
            raise "u dumb"
        if dx==0:
            return grillin[posx][posy] == grillin[posx][campl(posy+dy)]
        if dy==0:
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
    sc = 1
    if x+1<len(griz) and griz[x+1][y] != ' ':
        sc += 10
    if x-1>=0 and griz[x-1][y] != ' ':
        sc += 10
    if y+1<len(griz[0]) and griz[x][y+1] != ' ':
        sc += 10
    if y-1>=0 and griz[x][y-1] != ' ':
        sc += 10

    if x+1<len(griz) and y+1<len(griz[0]) and griz[x+1][y+1] != ' ':
        sc += 2
    if x+1<len(griz) and y-1>=0 and griz[x+1][y-1] != ' ':
        sc += 2
    if x-1>=0 and y+1<len(griz[0]) and griz[x-1][y+1] != ' ':
        sc += 2
    if x-1>=0 and y-1>=0 and griz[x-1][y-1] != ' ':
        sc += 2

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
                grille_temp_0 = clone(grille)
                grille_temp_0[E][F] = "0"

                grille_temp_1 = clone(grille)
                grille_temp_1[E][F] = "1"

                po_0 = verify_grille(grille_temp_0)
                po_1 = verify_grille(grille_temp_1)

                if not( po_0[0] and po_1[0]):
                    if po_0[0]:
                        coups_forces.append(grille_temp_0)
                    if po_1[0]:
                        coups_forces.append(grille_temp_1)
                else:
                    if po_0[0]:
                        coups_possible_temp.append((grille_temp_0, score_case(grille,E,F)))
                    if po_1[0]:
                        coups_possible_temp.append((grille_temp_1, score_case(grille,E,F)))
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


global global_index, num_possi, final_listing_re, listing_re
final_listing_re= []
listing_re= []
num_possi = 0
global_index=0

def resolve_complete(grilling, index = 0):
    global global_index, num_possi, final_listing_re, listing_re
    print(index, "|", global_index,"       ", end='\r')
    while inside(null_state, grilling):
        results_resolve = resolve(grilling)
        if results_resolve[1] != []:
            grilling = results_resolve[1][0]
        else:
            list_possible = []
            if len(results_resolve[0])>1:
                global_index+=len(results_resolve[0])
            for e in results_resolve[0]:
                try:
                    co_possible = resolve_complete(e, index+1)
                    list_possible.append(co_possible)
                    if co_possible not in final_listing_re:
                        print("solution trouvé %d       " % (len(final_listing_re)+1))
                        pretty_print(co_possible)
                        final_listing_re.append(co_possible)
                except:
                    pass
            grilling=list_possible[0]
            
            if index == 0:
                num_possi = len(final_listing_re)
    return grilling

import time
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