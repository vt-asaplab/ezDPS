import sys

from load_data import unpickle
from time import time
from extract_feature import wavelets_f

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
test_data = np.array(test['data'])
test_label = np.array(test['fine_labels'])

num_class = int(sys.argv[1])
train_sub_data = []
train_sub_label = []
test_sub_data = []
test_sub_label = []

for i in range(len(train_label)):
    if train_label[i] < num_class:
        train_sub_label.append(train_label[i])
        train_sub_data.append(train_data[i])

for i in range(len(test_label)):
    if test_label[i] < num_class:
        test_sub_data.append(test_data[i])
        test_sub_label.append(test_label[i])

train_sub_data = np.array(train_sub_data)
train_sub_label = np.array(train_sub_label)
test_sub_data = np.array(test_sub_data)
test_sub_label = np.array(test_sub_label)

shuf_data = train_sub_data
shuf_label = train_sub_label

shuf_data = wavelets_f(shuf_data, threshold=0.01)

n_components = 0.93
t0 = time()
pca = PCA(n_components=n_components, whiten=True).fit(shuf_data)
print("done in %0.3fs" % (time() - t0))

shuf_data = pca.transform(shuf_data)

print("done in %0.3fs" % (time() - t0))

clf = SVC(C=1000, gamma=0.001, kernel='rbf')

clf.fit(shuf_data, shuf_label)

shuf_data_test = wavelets_f(test_sub_data, threshold=0.01)
shuf_data_test = pca.transform(shuf_data_test)

scores = clf.score(shuf_data_test, test_sub_label)
print("Accuracy: %0.2f" % scores)



