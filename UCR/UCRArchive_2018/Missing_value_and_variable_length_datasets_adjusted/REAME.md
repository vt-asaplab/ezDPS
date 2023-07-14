
# Missing values and variable lengths data sets

This folder concerns 15 data sets with missing values and variable lengths. We herewith include the equal length version that leads to the result published on the UCR archive website in the interest of reproducible research. 

We makes all time series of equal length with the following practices:
- Missing values are treated with linear interpolation.
- Variable lengths are reconciled by padding low amplitude random noise to the end of each time series to the length of the longest time series.

The need for making time series of equal length arises because:
- Euclidean distance is only defined for equal length time series.
- Some of the well-known lowerbounding techniques for DTW only work if time series are of equal length (even thougth DTW distance is possible between variable lengths time series).

The first four data sets (numbered 1-4) are data sets with missing values. The rest are data sets with variable lengths. Within a data set, the difference between the length of the shortest and longest time series can be dramatic. For example, in case of PLAID, that is 101 vs. 1344. Some of the data sets might contain noisy examples, for which an ideal classifier should have the option to ignore or label them as 'not sure'. 

1. DodgerLoopDay
2. DodgerLoopGame
3. DodgerLoopWeekend
4. MelbournePedestrain
5. AllGestureWiimoteX
6. AllGestureWiimoteY
7. AllGestureWiimoteZ
8. GestureMidAirD1
9. GestureMidAirD2
10. GestureMidAirD3
11. GesturePebbleZ1
12. GesturePebbleZ2
13. PickupGestureWiimoteZ
14. PLAID
15. ShakeGestureWiimoteZ

We thank Germain Forestier and Hassan Ismail Fawaz at University of Haute Alsace for bringing this matter to our attention and other constructive comments.