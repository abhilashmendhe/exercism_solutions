#include "collatz_conjecture.h"

#include<stdexcept>
namespace collatz_conjecture {

    int steps(int n){
        int c{};
        while(n>1){
            
            if(n%2==0){
                n /= 2;
            } else {
                n = (n * 3) + 1;
            }
            c++;
        }
        if(n<1)
            throw std::domain_error("less");
        return c;
    }
}  // namespace collatz_conjecture
