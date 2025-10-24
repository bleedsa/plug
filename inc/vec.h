#ifndef __GRAPH_VEC_H__
#define __GRAPH_VEC_H__

#include <mem.h>
#include <assert.h>

template<typename X>
struct Vec {
	X*ptr;
	S L, i;

	Vec(S L):ptr{mk<X>(L)},L{L},i{0}{}
	~Vec(){free(ptr);}

	auto resize(S L)->V{this->L=L,ptr=remk<X>(ptr,L);}

	auto insert(X x,S idx)->V{
		if(i+1>L){resize(i*2);}
		i++; memmove(ptr+idx+1,ptr+idx,(i-idx-1)*Z(X)); ptr[idx]=x;
	}

	auto del(S idx)->V{memmove(ptr+idx,ptr+idx+1,(i-idx-1)*Z(X)); i--;}
	auto add(X x)->V{insert(x,i);}
	auto len()->S{return i;}
	X&operator[](S idx){assert(idx<i); return ptr[idx];}

};

#endif
