#include "sum_of_multiples.h"

#include<vector>
#include<unordered_set>
namespace sum_of_multiples {
        int to(std::vector<int> vec, int num){
        std::unordered_set<int> us {};
        int sum{};
        for(int &v: vec){
            int n = num - 1;
            while(n > 0){
                if(n%v==0)
                    us.emplace(n);
                n--;
            }
        }
        for(const auto elem: us){
            sum+=elem;
        }
        return sum;
    }
}  // namespace sum_of_multiples
