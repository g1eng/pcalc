typedef unsigned int uint;

int cpri(uint pr_test, uint max) {
    for(uint i=2;i<=max;i++)
	if (pr_test % i == 0) 
	    return 0;
    return 1;
}
