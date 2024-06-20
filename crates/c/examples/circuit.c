#include <stdio.h>
#include "qibo_core_c.h"

int main(int argc, char *argv[])
{
	qibo_core_circuit *c = qibo_core_circuit_new(5);
	// qibo_core_circuit_add(c, "H");
	// qibo_core_circuit_add(c, "X");

	printf("%s\n", qibo_core_circuit_draw(c));
	return 0;
}
