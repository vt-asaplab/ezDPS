from sklearn import tree
import numpy as np
from load_data import unpickle
from extract_feature import wavelets_f
from sklearn.decomposition import PCA
from time import time
from sklearn.model_selection import GridSearchCV, KFold, cross_val_score
from sklearn.metrics import confusion_matrix, accuracy_score
import sys
from sklearn.utils import shuffle

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

shuf_data = []
shuf_label = []
n_split = 5
if num_class in {32, 64, 100}:
    tag = 100
elif num_class in {8, 16}:
    tag = 200
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
        if count >= tag:
            break

shuf_data, shuf_label = shuffle(shuf_data,shuf_label,random_state=42)
train_sub_data = np.array(shuf_data)
train_sub_label = np.array(shuf_label)

if __name__ == '__main__':

    shuf_data = wavelets_f(train_sub_data, threshold=0.01)

    n_components = 0.95
    t0 = time()
    pca = PCA(n_components=n_components, whiten=True).fit(shuf_data)
    print("done in %0.3fs" % (time() - t0))

    print("pca components: ", pca.components_.shape)
    t0 = time()
    shuf_data = pca.transform(shuf_data)

    print("done in %0.3fs" % (time() - t0))
    clf = tree.DecisionTreeClassifier()

    n_split = 10
    kfold = KFold(n_splits=n_split)
    train_sub_sub_data = np.array(shuf_data)
    train_sub_sub_label = np.array(train_sub_label)
    count = 0
    for train_index, test_index in kfold.split(train_sub_sub_data):
        clf.fit(train_sub_sub_data[train_index], train_sub_sub_label[train_index])
        print(accuracy_score(clf.predict(train_sub_sub_data[test_index]), train_sub_sub_label[test_index]))

    scores = cross_val_score(clf, train_sub_sub_data, train_sub_sub_label, cv=kfold, n_jobs=-1)
    print("Accuracy: %0.2f (+/- %0.2f)" % (scores.mean(), scores.std() * 2))
