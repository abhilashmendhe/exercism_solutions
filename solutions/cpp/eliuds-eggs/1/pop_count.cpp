#include "pop_count.h"

namespace chicken_coop {

    int positions_to_quantity(int spots){
    int count{};
    int check{1};
    while(check <= spots && check != 0){
        if(check & spots){
            count++;
        }
        check <<= 1;
    }
    return count;
}

}  // namespace chicken_coop
