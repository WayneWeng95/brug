import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
from collections import Counter
from statistics import mean, variance, median

# df1 = pd.DataFrame([['Vector', 1.0000,1.1491,1.1128,1.0064,1.0065,1.0542,0.9953],
#                    ['VectorDQ', 1.0000,1.1845,1.1407,0.9964,0.9863,1.0187,0.9664],
#                    ['Linked List', 1.0000,1.0022,0.9983,0.0000,0.9962,1.0067],
#                    ['Hashmap', 1.0000,0.9152,0.9505,0.0000,0.9194,0.9773],
#                    ['BTreemap', 1.0000,1.0412,1.0332,0.0000,1.0262,1.0051],
#                    ['Hashset', 1.0000,1.0130,1.0130,1.0057,0.9957,1.0077],
#                    ['BTreeset', 1.0000,0.9936,0.9965,1.1622,1.0088,1.0004],
#                    ['BinaryHeap', 1.0000,1.0130,1.0183,0.9922,1.0011,0.9901,0.9855],
#                    ],
#                   columns=['Data Structure', 'SYS', 'Jemalloc', 'Mimalloc', 'MMAP', 'BrugTemplate', 'BrugAuto', 'BrugAuto_Trained'])
# # view data
# print(df1)

# colors = ['#d53e4f', '#fc8d59', '#fee08b', '#ffffbf', '#e6f598', '#99d594', '#3288bd']

# # plot grouped bar chart
# df1.plot(x='Data Structure',
#          kind='bar',
#          stacked=False,
#          rot=0,
#          ylim=(0.9, 1.25),
#         #  ylabel=('Execuation Time Normalized by SYS'),
#         #  title='Bar Graph with different Rust datasturctus using different allocators',
#         color = colors,
#          )


# df1 = pd.DataFrame([['Vector', 1.0000,1.1491,1.1128,1.0064],
#                    ['VectorDeque', 1.0000,1.1845,1.1407,0.9964],
#                    ['Linked List', 1.0000,1.0022,0.9983,0.0000],
#                    ['Hashmap', 1.0000,0.9152,0.9505,0.0000],
#                    ['BTreemap', 1.0000,1.0412,1.0332,0.0000],
#                    ['Hashset', 1.0000,1.0130,1.0130,1.0057],
#                    ['BTreeset', 1.0000,0.9936,0.9965,1.1622],
#                    ['BinaryHeap', 1.0000,1.0130,1.0183,0.9922],
#                    ],
#                   columns=['Data Structure', 'SYS', 'Jemalloc', 'Mimalloc', 'MMAP'])
# # view data
# print(df1)

# colors = ['#d53e4f', '#fc8d59', '#fee08b', '#ffffbf', '#e6f598', '#99d594', '#3288bd']

# # plot grouped bar chart
# df1.plot(x='Data Structure',
#          kind='bar',
#          stacked=False,
#          rot=0,
#          ylim=(0.9, 1.25),
#          ylabel=('Execuation Time Normalized by SYS'),
#         #  title='Bar Graph with different Rust datasturctus using different allocators',
#         color = colors,
#          )





# plt.ylabel(ylabel='Execuation Time Normalized by SYS',size = 11.5)
# plt.xlabel(xlabel='Data Structures',size =11.5)

# plt.legend(loc='upper center', bbox_to_anchor=(0.5, 1.05),
#           ncol=3, fancybox=True,shadow = True,ncols=4)

# d = {
#     'Sys': [
#         0.00, 0.00, 0.00, 0.00, 0.00, 39.93, 6.47, 4.00, 4.00, 5.27, 7.20, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00
#     ],
#     'Jemalloc': [
#         0.00, 0.13, 0.20, 0.11, 0.00, 39.20, 0.00, 0.00, 0.00, 0.00, 7.20, 11.13, 19.67, 36.73, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00
#     ],
#     'Mimalloc': [
#         0.57, 0.13, 0.00, 0.11, 0.22, 40.07, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00
#     ],
#     'MMAP': [0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 36.27, 72.00, 131.33, 253.20, 500.27, 990.27, 1987.20, 4015.93, 8305.13, 16773.67
#              ],
#     'Worst':[7.39 , 4.81 ,5.78, 8.95, 13.41, 40.53,9.60,4.33,4.80,6.07,7.27,11.60,21.67,38.00,77.53,145.13,290.47,549.67,1117.87,2267.40,4552.47,8778.53,18086.07]
#     # 'Brug': [0.60, 0.15, 0.20, 0.11, 0.22, 40.67, 6.47, 4.13, 4.20, 5.60, 7.13, 10.87, 18.93, 33.87, 65.93, 130.53, 263.20, 509.67, 1021.00, 2054.60, 4184.33, 8663.80, 17460.80
#     #                  ],
#         # 'BrugTemplate': [0.00, 0.00, 0.00, 0.00, 0.00, 1.47, 0.00, 0.73, 0.20, 0.33, -0.07, -0.27, -0.73, -2.40, -6.07, -0.80, 10.00, 9.40, 30.73, 67.40, 168.40, 376.00, 687.13
#         #         ],
#     # 'BrugTemplate': [0.00, 0.00, 0.00, 0.00, 0.00, 1.47, 0.00, 0.73, 0.20, 0.33, -0.07, -0.27, -0.73, -2.40, -6.07, -0.80, 10.00, 9.40, 30.73, 67.40, 168.40, 376.00, 687.13
#     #                  ],
#     # 'BrugAuto': [0.00, 0.00, 0.00, 0.00, 0.00, 0.20, 4.67, 0.73, 1.20, 1.87, 0.73, 0.20, 1.53, 3.20, 8.47, 7.33, 16.27, 8.13, 36.33, 112.20, 198.27, 34.40, 302.07],
#     # 'BrugAuto_Trained': [
#     #     0.00, 0.00, 0.00, 0.00, 0.00, 0.44, 5.17, 0.69, 1.27, 2.10, 0.68, 0.22, 1.25, 3.13, 8.07, 5.65, 11.53, 2.46, 25.92, 110.89, 179.61, 0.00, 169.33
#     # ]
# }



