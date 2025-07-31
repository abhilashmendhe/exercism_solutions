#include "protein_translation.h"
#include<iostream>
#include<vector>
#include<string>
#include<unordered_map>
namespace protein_translation {
    

    std::vector<std::string> proteins(std::string s){
        std::unordered_map<std::string, std::string> pmap;
        pmap["AUG"] = "Methionine";
        pmap["UUU"] = "Phenylalanine";
        pmap["UUC"] = "Phenylalanine";
        pmap["UUA"] = "Leucine";
        pmap["UUG"] = "Leucine";
        pmap["UCU"] = "Serine";
        pmap["UCC"] = "Serine";
        pmap["UCA"] = "Serine";
        pmap["UCG"] = "Serine";
        pmap["UAU"] = "Tyrosine";
        pmap["UAC"] = "Tyrosine";
        pmap["UGU"] = "Cysteine";
        pmap["UGC"] = "Cysteine";
        pmap["UGG"] = "Tryptophan";
        pmap["UAA"] = "STOP";
        pmap["UAG"] = "STOP";
        pmap["UGA"] = "STOP";

        std::vector<std::string> vec{};
        long unsigned int i{};
        while (i < s.size()){
            std::string st = pmap[s.substr(i,  3)];
            if (st.compare("STOP")==0){
                break;
            }
            vec.emplace_back(st);
            i+=3;
        }
        return vec;
    }
}  // namespace protein_translation
