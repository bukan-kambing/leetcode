package validatebinarysearchtree

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func isValidBST(root *TreeNode) bool {
	return driver(root, nil, nil)
}

func driver(root *TreeNode, max, min *int) bool {
	if root == nil {
		return true
	}

	if max != nil && root.Val >= *max {
		return false
	}

	if min != nil && root.Val <= *min {
		return false
	}

	return driver(root.Left, &(root.Val), min) && driver(root.Right, max, &(root.Val))
}
