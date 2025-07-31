#include "sieve.h"

#include<vector>
#include<iostream>
namespace sieve {

    std::vector<int> primes(int range){
        
        if(range==1){
            return {};
        }
        std::vector<int> nums(range-1);

        int p{2};
        for(int i{0}; i<range-1;i++){
            nums[i] = p++;
        }
        // std::cout<<"fine"<<std::endl;
        for(int i{0}; i<range-1;i++){
            if(nums[i]==-1)
                continue;
            int v{nums[i]};
            for(int j{i}; j<range-1; j+=v){
                if(j>=range)
                    break;
                if(v==nums[j])
                    continue;
                nums[j] = -1;
            }
        }
        std::vector<int> actual{};
        for(size_t i{}; i<nums.size(); i++){
            if(nums[i]!=-1)
                actual.push_back(nums[i]);
        }
        return actual;
    }
}  // namespace sieve
