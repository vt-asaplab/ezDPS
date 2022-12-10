import sys

from load_data import unpickle
from time import time

from sklearn.model_selection import train_test_split, KFold, cross_val_score
from sklearn.metrics import confusion_matrix, accuracy_score
from sklearn.decomposition import PCA
from sklearn.svm import SVC
from sklearn.utils import shuffle
import numpy as np


train_path = 'cifar-100-python/train'
test_path = 'cifar-100-python/test'
train = unpickle(train_path)
test = unpickle(test_path)

train_data = np.array(train['data'])
train_label = np.array(train['fine_labels'])
test_data_or = np.array(test['data'])
test_label_or = np.array(test['fine_labels'])
print(test_label_or)

num_class = int(sys.argv[1])
train_sub_data = []
train_sub_label = []
test_sub_data = []
test_sub_label = []

for i in range(len(train_label)):
    if train_label[i] < num_class:
        train_sub_label.append(train_label[i])
        train_sub_data.append(train_data[i])

shuf_data = []
shuf_label = []
n_split = 5
for i in range(n_split):
    shuf_file_name = "shu_list/cross_"+str(num_class)+"_shu_"+str(i)+".txt"
    shuf = np.loadtxt(shuf_file_name, dtype=int)
    count = 0
    for j in shuf:
        shuf_data.append(train_sub_data[j])
        shuf_label.append(train_sub_label[j])
    for j in range(len(train_label)):
        if j not in shuf:
            print(j)
            shuf_data.append(train_sub_data[j])
            shuf_label.append(train_sub_label[j])
            count += 1
        if count >= 800:
            break

shuf_data, shuf_label = shuffle(shuf_data,shuf_label,random_state=42)

n_components = 0.93
t0 = time()
pca = PCA(n_components=n_components, whiten=True).fit(shuf_data)
print("done in %0.3fs" % (time() - t0))

shuf_pca = pca.transform(shuf_data)

print("done in %0.3fs" % (time() - t0))

clf = SVC(C=1000.0, cache_size=200, class_weight='balanced', coef0=0.0,
    decision_function_shape='ovr', degree=3, gamma=0.001, kernel='rbf',
    max_iter=-1, probability=False, random_state=None, shrinking=True,
    tol=0.001, verbose=False)

n_split = 10
kfold = KFold(n_splits=n_split)
shuf_pca = np.array(shuf_pca)
shuf_label = np.array(shuf_label)
count = 0
for train_index, test_index in kfold.split(shuf_pca):
    clf.fit(shuf_pca[train_index], shuf_label[train_index])
    print(accuracy_score(clf.predict(shuf_pca[test_index]), shuf_label[test_index]))

scores = cross_val_score(clf, shuf_pca, shuf_label, cv=kfold, n_jobs=-1)
print("Accuracy: %0.2f (+/- %0.2f)" % (scores.mean(), scores.std() * 2))

