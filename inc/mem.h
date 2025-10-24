#ifndef __GRAPH_MEM_H__
#define __GRAPH_MEM_H__

#include <stdlib.h>

template<typename T>
inl X mk(S z)->T*{
	X P=(T*)malloc(z*Z(T));
	if(P==nullptr)fatal("cannot allocate buffer of size %zu",z);
	return P;
}

template<typename T>
inl X remk(T*P,S z)->T*{
	P=(T*)realloc(P,z*Z(T));
	if(P==nullptr)fatal("failed to realloc() buffer with size %zu",z);
	return P;
}

#endif
