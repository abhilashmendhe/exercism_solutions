#include "atbash_cipher.h"
#include<string>
#include<string_view>
namespace atbash_cipher {
    std::string encode(std::string_view str){
        std::string nstr{""};

        int i{};
        int size = str.size();
        int space{1};
        while(i < size){

            int chval = int(str[i]);
            if(chval <=90 && chval >=65) 
                chval += 32;
            int ival = 25 - (chval % 97) + 97;

            if(chval >= 48 && chval <= 57)
                ival = chval;
            else if(chval>122 || chval <92){
                i++;
                continue;
            }

            if(space==5){
                int nchval = int(str[i+1]);
                if((size-i)<=2 && (nchval>122 || nchval <92)){
                    
                    // if
                        nstr = nstr + char(ival);
                }else
                    nstr = nstr + char(ival) + " ";
                space = 0;
                // nstr = nstr + char(ival) + " ";
                space = 0;
            } else
                nstr+=char(ival);
            i++;
            space++;
        }

        return nstr;
    }
    std::string decode(std::string_view str){
        std::string nstr{""};

        int i{};
        int size = str.size();
        while(i < size){
            int chval = int(str[i]);
            int ival = 25 - (chval % 97) + 97;
            if(chval >= 48 && chval <= 57)
                ival = chval;
            else if(chval==32){
                i++;
                continue;
            }
            nstr+=char(ival);
            i++;
        }
        return nstr;
    }
}  // namespace atbash_cipher
