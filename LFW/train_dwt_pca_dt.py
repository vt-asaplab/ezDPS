from sklearn import tree
import numpy as np
from sklearn.datasets import fetch_lfw_people
from sklearn.utils import shuffle
from sklearn.decomposition import PCA
from extract_feature import wavelets_f
import sys
from sklearn.model_selection import KFold, cross_val_score
from sklearn.metrics import accuracy_score

num_class = int(sys.argv[1])

class_param = [[57, 0, 100, 0.8],
               [42, 1, 100, 0.9],
               [31, 0, 50, 0.8],
               [19, 5, 50, 0.9],
               [11, 15, 150, 0.73]]
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
para_2 = class_param[para_tag][2]
para_3 = class_param[para_tag][3]

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
        print(lab)
        count = count - 1
    cur += 1

for i in range(len(y)):
    if y[i] not in lab:
        X_sub.append(X[i])
        Y_sub.append(y[i])

X_sub = np.array(X_sub)
Y_sub = np.array(Y_sub)

shuf_data = []
shuf_label = []

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
    if count >= para_2:
        break

shuf_data, shuf_label = shuffle(shuf_data, shuf_label, random_state=42)

X_sub = np.array(shuf_data)
Y_sub = np.array(shuf_label)
X_sub = wavelets_f(X_sub, threshold=0.01)
# pca处理数据
n_components = para_3

pca = PCA(n_components=n_components, whiten=True, svd_solver='auto').fit(X_sub)

X_sub = pca.transform(X_sub)

# 获得一个决策树分类器
clf = tree.DecisionTreeClassifier()
# 拟合
shuf_data = X_sub
shuf_label = Y_sub
n_split = 10
kfold = KFold(n_splits=n_split)

shuf_label = np.array(shuf_label)
count = 0
for train_index, test_index in kfold.split(shuf_data):
    clf.fit(shuf_data[train_index], shuf_label[train_index])
    print(accuracy_score(clf.predict(shuf_data[test_index]), shuf_label[test_index]))

scores = cross_val_score(clf, shuf_data, shuf_label, cv=kfold, n_jobs=-1)
print("Accuracy: %0.2f (+/- %0.2f)" % (scores.mean(), scores.std() * 2))
