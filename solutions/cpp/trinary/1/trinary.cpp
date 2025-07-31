#include "trinary.h"
#include<string>
namespace trinary{
    int to_decimal(const std::string &str){
        int num{};
        try
        {
            num = std::stoi(str);
        }
        catch(const std::exception& e)
        {
            return 0;
        }
        
        int power = 0;
        int sum{};
        while(num){
            int remain = num % 10;
            if (power==0){
                power = 1;
                sum += (remain*power);
            } else {
                power *= 3;
                sum += (remain*power);
            }
            num /= 10;
        }
        return sum;
    }
}