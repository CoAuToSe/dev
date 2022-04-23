# import numpy as np
# list_point = []
# signe = lambda a : abs(a)/a
# for i in range(0,100,1):
#     x = np.random.random_sample()*2-1
#     y = ((1-x**2)**(1/2))* signe(np.random.random_sample()*2-1)
#     list_point.append((x,y))
# print(list_point)

import numpy as np
import matplotlib.pyplot as plt

# data_file = open("C:/Users/Aurel/Downloads/stage/log20210621_151955_imm_CAL.csv", "r")
data_file = open("C:/Users/Aurel/OneDrive/Documents/dev/01_07_2021/log20210709_080231_imm_.csv", "r")
lignes = data_file.readlines()
data_file.close()
list_datas_lignes = []
list_datas_colone = [[],[],[],[]]
# print(lignes[0].split(';'))
index_to_show = ['TKx', 'TKy', 'TKz']
colum_to_show = [-1] * len(index)
# print(colum, lignes[0].split(';'))
for E,e in enumerate(lignes[0].split(';')):
    for F,f in enumerate(index):
        # print(E,F,e,f)
        if e == f:
            colum_to_show[F] = E

print(colum_to_show)
for E,e in enumerate(lignes[1:len(lignes)-1]):
    datas_splited = e.split(";")
    # print(len(datas_splited))
    data1 = int(datas_splited[colum_to_show[0]])#TKx / TKx raw 
    data2 = int(datas_splited[colum_to_show[1]])#TKy / TKy raw
    data3 = int(datas_splited[colum_to_show[2]])#TKz / TKz raw
    data4 = (data1*data1 + data2*data2 + data3*data3)**(1/2)
    list_datas_lignes.append([data1,data2,data3,data4])
    list_datas_colone[0].append(data1)
    list_datas_colone[1].append(data2)
    list_datas_colone[2].append(data3)
    list_datas_colone[3].append(data4)

okazd = []
for i in range(0,len(list_datas_colone[0])):
    okazd.append(i)

fig = plt.figure()
ax = fig.add_subplot(projection='3d')
xs = list_datas_colone[0]
ys = list_datas_colone[1]
zs = list_datas_colone[2]
ax.scatter(xs, ys, zs)

ax.set_xlabel('X Label')
ax.set_ylabel('Y Label')
ax.set_zlabel('Z Label')

plt.show()