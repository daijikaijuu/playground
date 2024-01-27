-- Function to perform Bubble Sort on a table
function bubbleSort(arr)
  local n = #arr
  local swapped

  repeat
    swapped = false
    for i = 1, n - 1 do
      if arr[i] > arr[i + 1] then
        -- Swap elements if they are in the wrong order
        arr[i], arr[i + 1] = arr[i+1], arr[i]
        swapped = true
      end
    end
  until not swapped;
end

local numbers = {5, 2, 9, 1, 5, 6}
print("Original array:", table.concat(numbers, ", "))
bubbleSort(numbers)
print("Sorted array:", table.concat(numbers, ", "))
