import load_data
from sklearn.metrics import confusion_matrix, accuracy_score
from sklearn.decomposition import PCA
from sklearn.svm import SVC
from time import time
from extract_feature import wavelets_f
import numpy as np
import floating2fixed
from sklearn.model_selection import GridSearchCV, KFold, cross_val_score

num_class = 42
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

# #######################################################################
# dwt
threshold = 0.001
train_data = wavelets_f(train_data, threshold=threshold)
test_data = wavelets_f(test_data, threshold=threshold)

# #######################################################################
# pca
n_components = 0.97
t0 = time()
pca = PCA(n_components=n_components, whiten=True).fit(train_data)
print("done in %0.3fs" % (time() - t0))

# filename_pac = 'pca_4.sav'
# pickle.dump(pca, open(filename_pac, 'wb'))
# print('pca components saved!')
print("pca components: ", pca.components_.shape)


print("Projecting the input data on the eigenfaces orthonormal basis")
t0 = time()
X_train_pca = pca.transform(train_data)
X_test_pca = pca.transform(test_data)

# print(pca)
# save the data
# np.savetxt('data/pca_X_result.txt', X_test_pca[0], fmt='%f', newline='\n')
# np.savetxt('data/pca_explainedvariance.txt', pca.explained_variance_, fmt='%f', newline='\n')
# print(len(pca_mv))

sqrt_expl_var = np.sqrt(pca.explained_variance_)
X_transformed = X_test_pca * sqrt_expl_var
#
sqrt_expl_var_inv = 1 / sqrt_expl_var
sqrt_expl_var_inv = floating2fixed.float2fix_onedimension(sqrt_expl_var_inv)
# X_transformed = floating2fixed.float2fix_onedimension(X_transformed)
#
# out_field = X_transformed * sqrt_expl_var_inv
# for i in range(len(out_field)):
#     out_field[i] = floating2fixed.todens(floating2fixed.tobits(out_field[i]))
# print(out_field)

X_test_pca = floating2fixed.transform_multiple(X_test_pca)

print("done in %0.3fs" % (time() - t0))

# ########################################################################

# svc
print("Fitting the classifier to the training set")

t0 = time()
clf = SVC(C=1000.0, class_weight=None, gamma=0.001)
# param_grid = {'C': [1e3, 5e3, 1e4, 5e4, 1e5],
#               'gamma': [0.0001, 0.0005, 0.001, 0.005, 0.01, 0.1], }
# clf = GridSearchCV(
#     SVC(kernel='rbf', class_weight=None), param_grid
# )
# n_split = 10
# kfold = KFold(n_splits=n_split)
# X_train_pca = np.array(X_train_pca)
# count = 0
# for train_index, test_index in kfold.split(X_train_pca):
#     # print("TRAIN:", train_index, "TEST:", test_index)
#     clf.fit(X_train_pca[train_index], train_label[train_index])
#     predict_this_round = clf.predict(X_train_pca[test_index])
#     j = 0
#     shu_list = []
#     for i in test_index:
#         print(i)
#         if predict_this_round[j] == train_label[i]:
#             shu_list.append(i)
#         j += 1
#     # np.savetxt('shu_list/cross_'+str(num_class)+'_shu_'+str(count)+'.txt', shu_list, fmt='%i')
#     count += 1
#     print(accuracy_score(clf.predict(X_train_pca[test_index]), train_label[test_index]))
#
# scores = cross_val_score(clf, X_train_pca, train_label, cv=kfold, n_jobs=-1)
# print("Accuracy: %0.2f (+/- %0.2f)" % (scores.mean(), scores.std() * 2))
# #############################################################################

clf.fit(X_train_pca, train_label)
print(accuracy_score(clf.predict(X_test_pca), test_label))

