import sklearn.preprocessing

import load_data
from sklearn.metrics import confusion_matrix, accuracy_score
from sklearn.decomposition import PCA
from sklearn.svm import SVC, LinearSVC
from sklearn import pipeline
from sklearn.kernel_approximation import RBFSampler, Nystroem
from time import time
from extract_feature import wavelets_f
import numpy as np
import matplotlib.pyplot as plt
from sklearn.model_selection import GridSearchCV, KFold, cross_val_score
from sklearn.decomposition import KernelPCA
from scipy.interpolate import RBFInterpolator
from sklearn.metrics import pairwise
from sklearn.preprocessing import normalize, scale



num_class = 4
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

# np.savetxt("./data/dwt/dwt_x.txt", test_data[0], fmt="%f")
# #######################################################################
# dwt
print("input shape:", test_data.shape)

train_data = wavelets_f(train_data, threshold=0.02)
test_data = wavelets_f(test_data, threshold=0.02)

# ######################################################################
# Normalization

train_data = scale(train_data, axis=1)
np.savetxt("./data/nl/nl_x.txt", test_data[0], fmt="%f")
test_data = scale(test_data, axis=1)
np.savetxt("./data/nl/nl_y.txt", test_data[0], fmt="%f")


# #######################################################################
# pca
n_components = 0.99
t0 = time()
# pca = KernelPCA(n_components=int(n_components * train_data.shape[1]), kernel='linear').fit(train_data)
pca = PCA(n_components=n_components, whiten=False).fit(train_data)
print("done in %0.3fs" % (time() - t0))

# filename_pac = 'pca_4.sav'
# pickle.dump(pca, open(filename_pac, 'wb'))
# print('pca components saved!')
# print("pca components: ", pca.components_.shape)
print(train_data.shape)
X_train_pca = pca.transform(train_data)

# np.savetxt("./data/pca/input.txt", test_data[0], fmt="%f")
test_data = pca.transform(test_data)
print(test_data.shape)

# np.savetxt("./data/pca/eigenvalues.txt", pca.eigenvalues_, fmt="%f")
# np.savetxt("./data/pca/eigenvectors.txt", pca.eigenvectors_, fmt="%f")
# np.savetxt("./data/pca/X_fit.txt", pca.X_fit_, fmt="%f")
# np.savetxt("./data/pca/non_zero_index.txt", np.flatnonzero(pca.eigenvalues_), fmt="%f")
# np.savetxt("./data/pca/K_fit_rows.txt", pca._centerer.K_fit_rows_, fmt="%f")
# print(pca._centerer.K_fit_all_)
# np.savetxt("./data/pca/K_fit_all.txt", pca._centerer.K_fit_all_, fmt="%f")


# np.savetxt("./data/pca/output.txt", test_data[0], fmt="%f")


# ########################################################################
# svc

kernel_svm = SVC(gamma=0.001, kernel='rbf', C=1000)
X_train_pca = np.array(X_train_pca)
print("pca shape:", X_train_pca.shape)

# linear_svm = SVC(kernel='linear', gamma=0.001)
# linear_svm = LinearSVC(loss='hinge', multi_class='ovr')
linear_svm = LinearSVC()
# create pipeline from kernel approximation
# and linear svm

feature_map_nystroem = Nystroem(gamma=0.1, random_state=42)

nystroem_approx_svm = pipeline.Pipeline(
    [("feature_map", feature_map_nystroem), ("svm", LinearSVC())]
)

# fit and predict using linear and kernel svm:

kernel_svm_time = time()
kernel_svm.fit(X_train_pca, train_label)
kernel_svm_score = kernel_svm.score(test_data, test_label)
print("kernel svm acc:", kernel_svm_score)
kernel_svm_time = time() - kernel_svm_time
# print("The support vector for rbf kernel: ", kernel_svm.n_support_)

sample_sizes = 30 * np.arange(1, 16, 2)

nystroem_scores = []
nystroem_times = []

for D in sample_sizes:
    nystroem_approx_svm.set_params(feature_map__n_components=D)
    start = time()
    nystroem_approx_svm.fit(X_train_pca, train_label)
    nystroem_times.append(time() - start)
    # nystroem_approx_svm.transform(test_data)

    start = time()
    nystroem_score = nystroem_approx_svm.score(test_data, test_label)
    nystroem_scores.append(nystroem_score)
    print("The support vector for the last model of nystroem: ", nystroem_approx_svm['feature_map'])
    print("Nystroem acc:", nystroem_score)


# #############################################################################

# plot the results:
plt.figure(figsize=(16, 4))
accuracy = plt.subplot(121)
# second y axis for timings
timescale = plt.subplot(122)

accuracy.plot(sample_sizes, nystroem_scores, label="Nystroem approx.kernel")
timescale.plot(sample_sizes, nystroem_times, "--", label="Nystroem approx.kernel")


# horizontal lines for exact rbf and linear kernels:
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

accuracy.plot(
    [sample_sizes[0], sample_sizes[-1]],
    [kernel_svm_score, kernel_svm_score],
    label="rbf svm",
)
timescale.plot(
    [sample_sizes[0], sample_sizes[-1]],
    [kernel_svm_time, kernel_svm_time],
    "--",
    label="rbf svm",
)

# vertical line for dataset dimensionality = 64
accuracy.plot([64, 64], [0.7, 1], label="n_features")

# legends and labels
accuracy.set_title("Classification accuracy")
timescale.set_title("Training times")
accuracy.set_xlim(sample_sizes[0], sample_sizes[-1])
accuracy.set_xticks(())
timescale.set_xlabel("Sampling steps = transformed feature dimension")
accuracy.set_ylabel("Classification accuracy")
timescale.set_ylabel("Training time in seconds")
accuracy.legend(loc="best")
timescale.legend(loc="best")
plt.tight_layout()
plt.show()
