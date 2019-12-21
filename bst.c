#include <stdio.h>
#include <stdlib.h>

#include "bst.h"

struct BST_Node {
	struct BST_Node* left;
	struct BST_Node* right;
	int data;
};

void new_Node(struct BST_Node** node, int data)
{
	*node = malloc(sizeof(struct BST_Node));
	(*node)->data = data;
	(*node)->left = NULL;
	(*node)->right = NULL;
}

// allows duplicates to be inserted
void insert(struct BST_Node** node, int data)
{
	while (*node)
		node = ((*node)->data > data) ? &(*node)->left : &(*node)->right;
	new_Node(node, data);
}

// returns 1 if success
// returns 0 if duplicate value caused no insertion
int insertUnique(struct BST_Node** node, int data)
{
	while (*node)
		if ((*node)->data == data)
			return 0;
		else
			node = ((*node)->data > data) ? &(*node)->left : &(*node)->right;

	new_Node(node, data);
	return 1;
}

void ascendingPrint(struct BST_Node* iter)
{
	if (iter) {
		ascendingPrint(iter->left);
		printf("%d\n", iter->data);
		ascendingPrint(iter->right);
	}
}

void delete_tree(struct BST_Node* iter)
{
	if (iter) {
		delete_tree(iter->left);
		delete_tree(iter->right);
		free(iter);
	}
}
