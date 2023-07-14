import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
from sklearn import preprocessing, metrics, model_selection, neural_network, linear_model, ensemble
from sklearn.metrics import accuracy_score
from sklearn.linear_model import LogisticRegression
from sklearn.neighbors import KNeighborsClassifier
from sklearn.svm import SVC, LinearSVC
from sklearn.naive_bayes import GaussianNB
from sklearn.tree import DecisionTreeClassifier
from sklearn.ensemble import RandomForestClassifier
from sklearn.decomposition import PCA, KernelPCA
from extract_feature import wavelets_f
from sklearn.preprocessing import StandardScaler
from sklearn.preprocessing import normalize, scale
from sklearn.model_selection import train_test_split
from time import time
from sklearn.kernel_approximation import Nystroem
from sklearn import pipeline
import load_data


# test_data = pd.read_csv("test.csv")
num_class = int(sys.argv[1])
X, Y = load_data.load_data(num_class)
x_train, x_test, y_train, y_test = train_test_split(X, Y, test_size=0.2, random_state=42)

x_train = np.array(x_train)
x_test = np.array(x_test)

print(x_train.shape)
print(x_test.shape)

#####################################
x_train_dwt = wavelets_f(x_train, threshold=0.001)
x_test_dwt = wavelets_f(x_test, threshold=0.001)

#####################################
x_train_nl = scale(x_train_dwt, axis=1)
# np.savetxt("./data/nl/nl_x.txt", test_data[0], fmt="%f")
x_test_nl = scale(x_test_dwt, axis=1)
# np.savetxt("./data/nl/nl_y.txt", test_data[0], fmt="%f")

#####################################

n_components = 155
pca = PCA(n_components=n_components, whiten=False).fit(x_train_nl)
x_train_pca = pca.transform(x_train_nl)
x_test_pca = pca.transform(x_test_nl)
print(x_test_pca.shape)


##########################################
# svm = SVC(kernel="rbf", gamma=0.001, C=1000)
# svm.fit(x_train_pca, y_train)
# svc_preds = svm.score(x_test_pca, y_test)
#
# print(svc_preds)


kernel_svm = SVC(gamma=0.01, kernel='rbf', C=1000)
X_train_pca = np.array(x_train_pca)
print("pca shape:", X_train_pca.shape)

linear_svm = LinearSVC()

feature_map_nystroem = Nystroem(gamma=2, random_state=42)
nystroem_approx_svm = pipeline.Pipeline(
    [("feature_map", feature_map_nystroem), ("svm", LinearSVC())]
)

# fit and predict using linear and kernel svm:
kernel_svm_time = time()
kernel_svm.fit(X_train_pca, y_train)
kernel_svm_score = kernel_svm.score(x_test_pca, y_test)
print("kernel svm acc:", kernel_svm_score)
print("number of support vectors", kernel_svm.n_support_)
kernel_svm_time = time() - kernel_svm_time
# print("The support vector for rbf kernel: ", kernel_svm.n_support_)

sample_sizes = 30 * np.arange(1, 16, 2)

nystroem_scores = []
nystroem_times = []

for D in sample_sizes:
    nystroem_approx_svm.set_params(feature_map__n_components=D)
    start = time()
    nystroem_approx_svm.fit(X_train_pca, y_train)
    nystroem_times.append(time() - start)
    # nystroem_approx_svm.transform(test_data)

    start = time()
    nystroem_score = nystroem_approx_svm.score(x_test_pca, y_test)
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
