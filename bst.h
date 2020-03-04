#pragma once

struct BST_Node;

void insert(struct BST_Node** node, int data);
int insertUnique(struct BST_Node** node, int data);

int search(struct BST_Node* node, int key);
void ascendingPrint(struct BST_Node* iter);
void delete_tree(struct BST_Node* iter);
