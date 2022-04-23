import numpy as np
X = np.array([...]) # poids
Y = np.array([...]) # taille

def error(a):
    y_pred      = X * a 
    y_observed  = Y 
    size        = len(X)

    diff = sum((y_pred - y_observed)**2)/size
    return diff

def derror(a):
    size = len(X)
    return -2/size * sum(X * (Y - a * X))
    
def descent_gradient(a=-20, taux = 400000):

    grad = 100.0 
    while True:

        grad = derror(a) 
        g = grad/ taux

        if -0.5 <= grad <= 0.5:
            return a

        a += -g