import sys

import load_data
from sklearn.metrics import confusion_matrix, accuracy_score
from sklearn.decomposition import PCA
from sklearn.svm import SVC
from time import time
from extract_feature import wavelets_f
import numpy as np
from sklearn.model_selection import GridSearchCV, KFold, cross_val_score

num_class = int(sys.argv[1])
train_data, train_label, test_data, test_label = load_data.load_data(num_class)
train_data = np.array(train_data)
train_label = np.array(train_label)
test_data = np.array(test_data)
test_label = np.array(test_label)
print("Data load successfully!")
print("The train data shape is: ")
print(train_data.shape)
print("The test data shape is: ")
print(test_data.shape)

train_data = wavelets_f(train_data, threshold=0.02)

n_components = 0.98
t0 = time()
pca = PCA(n_components=n_components, whiten=False).fit(train_data)
print("done in %0.3fs" % (time() - t0))

print("pca components: ", pca.components_.shape)


print("Projecting the input data on the eigenfaces orthonormal basis")
t0 = time()
X_train_pca = pca.transform(train_data)

print("done in %0.3fs" % (time() - t0))
print("Fitting the classifier to the training set")

t0 = time()
clf = SVC(C=1000.0, class_weight='balanced', gamma=0.001)

n_split = 10
kfold = KFold(n_splits=n_split)
X_train_pca = np.array(X_train_pca)
count = 0
for train_index, test_index in kfold.split(X_train_pca):
    clf.fit(X_train_pca[train_index], train_label[train_index])
    print(accuracy_score(clf.predict(X_train_pca[test_index]), train_label[test_index]))

scores = cross_val_score(clf, X_train_pca, train_label, cv=kfold, n_jobs=-1)
print("Accuracy: %0.2f (+/- %0.2f)" % (scores.mean(), scores.std() * 2))
