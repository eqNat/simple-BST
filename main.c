#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#include "bst.h"

int main()
{

	printf("---------------smallTree---------------\n");

	{	// insert 50 ints, print data then delete
		struct BST_Node* smallTree = NULL;
		
		for (int i = 0; i < 50; i++)
			insert(&smallTree, rand() % 1000);

		printf("Fifty ints:\n");
		ascendingPrint(smallTree);

		delete_tree(smallTree);
		smallTree = NULL;
	}

	printf("---------------largeTree---------------\n");

	{	// insert 10 million ints, print elapsed time then delete
		struct BST_Node* largeTree = NULL;
		
		printf("Inserting 10 million ints...\n");

		clock_t t = clock();

		for (int i = 0; i < 10000000; i++)
			insert(&largeTree, rand());

		printf("Insertion took %.4f seconds.\n", ((double)clock()-t)/CLOCKS_PER_SEC);

		delete_tree(largeTree);
		largeTree = NULL;
	}
}
