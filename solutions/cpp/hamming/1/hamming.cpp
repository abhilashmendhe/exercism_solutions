#include "hamming.h"
#include<stdexcept>
#include<string>
namespace hamming {
    int compute(const std::string& f, const std::string& s){

        if (f.size() != s.size()){
            throw std::domain_error("errr");
        }
        int count{};
        for(size_t i{}; i<f.size(); i++){
            if (f[i]!=s[i]){
                count++;
            }
        }
        return count;
    }
}  // namespace hamming
