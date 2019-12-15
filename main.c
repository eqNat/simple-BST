#include <stdio.h>

#include "bst.h"

int main()
{
	// create empty tree
	struct BST_Node* root = NULL;

	// insert elements to binary search tree
	for (int i = 0; i < 50; i++)
		insert(&root, rand() % 1000);

	// print numbers in tree
	traversePrint(root);
}
