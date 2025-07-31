#include "trinary.h"
#include<string>
#include<cmath>
namespace trinary{
    int to_decimal(const std::string &str){
        int sum{};
        int power = str.length()-1;
        for(size_t i=0; i<str.length(); i++){
            if(!isdigit(str[i]))
                break;
            double val = pow(3, power--);
            sum += (str[i] - '0') * static_cast<int>(val);
        }
        return sum;
    }
}