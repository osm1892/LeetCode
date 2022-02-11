class Solution {
    fun buildArray(nums: IntArray): IntArray {
        val result = ArrayList<Int>(nums.size)
        for (i in nums) {
            result.add(nums[i])
        }
        return result.toIntArray()
    }
}