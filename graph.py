import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
from collections import Counter
from statistics import mean, variance, median

df1 = pd.DataFrame([['Vector', 1.0000,1.1491,1.1128,1.0064,1.0065,1.0542,0.9953],
                   ['VectorDeque', 1.0000,1.1845,1.1407,0.9964,0.9863,1.0187,0.9664],
                   ['Linked List', 1.0000,1.0022,0.9983,0.0000,0.9962,1.0067],
                   ['Hashmap', 1.0000,0.9152,0.9505,0.0000,0.9194,0.9773],
                   ['BTreemap', 1.0000,1.0412,1.0332,0.0000,1.0262,1.0051],
                   ['Hashset', 1.0000,1.0130,1.0130,1.0057,0.9957,1.0077],
                   ['BTreeset', 1.0000,0.9936,0.9965,1.1622,1.0088,1.0004],
                   ['BinaryHeap', 1.0000,1.0130,1.0183,0.9922,1.0011,0.9901,0.9855],
                   ],
                  columns=['Data Structure', 'SYS', 'Jemalloc', 'Mimalloc', 'MMAP', 'BrugTemplate', 'BrugAuto', 'BrugAuto_Trained'])
# view data
print(df1)

# plot grouped bar chart
df1.plot(x='Data Structure',
         kind='bar',
         stacked=False,
         rot=0,
         ylim=(0.8, 1.2),
         ylabel=('Execuation Time Normalized by SYS'),
         title='Bar Graph with different Rust datasturctus using different allocators')


# d = {
#     'Sys': [0.07, 0.00, 0.00, 0.00, 0.00, 0.73, 0.00, 0.60, 0.00, 0.00, 0.07, 0.40, 1.20, 3.87, 6.33, 2.93, 9.47, 2.73, 16.67, 14.13, 96.67, 0.00, 319.20],
#     'Jemalloc': [0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 3.00, 0.27, 0.73, 0.60, 0.07, 0.27, 0.73, 2.87, 11.60, 14.60, 37.27, 48.67, 118.07, 280.20, 536.53, 490.73, 1312.40
#                  ],
#     'Mimalloc': [0.07, 0.00, 0.00, 0.00, 0.00, 0.87, 3.07, 0.93, 0.00, 0.73, 0.07, 0.27, 2.73, 4.13, 10.73, 14.20, 27.93, 49.40, 127.60, 275.20, 464.73, 422.40, 1222.73
#                  ],
#     'MMAP': [0.07, 0.07, 0.07, 0.00, 0.00, 1.33, 3.13, 0.00, 0.80, 0.80, 0.13, 0.73, 1.00, 2.40, 6.07, 0.80, 0.00, 0.00, 0.00, 0.00, 0.00, 17.33, 0.00
#              ],
#     'BrugTemplate': [0.00, 0.00, 0.00, 0.00, 0.00, 1.47, 0.00, 0.73, 0.20, 0.33, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 10.00, 9.40, 30.73, 67.40, 168.40, 376.00, 687.13

#                      ],
#     'BrugAuto': [0.00, 0.00, 0.00, 0.00, 0.00, 0.20, 4.67, 0.73, 1.20, 1.87, 0.73, 0.20, 1.53, 3.20, 8.47, 7.33, 16.27, 8.13, 36.33, 112.20, 198.27, 34.40, 302.07],
#     'BrugAuto_Trained': [
#         0.00, 0.00, 0.00, 0.00, 0.00, 0.44, 5.17, 0.69, 1.27, 2.10, 0.68, 0.22, 1.25, 3.13, 8.07, 5.65, 11.53, 2.46, 25.92, 110.89, 179.61, 0.00, 169.33
#     ]
# }


# df2 = pd.DataFrame(data=d)
# # view data
# # df2 = df2.T
# print(df2)


# # plot grouped line chart
# df2.plot(
#     logy=True,
#     #     legend=False,
#     kind='line',
#     #     stacked=True,
#     rot=0,
#     xticks=df2.index,
#     ylabel=('Reallocation Time Difference(µs)'),
#     xlabel=('Reallocation (times)'),
#     ylim=0,
#     xlim=0,
#     title='Reallocation time difference compare to best performance when dumping 15 GB data into vector')


# f, (ax1, ax2) = plt.subplots(2, 1, sharex=True)

# ax1.set_title('Total reallocation time comapre to best performance',size=15)

# allocators = ['SYS', 'Jemalloc', 'Mimalloc', 'MMAP', 'BrugTemplate', 'BrugAuto', 'BrugAuto_Trained']
# Difference = [502.68, 2886.22, 2655.42, 62.35, 1379.42, 765.42, 528.41]
# ax1.bar(allocators, Difference)
# ax1.set_ylabel('(µs)',size=12)
# # plt.show()
# percentage = [1.5184, 8.7180, 8.0209, 0.1883, 4.1666, 2.3120, 1.5961]
# ax2.bar(allocators, percentage)
# plt.ylabel('(%)',size=12)
# # plt.subplots(2, 2, sharex='col')
# f.text(0.02, 0.5, 'Reallocation Time Difference', ha='center', va='center', rotation='vertical',size = 15)

# plt.xticks(rotation=90)
# plt.xlabel('Allocators',size=13)

plt.show()
