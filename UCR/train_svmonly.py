import load_data
from sklearn.metrics import confusion_matrix, accuracy_score
from sklearn.decomposition import PCA
from sklearn.svm import SVC
from time import time
from extract_feature import wavelets_f
import numpy as np
from sklearn.model_selection import GridSearchCV, KFold, cross_val_score

train_data, train_label, test_data, test_label = load_data.load_data(42)
train_data = np.array(train_data)
train_label = np.array(train_label)
test_data = np.array(test_data)
test_label = np.array(test_label)
print("Data load successfully!")
print("The train data shape is: ")
print(train_data.shape)
print("The test data shape is: ")
print(test_data.shape)
# ########################################################################
# svc - 原始
# print("Fitting the classifier to the training set")
# t0 = time()
# clf = SVC(C=1000.0, class_weight='balanced', gamma=0.2)
# # param_grid = {'C': [1e3, 5e3, 1e4, 5e4, 1e5],
# #               'gamma': [0.0001, 0.0005, 0.001, 0.005, 0.01, 0.1], }
# # clf = GridSearchCV(
# #     SVC(kernel='rbf', class_weight='balanced'), param_grid
# # )
# clf = clf.fit(train_data, train_label)
# # save the model to disk
# # filename = 'pipeline_4.sav'
# # pickle.dump(clf, open(filename, 'wb'))
# # print("svc parameters saved!")
#
# print("done in %0.3fs" % (time() - t0))
# # print("Best estimator found by grid search:")
# # print(clf.best_estimator_)
# print("support vectors ", clf.support_vectors_.shape)
#
# # #############################################################################
# # Quantitative evaluation of the model quality on the test set
#
# print("Predicting on the test set")
# t0 = time()
#
# y_pred = clf.predict(test_data)
# print("done in %0.3fs" % (time() - t0))
# print("Accuracy:", accuracy_score(test_label, y_pred))

clf = SVC(C=1000.0, class_weight='balanced', gamma=0.05)
n_split = 5
kfold = KFold(n_splits=n_split)
count = 0
for train_index, test_index in kfold.split(train_data):
    # print("TRAIN:", train_index, "TEST:", test_index)
    clf.fit(train_data[train_index], train_label[train_index])
    predict_this_round = clf.predict(train_data[test_index])
    j = 0
    shu_list = []
    for i in test_index:
        print(i)
        if predict_this_round[j] == train_label[i]:
            shu_list.append(i)
        j += 1
    # np.savetxt('shu_list/cross_'+str(num_class)+'_shu_'+str(count)+'.txt', shu_list, fmt='%i')
    count += 1
    print(accuracy_score(clf.predict(train_data[test_index]), train_label[test_index]))

scores = cross_val_score(clf, train_data, train_label, cv=kfold, n_jobs=-1)
print("Accuracy: %0.2f (+/- %0.2f)" % (scores.mean(), scores.std() * 2))