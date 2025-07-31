#include "pangram.h"
#include <set>
#include<cctype>
#include<string_view>
namespace pangram {
    bool is_pangram(std::string_view s){

        std::set<char> m;
        for(size_t i{}; i<s.size(); i++){
            int asciinum = int(std::tolower(s[i]));
            char ts = char(asciinum);
            if(asciinum>96 && asciinum<123){
                // std::cout<<asciinum;
                m.insert(ts);
            }
        }
        if(m.size()==26)
            return true;
        return false;
    }
}  // namespace pangram
