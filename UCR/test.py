import numpy as np
from sklearn.utils.extmath import row_norms, safe_sparse_dot
from sklearn.preprocessing import normalize, scale

a = [[1, 2, 3, 4],
     [1, 2, 3, 4],
     [1, 2, 3, 4]]

b = [[2, 3, 4, 5],
     [2, 3, 4, 5],
     [2, 3, 4, 5]]

# 这两个函数的输出是相同的，因为实际row_norms就是由einsum实现的，每行元素
out = np.einsum("ij, ij -> i", a, a)
# print(out)

aa = row_norms(a, squared=True)
print(aa)

bb = row_norms(b, squared=True)
print(bb)

a = np.array(a)
b = np.array(b)
distance = -2 * safe_sparse_dot(a, b.T, dense_output=True)
print(distance)
distance = distance + aa
distance = distance + bb
print(distance)

print(np.sum(a, axis=0))

K = [[9.,   2.,  -2.],
     [2.,  14., -13.],
     [-2., -13.,  21.]]

K_fit_rows = np.sum(K, axis=0) / 3
print(K_fit_rows)
K_pred_cols = (np.sum(K, axis=1) / K_fit_rows.shape[0])[:, np.newaxis]
print(K_pred_cols)
K_fit_all = K_fit_rows.sum() / 3

K -= K_fit_rows
K -= K_pred_cols
K += K_fit_all
print(K)

v = [[1, 2, 3, 4, 5],
     [1, 2, 3, 4, 5]]
v_prime = scale(v, axis=1)
print(v_prime)

print(2**20)
