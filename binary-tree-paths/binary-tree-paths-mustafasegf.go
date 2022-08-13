package binarytreepaths

import "fmt"

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func binaryTreePaths(root *TreeNode) []string {
	res := []string{}

	driver(root, fmt.Sprint(root.Val), &res)

	return res
}

func driver(root *TreeNode, str string, res *[]string) {
	if root.Left == nil && root.Right == nil {
		*res = append(*res, str)
		return
	}

	if root.Left != nil {
		driver(root.Left, str+"->"+fmt.Sprint(root.Left.Val), res)
	}

	if root.Right != nil {
		driver(root.Right, str+"->"+fmt.Sprint(root.Right.Val), res)
	}
}
