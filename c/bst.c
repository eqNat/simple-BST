#include <stdio.h>
#include <stdlib.h>

#include "bst.h"

struct BST_Node {
    int data;
    struct BST_Node* child[2];
};

void insert(struct BST_Node** node, int data)
{
    while (*node)
        node = &(*node)->child[((*node)->data > data)];
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
        node = &(*node)->child[((*node)->data > data)];
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
        node = node->child[(node->data > key)];
    }
    return 0;
}

void ascendingPrint(struct BST_Node* iter)
{
    if (iter) {
        ascendingPrint(iter->child[1]);
        printf("%d\n", iter->data);
        ascendingPrint(iter->child[0]);
    }
}

void delete_tree(struct BST_Node* iter)
{
    if (iter) {
        delete_tree(iter->child[0]);
        delete_tree(iter->child[1]);
        free(iter);
    }
}
