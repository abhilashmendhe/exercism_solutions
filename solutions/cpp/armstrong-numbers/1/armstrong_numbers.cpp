#include "armstrong_numbers.h"

namespace armstrong_numbers {
    bool is_armstrong_number(int n){
    
        int t1{n}, t2{n};
        if (n<10){
            return true;
        }
        int c{};
        while(t1>0){
            t1/=10;
            c++;
        }
        int sum{};
        for(int i{}; i<c; i++){
            int r = t2%10;
            int v{1};
            for(int j{}; j<c; j++){
                v *=r;
            }
            sum+=v;
            t2 /= 10;
        }
        if(sum==n)
            return true;
        return false;
    }
}  // namespace armstrong_numbers