# df2 = pd.DataFrame(data=d)
# df2.replace(0, np.nan, inplace=True)
# # view data
# # df2 = df2.T
# print(df2)


# # plot grouped line chart
# df2.plot(
#     logy=True,
#     #     legend=False,
#     # kind='line',
#     #     stacked=True,
#     linewidth = 2,
#     rot=0,
#     xticks=df2.index,
#     ylabel=('Reallocation Time (µs)'),
#     xlabel=('Reallocation instances'),
#     ylim=0,
#     xlim=0,
#     # title='Reallocation time difference compare to best performance when dumping 15 GB data into vector'
# )

# g_line = plt.Line2D((0,1),(0,0), color='green')

# plt.legend()


# f, (ax1, ax2) = plt.subplots(2, 1, sharex=True)

# # ax1.set_title('Total reallocation time comapre to best performance',size=15)

# allocators = ['SYS', 'Jemalloc', 'Mimalloc', 'MMAP', 'BrugTemplate', 'BrugAuto', 'BrugAuto_Trained']
# Difference = [502.68, 2886.22, 2655.42, 62.35, 1379.42, 765.42, 528.41]
# ax1.bar(allocators, Difference)
# ax1.set_ylabel('(µs)',size=12)
# # plt.show()
# percentage = [1.5184, 8.7180, 8.0209, 0.1883, 4.1666, 2.3120, 1.5961]
# ax2.bar(allocators, percentage)
# plt.ylabel('(%)',size=12)
# # plt.subplots(2, 2, sharex='col')
# f.text(0.02, 0.6, 'Reallocation Time Difference', ha='center', va='center', rotation='vertical',size = 15)

# plt.xticks(rotation=90,size=10)
# plt.xlabel('Allocators',size=14)


# d_arrow = {
#     # 'Sys': [
#     #     0.00, 0.00, 0.00, 0.00, 0.00, 39.93, 6.47, 4.00, 4.00, 5.27, 7.20, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00
#     # ],
#     # 'Jemalloc': [
#     #     0.63, 0.25, 0.20, 0.16, 0.23, 39.20, 0.00, 0.00, 0.00, 0.00, 7.20, 11.13, 19.67, 36.73, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00
#     # ],
#     # 'Mimalloc': [
#     #     0.57, 0.13, 0.45, 0.11, 0.22, 40.07, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00
#     # ],
#     # 'MMAP': [0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 36.27, 72.00, 131.33, 253.20, 500.27, 990.27, 1987.20, 4015.93, 8305.13, 16773.67
#     #          ],
#     # 'BrugTemplate': [0.00, 0.00, 0.00, 0.00, 0.00, 40.67, 6.47, 4.13, 4.20, 5.60, 7.13, 10.87, 18.93, 33.87, 65.93, 130.53, 263.20, 509.67, 1021.00, 2054.60, 4184.33, 8663.80, 17460.80
#     #                  ],
#     'BrugTemplate': [0.00, 0.00, 0.00, 0.00, 0.00, 1.47, 0.00, 0.73, 0.20, 0.33, -0.07, -0.27, -0.73, -2.40, -6.07, -0.80, 10.00, 9.40, 30.73, 67.40, 168.40, 376.00, 687.13
#                      ],
#     'BrugAuto': [0.00, 0.00, 0.00, 0.00, 0.00, 0.20, 4.67, 0.73, 1.20, 1.87, 0.73, 0.20, 1.53, 3.20, 8.47, 7.33, 16.27, 8.13, 36.33, 112.20, 198.27, 34.40, 302.07],
#     'BrugAuto_Trained': [
#         0.00, 0.00, 0.00, 0.00, 0.00, 0.44, 5.17, 0.69, 1.27, 2.10, 0.68, 0.22, 1.25, 3.13, 8.07, 5.65, 11.53, 2.46, 25.92, 110.89, 179.61, 0.00, 169.33
#     ]
# }


# df3 = pd.DataFrame(data=d_arrow)
# df3.replace(0, np.nan, inplace=True)
# # view data
# # df2 = df2.T
# print(df3)


