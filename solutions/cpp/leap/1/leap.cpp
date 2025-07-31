#include "leap.h"

namespace leap {
    bool is_leap_year(int n){
        if(n%4==0 && n%100!=0){
            return true;
        } else{
            if (n%400==0)
                return true;
            else
                return false;
            }
        return false;
        }
}  // namespace leap
