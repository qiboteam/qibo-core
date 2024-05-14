#include <stdio.h>
#include "qibo_core_c.h"

int main(int argc, char *argv[])
{
	qibo_core_circuit *c = qibo_core_circuit_new(5);

	printf("ciao\n");
	return 0;
}
