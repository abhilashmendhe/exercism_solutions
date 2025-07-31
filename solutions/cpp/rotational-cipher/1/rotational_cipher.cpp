#include "rotational_cipher.h"

#include<iostream>
#include<string>
namespace rotational_cipher {
    std::string rotate(std::string s, int num){
        std::string news{};
        for(size_t i=0; i<s.size(); i++){
            int c = s[i];
            if(c>=97 && c<=122){
                int al = c - 97  + num;
                al %= 26;
                al+=97;
                // std::cout<<al<<std::endl;
                news += char(al);
            }
            if(c>=65 && c<=90){
                int al = c - 65  + num;
                al %= 26;
                al+=65;
                news+=char(al);
            }
            if(!isalpha(c)){
                news+=char(c);
            }
        }
        
        return news;
    }
}  // namespace rotational_cipher
