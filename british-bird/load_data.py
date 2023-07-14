import pandas as pd
import numpy as np
from sklearn.preprocessing import LabelEncoder
from sklearn.utils import shuffle


def load_data(num_class):
    train_data = pd.read_csv("train.csv")
    features = list(train_data.columns)
    features.remove("genus")
    features.remove("id")
    features.remove("species")
    """A bird is expressed exactly as its genus and species name"""
    train_data["FullName"] = train_data["genus"] + "_" + train_data["species"]
    # test_data["FullName"] = test_data["genus"]+"_"+test_data["species"]
    print(train_data["FullName"].value_counts())

    X_original = train_data[features].values
    le = LabelEncoder()
    Y_original = le.fit_transform(train_data["FullName"].values)

    X = []
    Y = []

    for i in range(len(Y_original)):
        if Y_original[i] <= num_class:
            X.append(X_original[i])
            Y.append(Y_original[i])

    # X, Y = shuffle(X, Y)

    return X, Y
