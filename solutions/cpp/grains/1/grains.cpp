#include "grains.h"

namespace grains {
    unsigned long long square(int n){

        unsigned long long v {1};
        unsigned long long p2{1};
        n--;
        while(n){
            v += p2;
            p2 *= 2;
            n--;
        }
        return v;
    }
unsigned long long total(){
        return (square(64)-1) * 2 + 1;
    }
}  // namespace grains
