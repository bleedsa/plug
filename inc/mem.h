#ifndef __GRAPH_MEM_H__
#define __GRAPH_MEM_H__

#include <stdlib.h>

template<typename T>
inl auto mk(S z)->T*{
	auto P=(T*)malloc(z*Z(T));
	if(P==nullptr)fatal("cannot allocate buffer of size %zu",z);
	return P;
}

template<typename T>
inl auto remk(T*P,S z)->T*{
	P=(T*)realloc(P,z*Z(T));
	if(P==nullptr)fatal("failed to realloc() buffer with size %zu",z);
	return P;
}

#endif
