#include <algorithm>
#include <string>
#include "reverse_string.h"
namespace reverse_string 
{
        std::string reverse_string(const char* st){
        
        std::string str{st};
        size_t s = str.size();
        for(size_t i{}; i<s/2; i++){
            char t = str[i];
            str[i] = str[s-i-1];
            str[s-i-1] = t;
            // std::cout<<str[s-i-1]<<std::endl;
        }
        return str;
    }
}