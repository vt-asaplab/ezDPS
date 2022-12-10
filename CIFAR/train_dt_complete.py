from sklearn import tree
import numpy as np
from load_data import unpickle
from sklearn.utils import shuffle
from sklearn.model_selection import GridSearchCV, KFold, cross_val_score
from sklearn.metrics import confusion_matrix, accuracy_score
import sys
import random


train_path = 'cifar-100-python/train'
test_path = 'cifar-100-python/test'
train = unpickle(train_path)
test = unpickle(test_path)

print(train['data'].shape)

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

if __name__ == '__main__':

    clf = tree.DecisionTreeClassifier()
    clf.fit(train_sub_data, train_sub_label)

    scores = clf.score(test_sub_data, test_sub_label)
    print("Accuracy: %0.2f" % scores)
