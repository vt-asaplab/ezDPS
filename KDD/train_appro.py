# Import libraries
import os
import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
import seaborn as sns
from time import time
from sklearn.decomposition import PCA
from sklearn.svm import SVC, LinearSVC
from sklearn import pipeline
from sklearn.kernel_approximation import RBFSampler, Nystroem
from extract_feature import wavelets_f
# %config InlineBackend.figure_format = 'retina'
from sklearn.model_selection import train_test_split
from sklearn.metrics import accuracy_score
from load_data import load_data
from sklearn.preprocessing import scale

# Reading features list
num_class = int(sys.argv[1])
X, y = load_data(num_class)

# Split dataset between training and testing set (80/20 split)
X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.20, random_state=42)
X_train = np.array(X_train)
X_test = np.array(X_test)
y_train = np.array(y_train)
y_test = np.array(y_test)
print(X_train.shape, X_test.shape)
print(y_train.shape, y_test.shape)

###############################################################################################
# dwt
x_train_dwt = wavelets_f(X_train, threshold=0.001)
x_test_dwt = wavelets_f(X_test, threshold=0.001)

###############################################################################################
# nl
x_train_nl = scale(x_train_dwt, axis=1)
# np.savetxt("./data/nl/nl_x.txt", test_data[0], fmt="%f")
x_test_nl = scale(x_test_dwt, axis=1)
# np.savetxt("./data/nl/nl_y.txt", test_data[0], fmt="%f")

###############################################################################################
# PCA

n_components = 0.98
pca = PCA(n_components=n_components, whiten=False).fit(x_train_nl)
X_train = pca.transform(x_train_nl)
X_test = pca.transform(x_test_nl)

###############################################################################################
# Support Vector Classifier

rbf_svm = SVC(kernel='rbf', gamma=0.001)
poly_svm = SVC(kernel='poly', degree=2, gamma=1)

rbf_svm.fit(X_train, y_train)
poly_svm.fit(X_train, y_train)

score_rbf = rbf_svm.score(X_test, y_test)
score_poly = poly_svm.score(X_test, y_test)
result = poly_svm.predict(X_test)

print("rbf acc: ", score_rbf)
print("poly acc: ", score_poly)
# print(poly_svm.n_support_)
print("poly_sv: ", poly_svm.support_vectors_.shape)
# print(poly_svm.dual_coef_.shape)
print("rbf_sv: ", rbf_svm.support_vectors_.shape)
# kernel_svm = SVC(gamma=1, kernel='poly', degree=3)
# linear_svm = LinearSVC()
#
# # create pipeline from kernel approximation
# # and linear svm
# feature_map_fourier = RBFSampler(gamma=0.001, random_state=1)
# feature_map_nystroem = Nystroem(gamma=0.001, random_state=1)
# fourier_approx_svm = pipeline.Pipeline(
#     [("feature_map", feature_map_fourier), ("svm", LinearSVC())]
# )
#
# nystroem_approx_svm = pipeline.Pipeline(
#     [("feature_map", feature_map_nystroem), ("svm", LinearSVC())]
# )
#
# # fit and predict using linear and kernel svm:
#
# kernel_svm_time = time()
# kernel_svm.fit(X_train, y_train)
# kernel_svm_score = kernel_svm.score(X_test, y_test)
# kernel_svm_time = time() - kernel_svm_time
#
# linear_svm_time = time()
# linear_svm.fit(X_train, y_train)
# linear_svm_score = linear_svm.score(X_test, y_test)
# linear_svm_time = time() - linear_svm_time
#
# sample_sizes = 20 * np.arange(1, 50)
# fourier_scores = []
# nystroem_scores = []
# fourier_times = []
# nystroem_times = []
#
# for D in sample_sizes:
#     fourier_approx_svm.set_params(feature_map__n_components=D)
#     nystroem_approx_svm.set_params(feature_map__n_components=D)
#     start = time()
#     nystroem_approx_svm.fit(X_train, y_train)
#     nystroem_times.append(time() - start)
#
#     start = time()
#     fourier_approx_svm.fit(X_train, y_train)
#     fourier_times.append(time() - start)
#
#     fourier_score = fourier_approx_svm.score(X_test, y_test)
#     nystroem_score = nystroem_approx_svm.score(X_test, y_test)
#     nystroem_scores.append(nystroem_score)
#     fourier_scores.append(fourier_score)
#
#
# # #############################################################################
#
# # plot the results:
# plt.figure(figsize=(16, 4))
# accuracy = plt.subplot(121)
# # second y axis for timings
# timescale = plt.subplot(122)
#
# accuracy.plot(sample_sizes, nystroem_scores, label="Nystroem approx. kernel")
# timescale.plot(sample_sizes, nystroem_times, "--", label="Nystroem approx. kernel")
#
# accuracy.plot(sample_sizes, fourier_scores, label="Fourier approx. kernel")
# timescale.plot(sample_sizes, fourier_times, "--", label="Fourier approx. kernel")
#
# # horizontal lines for exact rbf and linear kernels:
# accuracy.plot(
#     [sample_sizes[0], sample_sizes[-1]],
#     [linear_svm_score, linear_svm_score],
#     label="linear svm",
# )
# timescale.plot(
#     [sample_sizes[0], sample_sizes[-1]],
#     [linear_svm_time, linear_svm_time],
#     "--",
#     label="linear svm",
# )
#
# accuracy.plot(
#     [sample_sizes[0], sample_sizes[-1]],
#     [kernel_svm_score, kernel_svm_score],
#     label="rbf svm",
# )
# timescale.plot(
#     [sample_sizes[0], sample_sizes[-1]],
#     [kernel_svm_time, kernel_svm_time],
#     "--",
#     label="rbf svm",
# )
#
# # vertical line for dataset dimensionality = 64
# accuracy.plot([64, 64], [0.7, 1], label="n_features")
#
# # legends and labels
# accuracy.set_title("Classification accuracy")
# timescale.set_title("Training times")
# accuracy.set_xlim(sample_sizes[0], sample_sizes[-1])
# accuracy.set_xticks(())
# accuracy.set_ylim(np.min(fourier_scores), 1)
# timescale.set_xlabel("Sampling steps = transformed feature dimension")
# accuracy.set_ylabel("Classification accuracy")
# timescale.set_ylabel("Training time in seconds")
# accuracy.legend(loc="best")
# timescale.legend(loc="best")
# plt.tight_layout()
# plt.show()
