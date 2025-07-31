#include "rna_transcription.h"

#include<string>
namespace rna_transcription {
    std::string to_rna(const std::string& str){
        std::string temp{};
        for(size_t i{}; i<str.size(); i++){
            if(str[i]=='G')
                temp+="C";
            else if(str[i]=='C')
                temp+="G";
            else if(str[i]=='T')
                temp+="A";
            else if(str[i]=='A')
                temp+="U";
        }
        
        return temp;
    }
    char to_rna(char c){
        if(c=='G')
            return 'C';
        if(c=='C')
            return 'G';
        if(c=='T')
            return 'A';
        if(c=='A')
            return 'U';
        return '1';
    }
}  // namespace rna_transcription
