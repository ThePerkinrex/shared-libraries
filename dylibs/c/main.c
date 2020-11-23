#include <stdio.h>
#include "api.h"

typedef struct DrinckStruct {
	char* name;
	unsigned int age;
} Drinck;

void add(Drinck a) {
	printf("%s, alcoholic (Age %u) HAHA\n", a.name, a.age);
}