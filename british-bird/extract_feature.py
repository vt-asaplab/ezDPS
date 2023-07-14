import numpy as np
import pywt

# 定义一些DB4算法中的系数
h3 = -0.1294095226
h2 = 0.2241438680
h1 = 0.8365163037
h0 = 0.4829629131

g3 = -0.4829629131
g2 = 0.8365163037
g1 = -0.2241438680
g0 = -0.1294095226

Ih3 = g0
Ih2 = h0
Ih1 = g2
Ih0 = h2

Ig3 = g1
Ig2 = h1
Ig1 = g3
Ig0 = h3


def simple_f(heartbeats, n):
    featuredata = np.zeros([n, 12])
    h1 = np.zeros([n, 70])
    h2 = np.zeros([n, 100])
    h3 = np.zeros([n, 100])
    for i in range(0, n):
        h1[i, 0:70] = heartbeats[i, 0:70]
        h2[i, 0:100] = heartbeats[i, 70:170]
        h3[i, 0:100] = heartbeats[i, 20:120]

        featuredata[i, 0:12] = [min(h1[i]), max(h1[i]), np.mean(h1[i]), np.median(h1[i]),
                                min(h2[i]), max(h2[i]), np.mean(h2[i]), np.median(h2[i]),
                                min(h3[i]), max(h3[i]), np.mean(h3[i]), np.median(h3[i])]

    return featuredata


def wavelets_f(x_total, threshold=0.2):
    featuredata = np.zeros([len(x_total), len(x_total[0])])
    for i in range(len(x_total)):
        coeffs = my_wave2dec(x_total[i])
        # print('len(coeffs): ', len(coeffs))
        coeffs = my_threshold(coeffs, 0.2)
        # 将信号进行小波重构
        datarec = my_iwave2dec(coeffs)
        featuredata[i] = np.array(datarec)
    return featuredata


def my_threshold(data, thre):
    data_out = []
    for i in range(len(data)):
        if data[i] >= 0:
            abs_data = data[i]
        else:
            abs_data = -data[i]
        temp = data[i] // abs_data
        comp = abs_data - thre
        if comp > 0:
            data_out.append(temp * comp)
        else:
            data_out.append(0)
    return data_out


def my_wave2dec(data):
    n = len(data)
    data_out = np.zeros(len(data))
    if n < 4:
        raise ValueError("The dimension of the data is less than 4")
    else:
        half = n >> 1
        temp = np.zeros(n)
        i = 0
        for j in range(0, n-3, 2):
            temp[i] = data[j] * h0 + data[j+1] * h1 + data[j+2] * h2 + data[j+3] * h3
            temp[i+half] = data[j] * g0 + data[j+1] * g1 + data[j+2] * g2 + data[j+3] * g3
            i = i+1
        temp[i] = data[n - 2] * h0 + data[n - 1] * h1 + data[0] * h2 + data[1] * h3
        temp[i + half] = data[n - 2] * g0 + data[n - 1] * g1 + data[0] * g2 + data[1] * g3
        for i in range(n):
            data_out[i] = temp[i]
        # level = level - 1
    return data_out


def my_iwave2dec(data):
    n = len(data)
    if n < 4:
        raise ValueError("The dimension of the data is less than 4")
    else:
        half = n >> 1
        halfplus = half + 1
        temp = np.zeros(n)
        temp[0] = data[half-1] * Ih0 + data[n-1] * Ih1 + data[0] * Ih2 + data[half] * Ih3
        temp[1] = data[half - 1] * Ig0 + data[n - 1] * Ig1 + data[0] * Ig2 + data[half] * Ig3
        j = 2
        for i in range(0, half-1):
            temp[j] = data[i] * Ih0 + data[i+half] * Ih1 + data[i+1] * Ih2 + data[i+halfplus] * Ih3
            j = j + 1
            temp[j] = data[i] * Ig0 + data[i + half] * Ig1 + data[i + 1] * Ig2 + data[i + halfplus] * Ig3
            j = j + 1
        for i in range(0, n):
            data[i] = temp[i]
    return data


