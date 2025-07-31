#include "luhn.h"
#include<cctype>
#include<string_view>
namespace luhn {
    bool valid(std::string_view str){
        
        int size = str.size()-1;
        if(size == 0 || str==" 0")
            return false;
        bool isSecond = false;
        int sum = 0;
        while(size >= 0){
            int c = (str[size] - '0');
            if((c >= 0 && c < 10) || (c == -16)){
                int val = c;
                if(c==-16){
                    size--;
                    continue;
                }
                if(!isSecond){
                    sum += val;
                    isSecond = true;

                }
                else {
                    val *= 2;
                    if(val > 9)   
                        val -= 9;
                    sum += val;
                    isSecond = false;
                }
                
            } else {
                return false;
                break;
            }
            size--;
        }
        if(sum%10==0)
            return true;
        return false;
    }
}  // namespace luhn
