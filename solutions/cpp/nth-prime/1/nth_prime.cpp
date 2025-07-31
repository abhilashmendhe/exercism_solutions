#include "nth_prime.h"

#include<stdexcept>
namespace nth_prime {
    int nth(int n){
        if (n==0){
            throw std::domain_error("err");
        }
        int c{};
        int p{2};
        while(c<n){
            if(check_prime(p)){
                c++;
            }
            p++;
        }
        return p - 1;
    }
    bool check_prime(int p){
        for(int i{2}; i<p; i++){
            if(p%i==0){
                return false;
            }
        }
        return true;
    }
}  // namespace nth_prime
