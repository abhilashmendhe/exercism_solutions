#include "largest_series_product.h"

#include<string>
#include<stdexcept>
#include<typeinfo>
namespace largest_series_product {
    int largest_product(const std::string str, unsigned long int n){
        if( n>str.size()){
            throw std::domain_error("err");
        }
        int max = -100000;
        unsigned long int s_size = str.size();
        for(unsigned long int i{}; i<s_size; i++){

            int prod = 1;
            if(i>(s_size-n))
            break;
            for(unsigned long int j{i}; j<n+i; j++){       
                char c = str[j];
                if(c<48 || c>57){
                    throw std::domain_error("err");
                }
                int val = int(c) - 48;
                prod *=val;
            }
            // std::cout<<prod<<std::endl;
            if (prod > max){
                max = prod;
            }
        }
        return max;
    }
} // largest_series_product