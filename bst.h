#pragma once

struct BST_Node;

void insert(struct BST_Node** node, int data);
int insertUnique(struct BST_Node** node, int data);

void traversePrint(struct BST_Node* iter);
void delete_tree(struct BST_Node* iter);
