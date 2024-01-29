#!/usr/bin/env python3


def selection_sort(items: list[int]) -> list[int]:
    for i in range(len(items) - 1):
        minIndex = i

        for j in range(i + 1, len(items)):
            if items[j] < items[minIndex]:
                minIndex = j

        if minIndex != i:
            items[i], items[minIndex] = items[minIndex], items[i]

    return items

print(selection_sort([-9, -100, 9, 0, 100, 4]))
