class Solution {
    fun buildArray(nums: IntArray): IntArray {
        val size = nums.size
        val result = MutableList<Int>(size) { 0 }

        for (i in 0 until size) {
            result[i] = nums[nums[i]]
        }
        return result.toIntArray()
    }
}