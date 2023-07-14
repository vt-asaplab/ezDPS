import numpy as np
import matplotlib.pyplot as plt
import pandas as pd
from librosa import core, onset, feature, display
import soundfile as sf
import umap
from IPython.display import Audio
import sklearn
from sklearn.decomposition import PCA
from sklearn.svm import SVC
from extract_feature import wavelets_f
from sklearn.model_selection import GridSearchCV, KFold, cross_val_score
from sklearn.metrics import confusion_matrix, accuracy_score

df = pd.read_csv("./data/birdsong_metadata.csv")
df.head()


def load_audio(file_id):
    data, samplerate = sf.read("./data/songs/songs/xc" + str(file_id) + ".flac")
    s = len(data) / samplerate
    sg = feature.melspectrogram(data, sr=samplerate, hop_length=512)

    # Take mean amplitude M from frame with the highest energy
    centerpoint = np.argmax(sg.mean(axis=0))
    M = sg[:, centerpoint].mean()

    # Filter out all frames with energy less than 5% of M
    mask = sg.mean(axis=0) >= M / 20

    audio_mask = np.zeros(len(data), dtype=bool)
    for i in range(0, len(mask)):
        audio_mask[i * 512:] = mask[i]
    return sg, mask, data, audio_mask, samplerate


df['length'] = np.zeros(len(df))

waves = {}


for file_id in df['file_id']:
    sg, mask, data, audio_mask, sample_rate = load_audio(file_id)
    waves[file_id] = data[audio_mask]
    df.loc[df['file_id'] == file_id,'length'] = len(data[audio_mask])
    # print(len(data[audio_mask])/sample_rate)

df['windows'] = df['length'].apply(lambda x: int(x/6.144000e+03))
n_windows = df.groupby('species')['windows'].sum().min()

windows = {}

for file_id in df['file_id']:
    wave = waves[file_id]
    species = df[df['file_id'] == file_id]['genus'].values[0] + "_" + df[df['file_id'] == file_id]['species'].values[0]
    if species not in windows:
        windows[species] = []
    for i in range(0, int(len(wave)/6.144000e+03)):
        windows[species].append(wave[i:int(i+6.144000e+03)])

# We randomly pick 20 windows for each species

# Save other samples for testing

windows_fixed = {}
windows_fixed_test = {}

for species in windows.keys():
    windows_fixed[species] = []
    windows_fixed_test[species] = []
    ws = windows[species]
    index = np.random.choice(len(ws), n_windows, replace=False)
    for i in range(0, len(ws)):
        if i in index:
            windows_fixed[species].append(ws[i])
        else:
            windows_fixed_test[species].append(ws[i])

# Extract Features from Window
new_dataset = pd.DataFrame()

for species in windows_fixed.keys():
    for i in range(0, n_windows):
        data_point = {'species':species.split('_')[1], 'genus':species.split('_')[0]}
        spec_centroid = feature.spectral_centroid(windows_fixed[species][i])[0]
        chroma = feature.chroma_stft(windows_fixed[species][i], sample_rate)
        for j in range(0, 13):
            data_point['spec_centr_'+str(j)] = spec_centroid[j]
            for k in range(0, 12):
                data_point['chromogram_'+str(k)+"_"+str(j)] = chroma[k,j]
        new_dataset = new_dataset.append(data_point,ignore_index=True)

# Extract Features from Window for test
new_dataset_test = pd.DataFrame()

for species in windows_fixed_test.keys():
    for i in range(0, len(windows_fixed_test[species])):
        data_point = {'species':species.split('_')[1], 'genus':species.split('_')[0]}
        spec_centroid = feature.spectral_centroid(windows_fixed_test[species][i])[0]
        chroma = feature.chroma_stft(windows_fixed_test[species][i], sample_rate)
        for j in range(0,13):
            data_point['spec_centr_'+str(j)] = spec_centroid[j]
            for k in range(0,12):
                data_point['chromogram_'+str(k)+"_"+str(j)] = chroma[k,j]
        new_dataset_test = new_dataset_test.append(data_point, ignore_index=True)

# Prepare dataset to fit a simple model

features = list(new_dataset.columns)
features.remove('species')
# features.remove('license')
features.remove('genus')

X = new_dataset[features].values
y = new_dataset['species'].values

X_test = new_dataset_test[features].values
y_test = new_dataset_test['species'].values

print(y_test.shape)

# Use Naive Bayes as benchmark

# from sklearn import naive_bayes
#
# NB = naive_bayes.GaussianNB()
#
# SSS = sklearn.model_selection.StratifiedShuffleSplit(n_splits=5, test_size=0.2)
#
# accs = []
#
# for train_index, val_index in SSS.split(X, y):
#     X_train, X_val = X[train_index], X[val_index]
#     y_train, y_val = y[train_index], y[val_index]
#
#     NB.fit(X_train, y_train)
#
#     y_pred = NB.predict(X_val)
#
#     accs.append(sklearn.metrics.accuracy_score(y_pred=y_pred, y_true=y_val))
#
# print(accs)
#
# y_pred = NB.predict(X_test)
# sklearn.metrics.accuracy_score(y_pred=y_pred, y_true=y_test)

# The data can be used to predict, let's export the newly created datasets

# new_dataset.to_csv("train.csv")
# new_dataset_test.to_csv("test.csv")

# train_data = wavelets_f(X, threshold=0.02)

# n_components = 0.98
# pca = PCA(n_components=n_components, whiten=False).fit(train_data)


# print("pca components: ", pca.components_.shape)

SSS = sklearn.model_selection.StratifiedShuffleSplit(n_splits=5, test_size=0.2)
best_score = 0
for gamma in [0.001, 0.01, 1, 10, 100]:
    for c in [0.001, 0.01, 1, 10, 100]:
        svm = SVC(gamma=gamma, C=c)
        scores = cross_val_score(svm, X, y, cv=5)
        score = scores.mean()
        if score > best_score:
            best_score = score
            best_parameter = {'gamma': gamma, 'C': c}

svm = SVC(**best_parameter)
svm.fit(X, y)

# X_test = wavelets_f(X_test)
# X_test = pca.transform(X_test)
test_score = svm.score(X_test, y_test)
print('Best score: {:.2f}'.format(best_score))
print('Best parameters:{}'.format(best_parameter))
print('Best score on test set: {:.2f}'.format(test_score))




# n_split = 5
# kfold = KFold(n_splits=n_split)
# X_train_pca = np.array(X_train_pca)
# count = 0
# for train_index, test_index in kfold.split(X_train_pca):
#     clf.fit(X_train_pca[train_index], y[train_index])
#     print(accuracy_score(clf.predict(X_train_pca[test_index]), y[test_index]))
#
#
# scores = cross_val_score(clf, X_train_pca, y, cv=kfold, n_jobs=-1)
# print("Accuracy: %0.2f (+/- %0.2f)" % (scores.mean(), scores.std() * 2))
#
#
# print("t:")
# print(clf.support_vectors_.shape)
