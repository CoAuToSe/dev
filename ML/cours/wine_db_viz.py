import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
import sklearn
from socket import gethostname as gh

# download the csv file first !
csv_file ='winequality-white.csv'
df = pd.read_csv(csv_file , header ='infer', delimiter = ';')



print(df.head())

X = df.drop('quality', axis =1) # we drop the column " quality "

print('Average ')
print(X.mean())

print('\nStandard deviation')
print(X.std())



corr = X.corr()
plt.title(gh())
sns.heatmap(corr )
plt.show()