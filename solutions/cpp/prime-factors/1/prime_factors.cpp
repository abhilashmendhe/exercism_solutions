#include "prime_factors.h"
#include<vector>

namespace prime_factors {
    std::vector<int> of(int n){
        std::vector<int> v;
        if(n<2)
            return v;
        int i{2};
        while(i <= n){
            if(n%i==0){
                v.push_back(i);
                n /= i;
            } else 
                i++;
        }
        return v;
    }
}  // namespace prime_factors
