#pragma once

#include <stdint.h>

struct BST_Node;

void insert(struct BST_Node** node, int32_t data);
int insertUnique(struct BST_Node** node, int32_t data);

void traversePrint(struct BST_Node* iter);

