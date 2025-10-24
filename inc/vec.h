#ifndef __GRAPH_VEC_H__
#define __GRAPH_VEC_H__

#include <mem.h>
#include <string.h>
#include <assert.h>

template<typename T>
struct B {
	T*P;
	S L, i;
	inl B(S L):P{mk<T>(L)},L{L},i{0}{}
	inl X resize(S L)->V{this->L=L,P=remk<T>(P,L);}
	inl X re(){if(i+1>L){resize(i*2);}}
	X insert(T x,S D)->V{re();i++;MMV(P+D+1,P+D,(i-D-1)*Z(T));P[D]=x;}
	inl X del(S D)->V{MMV(P+D,P+D+1,(i-D-1)*Z(T));i--;}
	inl X del(){free(P);}
	inl X add(T x)->V{insert(x,i);}
	inl X len()->S{return i;}
	T&operator[](S D){assert(D<i); return P[D];}

};

#endif
