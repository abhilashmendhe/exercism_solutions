
#include "lasagna_master.h"
#include<string>
#include<vector>
namespace lasagna_master {

    int preparationTime(std::vector<std::string> layers, int pep_time) {
        int t_time = pep_time * layers.size();
        return t_time;
    }
    amount quantities(std::vector<std::string> layers) {
        lasagna_master::amount a{0,0.0};
        
        for(auto &v : layers){
            if (v.compare("noodles")==0)
                a.noodles += 50;
            if (v.compare("sauce")==0)
                a.sauce += .2;
        }
        return a;
    }
    void addSecretIngredient(std::vector<std::string> &myList, std::vector<std::string> friendsList) {
        myList.pop_back();
        std::string last = friendsList.back();
        myList.push_back(last);
    }
    std::vector<double> scaleRecipe(std::vector<double> quantities, int num){
        std::vector<double> newv;
        for(auto &q: quantities){
            double val = (q/2) * num;
            newv.push_back(val);
        }
        return newv;
    }
    void addSecretIngredient(std::vector<std::string> &myList, std::string secIng){
        myList.pop_back();
        myList.push_back(secIng);
    }
}  // namespace lasagna_master
