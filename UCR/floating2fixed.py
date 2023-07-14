import xlrd
import xlwt
import pandas as pd
import numpy as np

# Specify total size of your fixed-point and size of fractions part
FP_Size = 32
FR_Size = 20
exp_2 = []
temp = float(2 ** 11)
exp_2.append(temp)
for i in range(31):
    temp = temp / 2.0
    exp_2.append(temp)

# Input to be converted
# inp = 1
"""
对二维数据进行fixpoint处理
"""
# from numpy import loadtxt
# input_data = loadtxt('values/svc/support_vectors.txt', delimiter=' ')
# print(len(input_data))
# print(len(input_data[0]))
#
# output = np.zeros(shape=(len(input_data), len(input_data[0])))
#
# for row in range(0, len(input_data)):
#     for col in range(0, len(input_data[0])):
#         result = npipp.zeros(shape=(1, 32))
#         flag = 0
#         inp = input_data[row][col]
#         if inp < 0:
#             flag = 1
#             inp = inp * -1
#         for i in range(FP_Size - FR_Size - 1, -FR_Size - 1, -1):
#             if inp >= 2 ** i:
#                 result[0, FR_Size + i] = 1
#                 inp -= 2 ** i
#         temp = 0
#         for i in range(0, 32, 1):
#             temp += (2 ** i) * result[0, i]
#         if flag == 1:
#             output[row][col] = -temp
#         else:
#             output[row][col] = temp
#     np.savetxt('values/svc_fixpoint/support_vectors_fixpoint.txt', output, fmt='%d', newline='\n')


"""
对一维数据进行fixpoint处理
"""
# from numpy import loadtxt
# input_data = loadtxt('poe.txt', delimiter=' ')
# print(len(input_data))
#
# output = []
#
# for inp in input_data:
#     result = np.zeros(shape=(1, 32))
#     flag = 0
#     if inp < 0:
#         flag = 1
#         inp = inp * -1
#     for i in range(FP_Size - FR_Size - 1, -FR_Size - 1, -1):
#         if inp >= 2 ** i:
#             result[0, FR_Size + i] = 1
#             inp -= 2 ** i
#     temp = 0
#     for i in range(0, 32, 1):
#         temp += (2 ** i) * result[0, i]
#     if flag == 1:
#         output.append(-temp)
#     else:
#         output.append(temp)
# np.savetxt('poe_fixpoint.txt', output, fmt='%d', newline='\n')

"""
一个数的小test
"""


def float2fix_single(fl):
    print(fl)
    result = np.zeros(shape=(1, 32))
    flag = 0
    if fl < 0:
        flag = 1
        fl = fl * -1
    for i in range(FP_Size - FR_Size - 1, -FR_Size - 1, -1):
        if fl >= 2 ** i:
            result[0, FR_Size + i] = 1
            fl -= 2 ** i

    temp = 0
    for i in range(0, 32, 1):
        temp += (2 ** i) * result[0, i]
    print("result in uint32:")
    if flag == 1:
        print(-int(temp))
    else:
        print(int(temp))

    print("absolute result in binary:")
    print(np.flip(result))
    return np.flip(result)


def fix2float_single(fi):
    print(fi)
    print(len(fi[0]))
    result = 0
    for i in range(len(fi[0])):
        result = result + fi[0][i] * exp_2[i]
    return result

def tobits(re):
    result = np.zeros(shape=(1, 32))
    for i in range(0, 32):
        if re == 0:
            break
        result[0, i] = re % 2
        re = (re - result[0, i]) / 2
    return result

def todens(bi):
    result = 0
    for i in range(0, 32):
        print(exp_2[i])
        result = result + bi[0][i] * exp_2[31-i]
    return result



def transform_multiple(fl_arr):
    m = len(fl_arr)
    n = len(fl_arr[0])

    fi_arr = np.zeros(shape=(m, n))

    for i in range(0, m):
        for j in range(0, n):
            fl = fl_arr[i][j]
            result = np.zeros(shape=(1, 32))
            flag = 0
            if fl < 0:
                flag = 1
                fl = fl * -1
            for k in range(FP_Size - FR_Size - 1, -FR_Size - 1, -1):
                if fl >= 2 ** k:
                    result[0, FR_Size + k] = 1
                    fl -= 2 ** k
            result = np.flip(result)

            fi = 0
            for l in range(len(result[0])):
                fi = fi + result[0][l] * exp_2[l]
            if flag == 1:
                fi = -fi

            fi_arr[i][j] = fi
    return fi_arr


def float2fix_onedimension(input_data):
    print(len(input_data))

    output = []
    for inp in input_data:
        result = np.zeros(shape=(1, 32))
        flag = 0
        if inp < 0:
            flag = 1
            inp = inp * -1
        for i in range(FP_Size - FR_Size - 1, -FR_Size - 1, -1):
            if inp >= 2 ** i:
                result[0, FR_Size + i] = 1
                inp -= 2 ** i
        temp = 0
        for i in range(0, 32, 1):
            temp += (2 ** i) * result[0, i]
        if flag == 1:
            output.append(-temp)
        else:
            output.append(temp)
    return output


if __name__ == '__main__':
    fl = -0.126
    # result = float2fix_single(fl)
    result = 1553562
    fi = tobits(result)
    print(fi)
    print(todens(fi))
    input = [1, 2, 3]
    print(float2fix_onedimension(input))
    # fl_arr = np.loadtxt('svc_parameter/8_0testdata.txt')
    # fi_arr = transform_multiple(fl_arr)
    # print(fi_arr)
    # np.savetxt('svc_parameter_fr2fi/8_0testdata.txt', fi_arr, fmt='%f', delimiter='\n')
