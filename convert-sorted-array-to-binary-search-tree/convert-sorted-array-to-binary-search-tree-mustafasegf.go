package convertsortedarraytobinarysearchtree

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func sortedArrayToBST(nums []int) *TreeNode {
  n := len(nums)
  if n == 0 {
    return nil
  }
  
  return &TreeNode{
    Val: nums[n/2],
    Left: sortedArrayToBST(nums[:n/2]),
    Right: sortedArrayToBST(nums[n/2+1:]),
  }
}
