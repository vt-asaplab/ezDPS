import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
from sklearn import preprocessing, metrics, model_selection, neural_network, linear_model, ensemble
from sklearn.metrics import accuracy_score
from sklearn.linear_model import LogisticRegression
from sklearn.neighbors import KNeighborsClassifier
from sklearn.svm import SVC
from sklearn.naive_bayes import GaussianNB
from sklearn.tree import DecisionTreeClassifier
from sklearn.ensemble import RandomForestClassifier
from sklearn.decomposition import PCA, KernelPCA
from extract_feature import wavelets_f
test_data = pd.read_csv("test.csv")
train_data = pd.read_csv("train.csv")
features = list(train_data.columns)
features.remove("genus")
features.remove("id")
features.remove("species")
fig, ax = plt.subplots(figsize=(20,10))
train_data["species"].value_counts().plot.bar(ax=ax)
plt.show()
"""bir kuş tam olarak cinsi ve tür adı olarak ifade edilir."""
train_data["FullName"] = train_data["genus"]+"_"+train_data["species"]
test_data["FullName"] = test_data["genus"]+"_"+test_data["species"]
#fig, ax = plt.subplots(figsize=(20,10))
#train_data["FullyName"].value_counts().plot.bar(ax=ax)
#plt.show()
"""tam olarak eşit bir data setine sahibiz"""
#fig, ax = plt.subplots(figsize=(20,10))
#test_data["FullyName"].value_counts().plot.bar(ax=ax)
#plt.show()
X = train_data[features].values
from sklearn.preprocessing import StandardScaler
scaler = StandardScaler()
X_test = scaler.fit_transform(test_data[features])
X_train = scaler.fit_transform(train_data[features])
y_test = test_data["FullName"].values

le = preprocessing.LabelEncoder()
Y_test = le.fit_transform(y_test)
Y_train = le.fit_transform(train_data["FullName"].values)

sss = model_selection.StratifiedShuffleSplit(n_splits=5, test_size=0.2)

# log_accuracy = []
# log_preds = np.zeros((5,len(X_test)))
#
# knn_accuracy = []
# knn_preds = np.zeros((5,len(X_test)))

svc_accuracy = []
svc_preds = np.zeros((5, len(X_test)))

# nb_accuracy = []
# nb_preds = np.zeros((5, len(X_test)))
#
# dec_tree_accuracy = []
# dec_tree_preds = np.zeros((5, len(X_test)))
#
# rf_accuracy = []
# rf_preds = np.zeros((5, len(X_test)))
i = 0
for train_index, val_index in sss.split(X, Y_train):
    X_train, X_val = X[train_index], X[val_index]
    y_train, y_val = Y_train[train_index], Y_train[val_index]

    # #Logistic Regression
    # logr = linear_model.LogisticRegression(solver='lbfgs', multi_class='auto')
    # logr.fit(X_train,y_train)
    # y_pred=logr.predict(X_val)
    # log_accuracy.append(accuracy_score(y_pred,y_val))
    # log_preds[i] = logr.predict(X_test)
    #
    # # KNN classification K parametresi (karekok(eğitim veri sayısı))/2
    #
    # knn = KNeighborsClassifier(n_neighbors=22, metric="minkowski")
    # knn.fit(X_train,y_train)
    # y_pred1 = knn.predict(X_val)
    # knn_accuracy.append(accuracy_score(y_pred1,y_val))
    # knn_preds[i] = knn.predict(X_test)

    # SVM classification

    n_components = 0.999
    X_dwt = wavelets_f(X_train, threshold=0.000001)
    pca = PCA(n_components=n_components, whiten=True).fit(X_dwt)
    print(X_dwt.shape)

    Kpca = KernelPCA(n_components=169, kernel="linear").fit(X_train)

    X_dwt_val = wavelets_f(X_val, threshold=0.000001)
    X_dwt_test = wavelets_f(X_test, threshold=0.000001)
    shuf_data = Kpca.transform(X_train)
    shuf_data_val = Kpca.transform(X_val)
    shuf_data_test = Kpca.transform(X_test)
    svm = SVC(kernel="rbf")
    svm.fit(shuf_data, y_train)
    y_pred2 = svm.predict(shuf_data_val)
    print(accuracy_score(y_pred2, y_val))
    svc_accuracy.append(accuracy_score(y_pred2, y_val))
    svc_preds[i] = svm.predict(shuf_data_test)


    # # NAİVE BAYES
    #
    # gnb = GaussianNB()
    # gnb.fit(X_train, y_train)
    # y_pred3 = gnb.predict(X_val)
    # nb_accuracy.append(accuracy_score(y_pred3,y_val))
    # nb_preds[i] = gnb.predict(X_test)
    #
    # # DECİSİON TREE
    #
    # dtc = DecisionTreeClassifier(criterion="entropy")
    # dtc.fit(X_train, y_train)
    # y_pred5 = dtc.predict(X_val)
    # dec_tree_accuracy.append(accuracy_score(y_pred5,y_val))
    # dec_tree_preds[i] = dtc.predict(X_test)
    #
    # #RANDOM FOREST
    #
    # rfc = RandomForestClassifier(n_estimators=22, criterion="entropy")
    # rfc.fit(X_train,y_train)
    # y_pred6 = rfc.predict(X_val)
    # rf_accuracy.append(accuracy_score(y_pred6, y_val))
    # rf_preds[i] = rfc.predict(X_test)

    i += 1

# print("Accuracy of Logistic Regression on validation set: ", np.mean(log_accuracy))
# print("Accuracy of KNN algorithm on validation set: ", np.mean(knn_accuracy))
print("Accuracy of SVM on validation set: ", np.mean(svc_accuracy))
# print("Accuracy of Naive Bayes on validation set: ", np.mean(nb_accuracy))
# print("Accuracy of Decision Tree on validation set: ", np.mean(dec_tree_accuracy))
# print("Accuracy of Random Forest on validation set: ", np.mean(rf_accuracy))