# # plot grouped line chart
# df3.plot(
#     # logy=True,
#     #     legend=False,
#     # kind='line',
#     #     stacked=True,
#     rot=0,
#     xticks=df2.index,
#     ylabel=('Reallocation Time (µs)'),
#     xlabel=('Reallocation instances'),
#     ylim=[0,200],
#     xlim=[0,20],
#     # title='Reallocation time difference compare to best performance when dumping 15 GB data into vector'
# )


# f, (ax1, ax2) = plt.subplots(2, 1, sharex=True)

# # ax1.set_title('Total reallocation time comapre to best performance',size=15)

# allocators = ['SYS', 'Jemalloc', 'Mimalloc', 'MMAP', 'BrugTemplate', 'BrugAuto', 'BrugAuto_Trained']
# Difference = [502.68, 2886.22, 2655.42, 62.35, 1379.42, 765.42, 528.41]
# ax1.bar(allocators, Difference)
# ax1.set_ylabel('(µs)',size=12)
# # plt.show()
# percentage = [1.5184, 8.7180, 8.0209, 0.1883, 4.1666, 2.3120, 1.5961]
# ax2.bar(allocators, percentage)
# plt.ylabel('(%)',size=12)
# # plt.subplots(2, 2, sharex='col')
# f.text(0.02, 0.6, 'Reallocation Time Difference', ha='center', va='center', rotation='vertical',size = 15)

# plt.xticks(rotation=90,size=10)
# plt.xlabel('Allocators',size=14)


# Graph For Rust bar graphes
# Data for the x-axis
x = ['Ptmalloc2 (SYS)', 'Jemalloc', 'Mimalloc',
     'MMAP', 'BrugTemplate', 'BrugAutoOpt']


# # Rust standard vector
# # # Data for the y-axis (values for each group)
# # y1 = [28.64, 32.14, 32.80, np.nan,
# #       np.nan, np.nan]  # Group A (5 data points)s
# # y2 = [29.68, 31.95, 32.70, 29.80, 30.42, 29.89]  # Group B (5 data points)


# # Rust standard vector data dump
# # Data for the y-axis (values for each group)
# y1 = [91.44, 267.88, 246.93, np.nan,
#       np.nan, np.nan]  # Group A (5 data points)s
# y2 = [88.05, 261.56, 246.78, 90.20, 90.16, 105.96]  # Group B (5 data points)


# # # Rust Arrow Mutable buffer integer
# # # # Data for the y-axis (values for each group)
# # y1 = [19.96, 18.25, 17.55, np.nan,
# #       np.nan, np.nan]  # Group A (5 data points)s
# # y2 = [16.97, 17.89, 17.11, 14.99, 15.03, 15.21]  # Group B (5 data points)


# # # Rust Arrow Mutable buffer data dump
# # # # Data for the y-axis (values for each group)
# # y1 = [238.40, 261.52, 247.38, np.nan,
# #       np.nan, np.nan]  # Group A (5 data points)s
# # y2 = [240.90, 258.83, 244.17, 87.76, 88.17, 100.33]  # Group B (5 data points)


# Rust Arrow Datafusion
# Data for the y-axis (values for each group)
y1 = [55.81,56.15,54.16, np.nan, 
      np.nan, np.nan]  # Group A (5 data points)s
y2 = [50.07,50.69,51.13,142.70,50.35,47.72]  # Group B (5 data points)


# Set the width of the bars
bar_width = 0.35

# Set the positions of the bars on the x-axis
pos1 = np.arange(len(y1))
pos2 = [x + bar_width for x in np.arange(len(y2))]

# Create a figure and axis
fig, ax = plt.subplots(figsize=(6,4))

# Plot the bars for Group A
ax.bar(pos1, y1, width=bar_width, label='Global allocator',color = 'lightcoral')

# Plot the bars for Group B
ax.bar(pos2, y2, width=bar_width, label='Brug Modes',color = 'cornflowerblue')

# Customize the graph
ax.set_xlabel('Allocators', size=13)
ax.set_ylabel('Execution Time (μs)', size=13)
# ax.set_title('Bar Plot with Two Groups')

# Set the tick positions and labels
ticks_pos = np.arange(len(x)) + bar_width / 2
ax.set_xticks(ticks_pos)
ax.set_xticklabels(x)
ax.set_ylim([0,75])
plt.xticks(rotation=30)

for i, v in enumerate(y1):
    ax.text(i, v, str(v), ha='center', va='bottom',size = 6)    #size 6/7
for i, v in enumerate(y2):
    ax.text(i + bar_width, v, str(v), ha='center', va='bottom',size = 6) #size 6/7

# plt.text(3.15,115,"646.08",size=6.5)
plt.text(3.15,65,"142.70",size=6.5)

# Add a legend
# plt.legend(loc='upper left', bbox_to_anchor=(0.5, 1.05),
#            ncol=3, fancybox=True, shadow=True)
plt.legend(loc='upper right')

# Adjust the spacing between subplots
plt.tight_layout()

# Display the graph


plt.show()

