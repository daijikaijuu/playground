#!/usr/bin/env python3

def bubble_sort(items: list[int]) -> list[int]:
    for i in range(len(items) - 1):
        for j in range(len(items) - i - 1):
            if items[j] > items[j+1]:
                items[j], items[j+1] = items[j+1], items[j]
    return items


print(bubble_sort([1, 3 , 2, 0, 9, 7, 8]))
