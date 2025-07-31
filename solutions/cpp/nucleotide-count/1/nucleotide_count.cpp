#include "nucleotide_count.h"

#include<map>
#include<string>
#include<stdexcept>
namespace nucleotide_count {
    std::map<char, int> count(const std::string& str){
        std::map<char, int> nuc_count{{'A', 0}, {'C', 0}, {'G', 0}, {'T', 0} };
      
        for(size_t i{}; i<str.size(); i++){
            if(str[i]=='A')
                nuc_count['A'] += 1;
            else if(str[i]=='G')
                nuc_count['G'] += 1;
            else if(str[i]=='C')
                nuc_count['C'] += 1;
            else if(str[i]=='T')
                nuc_count['T'] += 1;
            else
            {
                throw std::invalid_argument("Invalid syntax.");
            }
        }
    
        return nuc_count;
    
    }
}  // namespace nucleotide_count
