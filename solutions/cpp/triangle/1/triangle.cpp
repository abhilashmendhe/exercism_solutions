#include "triangle.h"
#include<iostream>
namespace triangle {
         void check(double a, double b, double c){
        
        if (a<=0 || b<=0 || c<=0)
            throw std::domain_error("error");
        if((a+b)<c || (b+c)<a || (a+c)<b)
            throw std::domain_error("error");

    }
    int kind(double a, double b, double c){
        
        check(a, b, c);
        if(a==b && b==c && c==a)
            return 1;
        else if(a==b || b==c || c==a)
            return 2;
        else 
            return 3;
        
    }

}  // namespace triangle
