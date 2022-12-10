import pandas as pd
import numpy as np


def load_data(num_class):
    train = pd.read_csv('./UCRArchive_2018/NonInvasiveFetalECGThorax1/NonInvasiveFetalECGThorax1_TRAIN.tsv', sep='\t')
    train = np.array(train)
    train_data = []
    train_label = []

    for i in range(len(train)):
        if train[i, 0] <= num_class:
            train_data.append(train[i, 1:])
            train_label.append(train[i, 0])
    # train_label = train[:, 0]
    # train_data = train[:, 1:]
    # print(train_data)
    # test = pd.read_csv()
    # print(train)
    test_data = []
    test_label = []
    test = pd.read_csv('./UCRArchive_2018/NonInvasiveFetalECGThorax1/NonInvasiveFetalECGThorax1_TEST.tsv', sep='\t')
    test = np.array(test)
    for i in range(len(test)):
        if test[i, 0] <= num_class:
            test_data.append(test[i, 1:])
            test_label.append(test[i, 0])
    # test_label = test[:, 0]
    # test_data = test[:, 1:]
    return train_data, train_label, test_data, test_label
