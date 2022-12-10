import sys
from time import time
import logging

from sklearn.model_selection import train_test_split, KFold, cross_val_score
from sklearn.datasets import fetch_lfw_people
from sklearn.metrics import confusion_matrix, accuracy_score
from sklearn.decomposition import PCA
from sklearn.svm import SVC
from extract_feature import wavelets_f
from sklearn.utils import shuffle
import numpy as np

num_class = int(sys.argv[1])

class_param = [[57, 0],
               [42, 1],
               [31, 0],
               [19, 5],
               [11, 15]]
para_tag = 0
if num_class == 8:
    para_tag = 0
elif num_class == 16:
    para_tag = 1
elif num_class == 32:
    para_tag = 2
elif num_class == 64:
    para_tag = 3
elif num_class == 128:
    para_tag = 4

para_0 = class_param[para_tag][0]
para_1 = class_param[para_tag][1]

lfw_people = fetch_lfw_people(min_faces_per_person=para_0, resize=0.7)

n_samples, h, w = lfw_people.images.shape
print("h", h)
print("w", w)

X = lfw_people.data
n_features = X.shape[1]

y = lfw_people.target
target_names = lfw_people.target_names
n_classes = target_names.shape[0]

print("Total dataset size:")
print("n_samples: %d" % n_samples)
print("n_features: %d" % n_features)
print("n_classes: %d" % n_classes)

count = para_1
X_sub = []
Y_sub = []

cur = 0  # cur指示当前项
lab = []
while count != 0:
    if y[cur] not in lab:
        lab.append(y[cur])
        count = count - 1
    cur += 1

for i in range(len(y)):
    if y[i] not in lab:
        X_sub.append(X[i])
        Y_sub.append(y[i])

print(np.array(X_sub).shape)
print(np.array(Y_sub).shape)

X_sub = np.array(X_sub)
Y_sub = np.array(Y_sub)

shuf_data = []
shuf_label = []
n_split = 5
shuf_file_name = "shu_list/cross_"+str(num_class)+"_shu.txt"
shuf = np.loadtxt(shuf_file_name, dtype=int)
count = 0
for j in shuf:
    shuf_data.append(X_sub[j])
    shuf_label.append(Y_sub[j])
for j in range(len(Y_sub)):
    if j not in shuf:
        shuf_data.append(X_sub[j])
        shuf_label.append(Y_sub[j])
        count += 1
    if count >= 50:
        break

shuf_data, shuf_label = shuffle(shuf_data, shuf_label, random_state=42)

shuf_data = np.array(shuf_data)
shuf_label = np.array(shuf_label)

shuf_data = wavelets_f(shuf_data, threshold=0.01)

n_components = 0.9

print("Extracting the top %d eigenfaces from %d faces"
      % (n_components, shuf_data.shape[0]))
t0 = time()
pca = PCA(n_components=n_components, whiten=True).fit(shuf_data)
print("done in %0.3fs" % (time() - t0))

t0 = time()
shuf_pca = pca.transform(shuf_data)

print("done in %0.3fs" % (time() - t0))

print("Fitting the classifier to the training set")
t0 = time()
clf = SVC(C=1000.0, cache_size=200, class_weight='balanced', coef0=0.0,
    decision_function_shape='ovr', degree=3, gamma=0.005, kernel='rbf',
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



