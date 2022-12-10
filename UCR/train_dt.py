import sys
import load_data
from sklearn.metrics import confusion_matrix, accuracy_score
from sklearn import tree
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

clf = tree.DecisionTreeClassifier()
n_split = 5
kfold = KFold(n_splits=n_split)
count = 0
for train_index, test_index in kfold.split(train_data):
    clf.fit(train_data[train_index], train_label[train_index])
    print(accuracy_score(clf.predict(train_data[test_index]), train_label[test_index]))

scores = cross_val_score(clf, train_data, train_label, cv=kfold, n_jobs=-1)
print("Accuracy: %0.2f (+/- %0.2f)" % (scores.mean(), scores.std() * 2))
