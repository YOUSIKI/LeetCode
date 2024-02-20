/*
 * @lc app=leetcode.cn id=441 lang=rust
 *
 * [441] 排列硬币
 *
 * https://leetcode.cn/problems/arranging-coins/description/
 *
 * algorithms
 * Easy (45.11%)
 * Likes:    290
 * Dislikes: 0
 * Total Accepted:    130.3K
 * Total Submissions: 288.9K
 * Testcase Example:  '5'
 *
 * 你总共有 n 枚硬币，并计划将它们按阶梯状排列。对于一个由 k 行组成的阶梯，其第 i 行必须正好有 i 枚硬币。阶梯的最后一行 可能 是不完整的。
 *
 * 给你一个数字 n ，计算并返回可形成 完整阶梯行 的总行数。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：n = 5
 * 输出：2
 * 解释：因为第三行不完整，所以返回 2 。
 *
 *
 * 示例 2：
 *
 *
 * 输入：n = 8
 * 输出：3
 * 解释：因为第四行不完整，所以返回 3 。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= n <= 2^31 - 1
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        // (1 + x) * x / 2 <= n
        // x^2 + x - 2n <= 0
        // x = floor((-1 + sqrt(1 + 8n)) / 2)
        let x = (-1.0 + (1.0 + 8.0 * n as f64).sqrt()) / 2.0;
        x.floor() as i32
    }
}
// @lc code=end

fn main() {
    assert_eq!(Solution::arrange_coins(5), 2);
    assert_eq!(Solution::arrange_coins(8), 3);
}