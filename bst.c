#include <stdio.h>
#include <stdlib.h>

#include "bst.h"

struct BST_Node {
	struct BST_Node* left;
	struct BST_Node* right;
	int data;
};

// allows duplicates to be inserted
void insert(struct BST_Node** node, int data)
{
	while (*node)
		node = ((*node)->data > data) ? &(*node)->left : &(*node)->right;
	*node = calloc(sizeof(struct BST_Node), 1);
	(*node)->data = data;
}

// returns 1 if success
// returns 0 if duplicate value caused failure
int insertUnique(struct BST_Node** node, int data)
{
	while (*node) {
		if ((*node)->data == data)
			return 0;
		node = ((*node)->data > data) ? &(*node)->left : &(*node)->right;
	}
	*node = calloc(sizeof(struct BST_Node), 1);
	(*node)->data = data;
	return 1;
}

// returns 1 if found
// returns 0 if not found
int search(struct BST_Node* node, int key)
{
	while (node) {
		if (node->data == key)
			return 1;
		node = (node->data > key) ? node->left : node->right;
	}
	return 0;
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
