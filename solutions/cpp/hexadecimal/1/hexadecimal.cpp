#include "hexadecimal.h"
#include <string_view>
#include<cmath>
namespace hexadecimal {
    int convert(std::string_view s){
        int decimal = 0;
        int size = s.size()-1;
        int i=0;
        while(size >= 0){
            char c = s[i];
            switch(c){
                case 'a':
                    decimal += pow(16, size) * 10;
                    break;
                case 'b':
                    decimal += pow(16, size) * 11;
                    break;
                case 'c':
                    decimal += pow(16, size) * 12;
                    break;
                case 'd':
                    decimal += pow(16, size) * 13;
                    break;
                case 'e':
                    decimal += pow(16, size) * 14;
                    break;
                case 'f':
                    decimal += pow(16, size) * 15;
                    break;
                case '0':
                case '1':
                case '2':
                case '3':
                case '4':
                case '5':
                case '6':
                case '7':
                case '8':
                case '9':
                    decimal += pow(16,size)*int(s[i] - '0');
                    break;
                default:
                    decimal = 0;
                    size = 0;
                    break;
            }
            i++;
            size--;
        }
        return decimal;
    }
}  // namespace hexadecimal
