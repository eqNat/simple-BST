#include <stdio.h>
#include <stdlib.h>

#include "bst.h"

struct BST_Node {
	struct BST_Node* left;
	struct BST_Node* right;
	int data;
};
#define NODE_SIZE sizeof(struct BST_Node)

struct BST_Node* createNode(int data)
{
	struct BST_Node *new_node = malloc(NODE_SIZE);
	new_node->data = data;
	new_node->left = NULL;
	new_node->right = NULL;
	return new_node;
}

// allows duplicates to be inserted
void insert(struct BST_Node** node, int data)
{
	while (*node)
		node = ((*node)->data > data) ? &(*node)->left : &(*node)->right;
	*node = createNode(data);
}

// returns 1 if success
// returns 0 if duplicate value caused no insertion
int insertUnique(struct BST_Node** node, int data)
{
	while (*node) {
		if ((*node)->data == data)
			return 0;
		node = ((*node)->data > data) ? &(*node)->left : &(*node)->right;
	}
	*node = createNode(data);
	return 1;
}

void traversePrint(struct BST_Node* iter)
{
	if (iter) {
		traversePrint(iter->left);
		printf("%d\n", iter->data);
		traversePrint(iter->right);
	}
}
