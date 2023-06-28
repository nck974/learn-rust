def max_sum_sliding_window(nums, k):
    result = []
    window_sum = sum(nums[:k])  # Calculate sum for the first window
    result.append(window_sum)

    for i in range(k, len(nums)):
        window_sum = window_sum - nums[i - k] + nums[i]  # Update the window sum by subtracting the element that is no longer in the window and adding the next element
        result.append(window_sum)  # Append the current window sum to the result array

    return result[:-k+1]  # Exclude extra elements in the result due to incomplete windows

nums = [4, 2, 1, 7, 8, 1, 2, 8, 1, 0]
k = 3

result = max_sum_sliding_window(nums, k)
print(result)