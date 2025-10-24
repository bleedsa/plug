#ifndef __GRAPH_MEM_H__
#define __GRAPH_MEM_H__

#include <stdlib.h>

template<typename X>
inl auto mk(S z)->X*{
	auto P=(X*)malloc(z*Z(X));
	if(P==nullptr)fatal("cannot allocate buffer of size %zu",z);
	return P;
}

template<typename X>
inl auto remk(X*P,S z)->X*{
	P=(X*)realloc(P,z*Z(X));
	if(P==nullptr)fatal("failed to realloc() buffer with size %zu",z);
	return P;
}

#endif
